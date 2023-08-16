use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      class: "app-home",
      "CroftSoft Dioxus Prototype"
    }
    p {
      class: "app-home",
      "This is the home page."
    }
    }
  }
}
