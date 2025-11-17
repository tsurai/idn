use wasm_bindgen::JsValue;
use rs_fsrs::{Rating, State};

pub fn card_to_object(card: rs_fsrs::Card) -> Result<js_sys::Object, JsValue> {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &"due".into(),
        &format!("{}", card.due.timestamp()).into())?;
    js_sys::Reflect::set(
        &obj,
        &"stability".into(),
        &card.stability.into())?;
    js_sys::Reflect::set(
        &obj,
        &"difficulty".into(),
        &card.difficulty.into())?;
    js_sys::Reflect::set(
        &obj,
        &"elapsed_days".into(),
        &(card.elapsed_days as i32).into())?;
    js_sys::Reflect::set(
        &obj,
        &"scheduled_days".into(),
        &(card.scheduled_days as i32).into())?;
    js_sys::Reflect::set(
        &obj,
        &"reps".into(),
        &card.reps.into())?;
    js_sys::Reflect::set(
        &obj,
        &"lapses".into(),
        &card.lapses.into())?;
    js_sys::Reflect::set(
        &obj,
        &"state".into(),
        &(card.state as usize).into())?;
    js_sys::Reflect::set(
        &obj,
        &"last_review".into(),
        &format!("{}", card.last_review.timestamp()).into())?;
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
            0)
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
        state: num_to_state(js_sys::Reflect::get(&obj, &"state".into())?
            .as_f64()
            .ok_or("state has to be a number")? as usize),
        last_review: chrono::DateTime::from_timestamp(
            js_sys::Reflect::get(&obj, &"last_review".into())?
                .as_string()
                .ok_or("datetime string")?
                .parse::<i64>()
                .expect("unix timestamp"),
            0)
            .ok_or("valid datetime")?,
    })
}

pub fn num_to_state(val: usize) -> State {
    match val {
        0 => State::New,
        1 => State::Learning,
        2 => State::Review,
        3 => State::Relearning,
        _ => panic!("invalid state value")
    }
}

pub fn num_to_rating(val: usize) -> Rating {
    match val {
        0 => Rating::Again,
        1 => Rating::Hard,
        2 => Rating::Good,
        3 => Rating::Easy,
        _ => panic!("invalid state value")
    }
}
