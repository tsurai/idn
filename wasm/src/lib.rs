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
    ui: Rc<ui::Ui>,
}

#[wasm_bindgen]
impl Idn {
    #[wasm_bindgen]
    pub async fn new(version: u32) -> Result<Idn, JsValue> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        JsFuture::from(web_sys::window()
            .ok_or("failed to get window")?
            .navigator()
            .service_worker()
            .register("/idn/worker.js"))
            .await?;

        let db = Rc::new(db::IdxDb::new(version).await?);
        let ui = ui::Ui::new()?;

        Ok(Self {
            db,
            ui: Rc::new(ui),
        })
    }

    #[wasm_bindgen]
    pub async fn review(&self) -> Result<(), JsValue> {
        self.ui.init_review(self.db.clone()).await?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn learn(&self) {
        if let Err(e) = self.ui.init_learn(self.db.clone()).await {
            crate::console_log!("error: {e:?}");
        }
    }

    #[wasm_bindgen]
    pub async fn stats(&self) {
        if let Err(e) = self.ui.stats(self.db.clone()).await {
            crate::console_log!("error: {e:?}");
        }
    }
}
