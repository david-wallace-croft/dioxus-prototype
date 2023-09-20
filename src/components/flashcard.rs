use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  // let mut count = use_state(cx, || 0);
  render! {
    div {
      class: "box",
    div {
    button {
    "SHOW"
    }
    }
    div {
    "8 x 6 = ?"
    }
    div {
    button {
    "0"
    }
    button {
    "4"
    }
    button {
    "12"
    }
    button {
    "14"
    }
    button {
    "42"
    }
    button {
    "44"
    }
    button {
    "48"
    }
    button {
    "55"
    }
    button {
    "84"
    }
    button {
    "99"
    }
    }
    }
  }
}
