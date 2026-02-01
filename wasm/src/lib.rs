use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use std::rc::Rc;

mod db;
mod ui;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Idn {
    db: Rc<db::IdxDb>,
    ui: Option<Rc<ui::Ui>>,
}

#[wasm_bindgen]
impl Idn {
    #[wasm_bindgen]
    pub async fn new() -> Result<Idn, JsValue> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let db = Rc::new(db::IdxDb::new().await?);

        if Self::is_worker() {
            Ok(Self { db, ui: None })
        } else {
            JsFuture::from(
                web_sys::window()
                    .ok_or("failed to get window")?
                    .navigator()
                    .service_worker()
                    .register("/idn/service_worker.js"),
            )
            .await?;

            let ui = ui::Ui::new()?;

            Ok(Self {
                db,
                ui: Some(Rc::new(ui)),
            })
        }
    }

    // Determines whether the code is running as a worker or main thread script
    pub fn is_worker() -> bool {
        web_sys::window().is_none()
    }

    #[wasm_bindgen]
    pub async fn review(&self) -> Result<(), JsValue> {
        if let Some(ref ui) = self.ui {
            ui.init_review(self.db.clone()).await?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn learn(&self) -> Result<(), JsValue> {
        if let Some(ref ui) = self.ui {
            ui.init_learn(self.db.clone()).await?
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn stats(&self) -> Result<(), JsValue> {
        if let Some(ref ui) = self.ui {
            ui.stats(self.db.clone()).await?;
        }
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn add_sentences(&self, vocab: &str) -> Result<(), JsValue> {
        self.db.add_sentences(vocab).await
    }
}
