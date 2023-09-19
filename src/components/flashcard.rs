use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  let mut count = use_state(cx, || 0);
  render! {
    div {
      class: "app-high-five box",
      h1 {
        "High-Five counter: {count}",
      }
      button {
        class: "app-high-five",
        onclick: move |_| count -= 1,
        "Down low!",
      }
      button {
        class: "app-high-five",
        onclick: move |_| count += 1,
        "Up high!",
      }
    }
  }
}
