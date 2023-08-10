use components::app::App;

pub mod components;

pub fn launch() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
  dioxus_web::launch(App)
}
