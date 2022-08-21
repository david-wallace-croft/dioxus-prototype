use dioxus_prototype::app;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
  dioxus::web::launch(app)
}
