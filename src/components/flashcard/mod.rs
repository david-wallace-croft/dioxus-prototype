mod answer_button;

use crate::components::flashcard::answer_button::AnswerButton;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  let mut incorrect: &UseState<bool> = use_state(cx, || false);
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
    AnswerButton {
      incorrect: is_incorrect(incorrect),
      label: "0",
      on_click: move |event| on_click(event, incorrect),
    },
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

fn is_incorrect(incorrect: &UseState<bool>) -> bool {
  let incorrect: bool = *incorrect.get();
  log::info!("incorrect = {incorrect:?}");
  incorrect
}

fn on_click(
  event: MouseEvent,
  incorrect: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  event.stop_propagation();
  incorrect.set(true);
}
