use js_sys::{wasm_bindgen::JsCast, Promise};
use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{DomException, Event, IdbCursorWithValue, IdbDatabase, IdbKeyRange, IdbObjectStoreParameters, IdbRequest};
use rs_fsrs::Card;
use chrono::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::utils;

pub struct IdxDb {
    db: Rc<IdbDatabase>,
}

impl IdxDb {
    pub async fn new(version: u32) -> Result<Self, JsValue> {
        let request = web_sys::window()
            .ok_or("valid window")?
            .indexed_db()?
            .ok_or("no indexedDB support")?
            .open_with_u32("idn", version)?;

        let has_upgrade = Rc::new(RefCell::new(false));

        let promise = Promise::new(&mut |resolve, reject| {
            let cb_success = Closure::<dyn Fn(_)>::new(move |evt: Event| {
                let db = evt
                    .target()
                    .expect("IdxDb target")
                    .dyn_ref::<IdbRequest>()
                    .expect("IdxDb request")
                    .result()
                    .expect("IdxDb request result")
                    .dyn_into::<IdbDatabase>()
                    .expect("IdxDb database");

                resolve.call1(&JsValue::NULL, &db).ok();
            });
            request.set_onsuccess(Some(cb_success.as_ref().unchecked_ref()));
            cb_success.forget();

            let cb_error = Closure::<dyn Fn(_)>::new(move |evt: Event| {
                reject.call1(&JsValue::NULL, &evt).ok();
            });
            request.set_onerror(Some(cb_error.as_ref().unchecked_ref()));
            cb_error.forget();

            let has_upgrade = has_upgrade.clone();
            let cb_upgrade = Closure::<dyn FnMut(_)>::new(move |evt: Event| {
                let request = evt
                    .target()
                    .and_then(|x| x.dyn_into::<IdbRequest>().ok())
                    .expect("IdbRequest");
                let db = request.result().ok()
                    .and_then(|x| x.dyn_into::<IdbDatabase>().ok())
                    .expect("IdbDatabase");

                if let Err(e) = Self::init(&db) && e.name() != "ConstraintError" {
                    crate::console_log!("initialization error: {e:?}");
                } else {
                    has_upgrade.replace(true);
                }
            });
            request.set_onupgradeneeded(Some(cb_upgrade.as_ref().unchecked_ref()));
            cb_upgrade.forget();
        });

        let db = JsFuture::from(promise)
            .await
            .and_then(|x| x.dyn_into::<IdbDatabase>())?;

        if *has_upgrade.borrow() {
            Self::upgrade(&db).await?
        }

        Ok(IdxDb {
            db: Rc::new(db)
        })
    }

    fn init(db: &IdbDatabase) -> Result<(), DomException> {
        let parameters = IdbObjectStoreParameters::new();
        parameters.set_key_path(&"id".into());
        let store = db
            .create_object_store_with_optional_parameters("vocabs", &parameters)?;
        store.create_index_with_str("vocab", "vocab")?;
        store.create_index_with_str_sequence("idx_lesson", &js_sys::Array::of2(&"seen".into(), &"lesson".into()).into())?;

        let parameters = IdbObjectStoreParameters::new();
        parameters.set_key_path(&"id".into());
        let store = db.create_object_store_with_optional_parameters("cards", &parameters)?;
        store.create_index_with_str("due", "due")?;

        Ok(())
    }

    async fn upgrade(db: &IdbDatabase) -> Result<(), JsValue> {
        let content = JsFuture::from(
            JsFuture::from(web_sys::window()
                .ok_or("failed to get window")?
                .fetch_with_str("/idn/data/vocabs.txt"))
                .await?
                .dyn_ref::<web_sys::Response>()
                .ok_or("failed to convert to Response")?
                .text()?
            )
            .await?
            .as_string()
            .ok_or("failed to convert to string")?;

        let transaction = db
            .transaction_with_str_and_mode("vocabs", web_sys::IdbTransactionMode::Readwrite)?;

        let cb_error = Closure::<dyn Fn(_)>::new(move |evt: Event| {
            evt.prevent_default();
        });
        transaction.set_onerror(Some(cb_error.as_ref().unchecked_ref()));
        cb_error.forget();

        let vocab_store = transaction
            .object_store("vocabs")?;

        for line in content.lines() {
            let mut fields = line.splitn(7, ',');

            let vocab_obj = js_sys::Object::new();
            let id = fields.next()
                .and_then(|x| x.parse::<u32>().ok())
                .map(|x| x.into())
                .unwrap_or(JsValue::NULL);

            js_sys::Reflect::set(
                &vocab_obj,
                &"id".into(),
                &id)?;

            for attr in ["lesson", "vocab", "meaning", "pos", "detail", "sentences"] {
                let val = fields.next()
                    .map(|x| if attr == "lesson" {
                        x.parse::<f64>().expect("invalid lesson format").into()
                    } else {
                        x.into()
                    })
                    .unwrap_or(JsValue::NULL);
                js_sys::Reflect::set(
                    &vocab_obj,
                    &attr.into(),
                    &val)?;
            }

            js_sys::Reflect::set(
                &vocab_obj,
                &"seen".into(),
                &0.into())?;

            match Self::do_request(&vocab_store.add(&vocab_obj).expect("add")).await {
                Ok(_) => (),
                Err(e) if e.dyn_ref::<DomException>().map(|x| x.name() == "ConstraintError").unwrap_or(false) => {
                    if let Ok(vocab) = Self::do_request(&vocab_store.get(&id)?).await {
                        let seen = js_sys::Reflect::get(&vocab, &"seen".into())?;
                        js_sys::Reflect::set(
                            &vocab_obj,
                            &"seen".into(),
                            &seen)?;
                        Self::do_request(&vocab_store.put(&vocab_obj)?).await?;
                    }
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub async fn get_lesson_vocab(&self) -> Result<Option<Vocab>, JsValue> {
        let range = IdbKeyRange::upper_bound(
            &(js_sys::Array::of2(&0.into(), &js_sys::Number::POSITIVE_INFINITY.into())).into()
        )?;
        let cursor_request = self.db
            .transaction_with_str("vocabs")?
            .object_store("vocabs")?
            .index("idx_lesson")?
            .open_cursor_with_range(&range.into())?;

        if let Ok(cursor) = Self::do_request(&cursor_request)
            .await
            .and_then(|x| x.dyn_into::<IdbCursorWithValue>())
        {
            Ok(Some(Vocab::from(cursor.value()?)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_review_card(&self) -> Result<Option<(Card, Vocab)>, JsValue> {
        let range = IdbKeyRange::upper_bound(&format!("{}", Utc::now().timestamp()).into())?;
        let cursor_request = self.db
            .transaction_with_str("cards")?
            .object_store("cards")?
            .index("due")?
            .open_cursor_with_range(&range.into())?;

        if let Ok(cursor) = Self::do_request(&cursor_request)
            .await
            .and_then(|x| x.dyn_into::<IdbCursorWithValue>())
        {
            let card = utils::object_to_card(cursor.value()?)?;
            let id = js_sys::Reflect::get(&cursor.value()?, &"id".into())?
                .as_f64()
                .ok_or("not a number")? as usize;
            let vocab = self.get_vocab_by_id(id).await?
                .ok_or("invalid vocab id")?;
            Ok(Some((card, vocab)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_stats(&self) -> Result<(Option<DateTime<Utc>>, usize, usize), JsValue> {
        let range = IdbKeyRange::upper_bound(
            &(js_sys::Array::of2(&0.into(), &js_sys::Number::POSITIVE_INFINITY.into())).into()
        )?;
        let request = self.db
            .transaction_with_str("vocabs")?
            .object_store("vocabs")?
            .index("idx_lesson")?
            .count_with_key(&range.into())?;
        let vocab_count = Self::do_request(&request)
            .await?
            .as_f64()
            .ok_or("not a number")? as usize;

        let (time, review_count) = self.get_review_stats().await?;
        Ok((time, review_count, vocab_count))
    }
    
    pub async fn get_review_stats(&self) -> Result<(Option<DateTime<Utc>>, usize), JsValue> {
        let range = IdbKeyRange::upper_bound(&format!("{}", Utc::now().timestamp()).into())?;
        let index = self.db
            .transaction_with_str("cards")?
            .object_store("cards")?
            .index("due")?;

        let request = index
            .count_with_key(&range.clone().into())?;
        let count = Self::do_request(&request)
            .await?
            .as_f64()
            .ok_or("not a number")? as usize;

        if count == 0 {
            let request = index
                .open_cursor()?;

            let cursor = Self::do_request(&request)
                .await?;

            if cursor.is_null() {
                Ok((None, 0))
            } else {
                let obj = cursor
                    .dyn_into::<IdbCursorWithValue>()?
                    .value()?;
                let next_review = utils::object_to_card(obj)?.due;

                Ok((Some(next_review), 0))
            }
        } else {
            Ok((None, count))
        }
    }

    pub async fn get_card_by_id(&self, id: usize) -> Result<Option<Card>, JsValue> {
        let get_request = self.db
            .transaction_with_str("cards")?
            .object_store("cards")?
            .get(&(id as f64).into())?;

        let obj = Self::do_request(&get_request).await?;
        let card = utils::object_to_card(obj)?;

        Ok(Some(card))
    }

    pub async fn get_vocab_by_id(&self, id: usize) -> Result<Option<Vocab>, DomException> {
        let get_request = self.db
            .transaction_with_str("vocabs")?
            .object_store("vocabs")?
            .get(&(id as f64).into())?;

        let obj = Self::do_request(&get_request).await?;
        Ok(Some(Vocab::from(obj)?))
    }

    pub async fn update_card(&self, id: usize, card: Card) -> Result<(), JsValue> {
        let obj = utils::card_to_object(card)?;
        js_sys::Reflect::set(&obj, &"id".into(), &(id as f64).into())?;

        let put_request = self.db
            .transaction_with_str_and_mode("cards", web_sys::IdbTransactionMode::Readwrite)?
            .object_store("cards")?
            .put(&obj.into())?;

        Self::do_request(&put_request).await?;

        Ok(())
    }

    async fn do_request(request: &IdbRequest) -> Result<JsValue, JsValue> {
        let promise = Promise::new(&mut |resolve, reject| {
            let resolve_ = resolve.clone();
            let reject_ = reject.clone();
            let cb_success = Closure::<dyn Fn(_)>::new(move |evt: Event| {
                if let Some(res) = evt.target()
                    .and_then(|x| x.dyn_into::<IdbRequest>().ok())
                    .and_then(|x| x.result().ok())
                {
                    resolve.call1(&JsValue::NULL, &res).ok();
                } else {
                    reject_.call1(&JsValue::NULL, &"invalid query result".into()).ok();
                }
            });
            request.set_onsuccess(Some(cb_success.as_ref().unchecked_ref()));
            cb_success.forget();

            let cb_error = Closure::<dyn Fn(_)>::new(move |evt: Event| {
                let error = evt.target()
                    .and_then(|x| x.dyn_into::<IdbRequest>().ok())
                    .and_then(|x| x.error().ok())
                    .flatten();

                if let Some(e) = error {
                    reject.call1(&JsValue::NULL, &e).ok();
                } else {
                    evt.prevent_default();
                    resolve_.call1(&JsValue::NULL, &JsValue::UNDEFINED).ok();
                }
            });
            request.set_onerror(Some(cb_error.as_ref().unchecked_ref()));
            cb_error.forget();
        });

        JsFuture::from(promise).await
    }

    pub async fn create_card(&self, id: usize) -> Result<(), JsValue> {
        let get_request = self.db
            .transaction_with_str("vocabs")?
            .object_store("vocabs")?
            .get(&(id as f64).into())?;

        let obj = Self::do_request(&get_request).await?;
        js_sys::Reflect::set(&obj, &"seen".into(), &1.into())?;

        let put_request = self.db
            .transaction_with_str_and_mode("vocabs", web_sys::IdbTransactionMode::Readwrite)?
            .object_store("vocabs")?
            .put(&obj)?;

        Self::do_request(&put_request).await?;

        let transaction = self.db
            .transaction_with_str_and_mode("cards", web_sys::IdbTransactionMode::Readwrite)?;
        let card_store = transaction
            .object_store("cards")?;

        let card_obj = utils::card_to_object(rs_fsrs::Card::new())?;

        js_sys::Reflect::set(
            &card_obj,
            &"id".into(),
            &(id as f64).into())?;

        Self::do_request(&card_store.add(&card_obj)?)
            .await?;

        Ok(())
    }
    
}

#[derive(Debug)]
pub struct Vocab {
    pub id: usize,
    pub vocab: String,
    pub meaning: String,
    pub pos: String,
    pub detail: String,
    pub sentences: String,
}

impl Vocab {
    pub fn from(obj: JsValue) -> Result<Self, JsValue> {
        Ok(Self {
            id: js_sys::Reflect::get(&obj, &"id".into())?
                .as_f64()
                .ok_or("not a number")? as usize,
            vocab: js_sys::Reflect::get(&obj, &"vocab".into())?
                .as_string()
                .ok_or("not a string")?,
            meaning: js_sys::Reflect::get(&obj, &"meaning".into())?
                .as_string()
                .ok_or("not a string")?,
            pos: js_sys::Reflect::get(&obj, &"pos".into())?
                .as_string()
                .ok_or("not a string")?,
            detail: js_sys::Reflect::get(&obj, &"detail".into())?
                .as_string()
                .ok_or("not a string")?,
            sentences: js_sys::Reflect::get(&obj, &"sentences".into())?
                .as_string()
                .ok_or("not a string")?,
        })
    }
}
