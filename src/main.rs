use dioxus_prototype::App;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
  dioxus_web::launch(App)
}
