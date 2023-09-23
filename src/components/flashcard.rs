use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  // let mut count = use_state(cx, || 0);
  render! {
    div {
      class: "app-flashcard box",
    div {
    button {
      class: "app-show",
    "SHOW"
    }
    }
    div {
      class: "app-question",
    "8 x 6 = ?"
    }
    div {
    button {
      class: "app-answer",
    "0"
    }
    button {
      class: "app-answer",
    "4"
    }
    button {
      class: "app-answer",
    "12"
    }
    button {
      class: "app-answer",
    "14"
    }
    button {
      class: "app-answer",
    "42"
    }
    button {
      class: "app-answer",
    "44"
    }
    button {
      class: "app-answer",
    "48"
    }
    button {
      class: "app-answer",
    "55"
    }
    button {
      class: "app-answer",
    "84"
    }
    button {
      class: "app-answer",
    "99"
    }
    }
    }
  }
}
