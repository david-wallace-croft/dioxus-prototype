mod answer_button;

use crate::components::flashcard::answer_button::AnswerButton;
use dioxus::prelude::*;

// struct AnswerData {
//   incorrect: bool,
//   label: &str,
// }

// impl AnswerData {
//   fn new(label: &str) -> Self {
//     Self {
//       incorrect: false,
//       label,
//     }
//   }
// }

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  let answers: [&str; 10] = [
    "0", "4", "12", "14", "42", "44", "48", "55", "84", "99",
  ];
  // let incorrects: [bool; 10] = [false; 10];
  let mut incorrects: &UseState<[bool; 10]> = use_state(cx, || [false; 10]);
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
    for (index, answer) in answers.iter().enumerate() {
      AnswerButton {
        incorrect: incorrects[index],
        label: answer,
        on_click: move |event| on_click(event, incorrects, index),
      }
    }
    }
    }
  }
}

// fn is_incorrect(incorrect: &UseState<bool>) -> bool {
//   let incorrect: bool = *incorrect.get();
//   log::info!("incorrect = {incorrect:?}");
//   incorrect
// }

fn on_click(
  event: MouseEvent,
  incorrects: &UseState<[bool; 10]>,
  index: usize,
) {
  log::info!("Clicked! {event:?}");
  event.stop_propagation();
  let mut incorrects_copy = *incorrects.get();
  incorrects_copy[index] = true;
  incorrects.set(incorrects_copy);
}
