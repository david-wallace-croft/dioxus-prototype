use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
      h1 { class: "app-home", "Home Page" }
      p { "This is the home page." }
  }
}
