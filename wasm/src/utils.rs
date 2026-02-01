use rs_fsrs::{Rating, State};
use wasm_bindgen::{JsValue, prelude::*};
use wasm_bindgen_futures::JsFuture;

// Fetches a given amount of indonesian sentences with english translation from tatoeba via the API
// https://api.tatoeba.org/unstable#?route=get-/unstable/sentences
pub async fn fetch_tatoeba(
    query: &str,
    max_words: usize,
    limit: usize,
) -> Result<JsValue, JsValue> {
    let url = format!(
        r"https://api.tatoeba.org/unstable/sentences?lang=ind&q={query}&word_count=2-{max_words}&limit={limit}&is_orphan=no&is_unapproved=no&is_native=yes&license=%21PROBLEM&sort=random&showtrans%3Alang=eng&showtrans%3Ais_direct=yes"
    );

    let response = JsFuture::from(
        js_sys::global()
            .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()?
            .fetch_with_str(url.as_str()),
    )
    .await?
    .dyn_into::<web_sys::Response>()?;

    let json = JsFuture::from(response.json()?).await?;

    js_sys::Reflect::get(&json, &"data".into())
}

// Removes unnecessary fields from a sentence object returned by tatoeba's API
// Owner and license data are preserved for proper attribution
pub fn clean_sentence(sentence: &JsValue) -> Result<(), JsValue> {
    let obj = sentence
        .dyn_ref::<js_sys::Object>()
        .ok_or("invalid sentence data")?;
    js_sys::Reflect::delete_property(obj, &"lang".into())?;
    js_sys::Reflect::delete_property(obj, &"script".into())?;
    js_sys::Reflect::delete_property(obj, &"transcriptions".into())?;
    js_sys::Reflect::delete_property(obj, &"audios".into())?;

    let translations = js_sys::Array::from(&js_sys::Reflect::get(obj, &"translations".into())?);

    for t in translations.iter() {
        let obj = t
            .dyn_ref::<js_sys::Object>()
            .ok_or("invalid translation data")?;
        js_sys::Reflect::delete_property(obj, &"lang".into())?;
        js_sys::Reflect::delete_property(obj, &"script".into())?;
        js_sys::Reflect::delete_property(obj, &"transcriptions".into())?;
        js_sys::Reflect::delete_property(obj, &"audios".into())?;
        js_sys::Reflect::delete_property(obj, &"is_direct".into())?;
    }

    Ok(())
}

pub fn card_to_object(card: rs_fsrs::Card) -> Result<js_sys::Object, JsValue> {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &"due".into(),
        &format!("{}", card.due.timestamp()).into(),
    )?;
    js_sys::Reflect::set(&obj, &"stability".into(), &card.stability.into())?;
    js_sys::Reflect::set(&obj, &"difficulty".into(), &card.difficulty.into())?;
    js_sys::Reflect::set(
        &obj,
        &"elapsed_days".into(),
        &(card.elapsed_days as i32).into(),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"scheduled_days".into(),
        &(card.scheduled_days as i32).into(),
    )?;
    js_sys::Reflect::set(&obj, &"reps".into(), &card.reps.into())?;
    js_sys::Reflect::set(&obj, &"lapses".into(), &card.lapses.into())?;
    js_sys::Reflect::set(&obj, &"state".into(), &(card.state as usize).into())?;
    js_sys::Reflect::set(
        &obj,
        &"last_review".into(),
        &format!("{}", card.last_review.timestamp()).into(),
    )?;
    Ok(obj)
}

pub fn object_to_card(obj: wasm_bindgen::JsValue) -> Result<rs_fsrs::Card, JsValue> {
    Ok(rs_fsrs::Card {
        due: chrono::DateTime::from_timestamp(
            js_sys::Reflect::get(&obj, &"due".into())?
                .as_string()
                .expect("datetime string")
                .parse::<i64>()
                .expect("unix timestamp"),
            0,
        )
        .expect("valid datetime"),
        stability: js_sys::Reflect::get(&obj, &"stability".into())?
            .as_f64()
            .ok_or("stability has to be a float")?,
        difficulty: js_sys::Reflect::get(&obj, &"difficulty".into())?
            .as_f64()
            .ok_or("difficulty has to be a float")?,
        elapsed_days: js_sys::Reflect::get(&obj, &"elapsed_days".into())?
            .as_f64()
            .ok_or("elapsed_days has to be a number")? as i64,
        scheduled_days: js_sys::Reflect::get(&obj, &"scheduled_days".into())?
            .as_f64()
            .ok_or("scheduled_days has to be a number")? as i64,
        reps: js_sys::Reflect::get(&obj, &"reps".into())?
            .as_f64()
            .ok_or("reps has to be a number")? as i32,
        lapses: js_sys::Reflect::get(&obj, &"lapses".into())?
            .as_f64()
            .ok_or("lapses has to be a number")? as i32,
        state: num_to_state(
            js_sys::Reflect::get(&obj, &"state".into())?
                .as_f64()
                .ok_or("state has to be a number")? as usize,
        ),
        last_review: chrono::DateTime::from_timestamp(
            js_sys::Reflect::get(&obj, &"last_review".into())?
                .as_string()
                .ok_or("datetime string")?
                .parse::<i64>()
                .expect("unix timestamp"),
            0,
        )
        .ok_or("valid datetime")?,
    })
}

pub fn num_to_state(val: usize) -> State {
    match val {
        0 => State::New,
        1 => State::Learning,
        2 => State::Review,
        3 => State::Relearning,
        _ => panic!("invalid state value"),
    }
}

pub fn num_to_rating(val: usize) -> Rating {
    match val {
        0 => Rating::Again,
        1 => Rating::Hard,
        2 => Rating::Good,
        3 => Rating::Easy,
        _ => panic!("invalid state value"),
    }
}
