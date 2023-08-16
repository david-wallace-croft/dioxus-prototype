use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Blog(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      "Blog Page"
    }
    }
  }
}
