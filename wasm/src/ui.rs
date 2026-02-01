use chrono::{DateTime, Datelike, Utc};
use rs_fsrs::FSRS;
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, Worker, WorkerOptions, WorkerType};
use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlTimeElement};

use std::rc::Rc;

use crate::db::IdxDb;
use crate::utils;

pub struct Ui {
    document: web_sys::Document,
}

impl Ui {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self {
            document: web_sys::window()
                .ok_or("failed to get window")?
                .document()
                .ok_or("failed to get document")?,
        })
    }

    pub async fn init_review(&self, db: Rc<IdxDb>) -> Result<(), JsValue> {
        let card_ele = self
            .document
            .query_selector("section.vocab")?
            .ok_or("failed to get card element")?;

        let flip_cb = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
            card_ele.set_attribute("data-mode", "back").ok();
        });
        self.document
            .get_element_by_id("flip")
            .ok_or("failed to get card element")?
            .dyn_into::<web_sys::HtmlButtonElement>()?
            .set_onclick(Some(flip_cb.as_ref().unchecked_ref()));
        flip_cb.forget();

        let d = db.clone();
        let answer_cb = Closure::<dyn FnMut(_)>::new(move |evt: web_sys::Event| {
            if let Some(rating) = evt
                .target()
                .and_then(|x| x.dyn_into::<HtmlButtonElement>().ok())
                .and_then(|x| x.value().parse::<usize>().ok())
            {
                let document = web_sys::window()
                    .expect("failed to get window")
                    .document()
                    .expect("failed to get document");

                let id = document
                    .get_element_by_id("vocab_id")
                    .expect("card id")
                    .dyn_into::<HtmlInputElement>()
                    .expect("expected input element")
                    .value()
                    .parse::<usize>()
                    .expect("invalid id format");

                let db = d.clone();
                spawn_local(async move {
                    // apply review result
                    if let Ok(Some(card)) = db.get_card_by_id(id).await {
                        let fsrs = FSRS::default();
                        let card = fsrs
                            .next(card, Utc::now(), utils::num_to_rating(rating))
                            .card;
                        db.update_card(id, card)
                            .await
                            .expect("failed to update card");

                        Self::show_review_card(db.clone())
                            .await
                            .expect("new review card");
                    }
                });
            }
        });

        for node in self.document.query_selector_all("button.answer")?.values() {
            node?
                .dyn_into::<web_sys::HtmlButtonElement>()?
                .set_onclick(Some(answer_cb.as_ref().unchecked_ref()));
        }
        answer_cb.forget();

        Self::show_review_card(db).await?;

        Ok(())
    }

    pub async fn init_learn(&self, db: Rc<IdxDb>) -> Result<(), JsValue> {
        let options = WorkerOptions::new();
        options.set_type(WorkerType::Module);
        let worker = Rc::new(Worker::new_with_options("/idn/worker.js", &options)?);

        let w = worker.clone();
        let worker_exit_callback = Closure::<dyn FnMut(_)>::new(move |_: Event| {
            w.terminate();
        });
        web_sys::window()
            .ok_or("failed to get window")?
            .set_onbeforeunload(Some(worker_exit_callback.as_ref().unchecked_ref()));
        worker_exit_callback.forget();

        if Self::show_lesson_card(db.clone()).await?.is_some() {
            let next_cb = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let db = db.clone();
                let w = worker.clone();
                spawn_local(async move {
                    let document = web_sys::window()
                        .expect("failed to get window")
                        .document()
                        .expect("failed to get document");
                    let id = document
                        .get_element_by_id("vocab_id")
                        .expect("failed to get vocab_id element")
                        .dyn_ref::<HtmlInputElement>()
                        .expect("expected input element")
                        .value()
                        .parse::<usize>()
                        .expect("invalid id format");
                    let vocab = db.create_card(id).await.expect("failed to create card");
                    w.post_message(&vocab.into()).ok();

                    if let Err(e) = Self::show_lesson_card(db.clone()).await {
                        crate::console_log!("failed to get vocab card: {e:?}");
                    }
                });
            });
            self.document
                .get_element_by_id("next")
                .ok_or("failed to get card element")?
                .dyn_into::<web_sys::HtmlButtonElement>()?
                .set_onclick(Some(next_cb.as_ref().unchecked_ref()));
            next_cb.forget();
        } else {
            crate::console_log!("no new vocabs to learn");
        }

        Ok(())
    }

    pub async fn stats(&self, db: Rc<IdxDb>) -> Result<(), JsValue> {
        let (due, num_reviews, num_vocabs) = db.get_stats().await?;
        if num_vocabs > 0 {
            self.document
                .get_element_by_id("num_vocabs")
                .ok_or("failed to get num_vocab element")?
                .dyn_into::<web_sys::HtmlElement>()?
                .set_inner_text(&format!("{num_vocabs}"));
        }
        if num_reviews > 0 {
            self.document
                .get_element_by_id("num_reviews")
                .ok_or("failed to get num_reviews element")?
                .dyn_into::<web_sys::HtmlElement>()?
                .set_inner_text(&format!("{num_reviews}"));
        }
        if let Some(due) = due {
            self.document
                .get_element_by_id("review_time")
                .expect("time element")
                .dyn_into::<HtmlTimeElement>()
                .expect("expected time element")
                .set_inner_text(&Self::date_to_fancy(due));
        }

        self.document
            .query_selector("body")?
            .ok_or("failed to get body")?
            .remove_attribute("data-loading")?;

        Ok(())
    }

    fn show_vocab(vocab: &crate::db::Vocab) -> Result<(), JsValue> {
        let document = web_sys::window()
            .ok_or("failed to get window")?
            .document()
            .ok_or("failed to get document")?;
        document
            .get_element_by_id("vocab_id")
            .ok_or("failed to get vocab_id element")?
            .dyn_into::<HtmlInputElement>()?
            .set_value(&format!("{}", vocab.id));
        document
            .get_element_by_id("vocab")
            .ok_or("failed to get vocab element")?
            .dyn_into::<HtmlElement>()?
            .set_inner_text(&vocab.vocab);
        document
            .get_element_by_id("meaning")
            .ok_or("failed to get meaning element")?
            .dyn_into::<HtmlElement>()?
            .set_inner_text(&vocab.meaning);
        document
            .get_element_by_id("pos")
            .ok_or("failed to get pos element")?
            .dyn_into::<HtmlElement>()?
            .set_inner_text(&vocab.pos);
        document
            .get_element_by_id("detail")
            .ok_or("failed to get detail element")?
            .dyn_into::<HtmlElement>()?
            .set_inner_text(&vocab.detail);

        let sentences: Vec<&str> = vocab.sentences.split(',').collect();
        let mut output = String::new();
        let num_sentences = sentences.len();

        if num_sentences > 0 && num_sentences.is_multiple_of(2) {
            for s in sentences.as_slice().chunks(2) {
                output.push_str(&format!(
                    "<dt><dfn lang=\"id\">{}</dfn></dt><dd lang=\"en\"><b>{}</b></dd>",
                    s[0], s[1]
                ));
            }
        }

        document
            .get_element_by_id("sentences")
            .ok_or("failed to get sentences element")?
            .dyn_into::<HtmlElement>()?
            .set_inner_html(&output);
        Ok(())
    }

    async fn show_lesson_card(db: Rc<IdxDb>) -> Result<Option<usize>, JsValue> {
        let document = web_sys::window()
            .ok_or("failed to get window")?
            .document()
            .ok_or("failed to get document")?;

        if let Some(vocab) = db.get_lesson_vocab().await? {
            Self::show_vocab(&vocab)?;
            Ok(Some(vocab.id))
        } else {
            document
                .query_selector("section.vocab")?
                .ok_or("card element")?
                .dyn_into::<HtmlElement>()?
                .set_attribute("data-mode", "done")?;
            Ok(None)
        }
    }

    async fn show_review_card(db: Rc<IdxDb>) -> Result<(), JsValue> {
        spawn_local(async move {
            let document = web_sys::window()
                .expect("failed to get window")
                .document()
                .expect("failed to get document");

            match db.get_review_card().await {
                Ok(Some((_, vocab))) => {
                    Self::show_vocab(&vocab).expect("vocab card");
                    document
                        .get_element_by_id("card")
                        .expect("front element")
                        .dyn_into::<HtmlElement>()
                        .expect("expected element")
                        .set_attribute("data-mode", "front")
                        .ok();
                }
                Ok(None) => {
                    if let Ok((Some(date), _)) = db.get_review_stats().await {
                        document
                            .get_element_by_id("review_time")
                            .expect("time element")
                            .dyn_into::<HtmlTimeElement>()
                            .expect("expected time element")
                            .set_inner_html(&Self::date_to_fancy(date));
                    }

                    document
                        .get_element_by_id("card")
                        .expect("card element")
                        .dyn_into::<HtmlElement>()
                        .expect("expected element")
                        .set_attribute("data-mode", "done")
                        .ok();
                }
                _ => (),
            }
        });

        Ok(())
    }

    fn date_to_fancy(date: DateTime<Utc>) -> String {
        let date = date.with_timezone(&chrono::Local::now().timezone());
        let now = chrono::Local::now();

        let day = match date.num_days_from_ce() - now.num_days_from_ce() {
            0 => "today",
            1 => "tomorrow",
            2..6 => &date.weekday().to_string(),
            _ => &date.format("%F").to_string(),
        };

        let time = date.time().format("%H:%M").to_string();

        format!("{day} at {time}")
    }
}
