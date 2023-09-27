mod answer_button;
mod question_text;
mod show_button;

use crate::components::flashcard::answer_button::{AnswerButton, Mode};
use crate::components::flashcard::question_text::QuestionText;
use crate::components::flashcard::show_button::ShowButton;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  // TODO: change to Vec
  let answers: [&str; 10] = [
    "0", "4", "12", "14", "42", "44", "48", "55", "84", "99",
  ];
  const CORRECT_ANSWER_INDEX: usize = 6;
  // TODO: change to Vec
  let modes: &UseState<[Mode; 10]> = use_state(cx, || [Mode::Untouched; 10]);
  render! {
    div {
      class: "app-flashcard box",
    ShowButton { }
    div {
      margin: "2rem 0",
    QuestionText {
      text: "8 x 6 = ?"
    }
    }
    div {
      display: "flex",
      flex_wrap: "wrap",
      gap: "1rem",
    for (index, answer) in answers.iter().enumerate() {
      AnswerButton {
        label: answer,
        mode: modes[index],
        on_click: move |event| on_click(CORRECT_ANSWER_INDEX, event, index, modes),
      }
    }
    }
    }
  }
}

fn on_click(
  correct_answer_index: usize,
  event: MouseEvent,
  index: usize,
  modes_state: &UseState<[Mode; 10]>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  if index == correct_answer_index {
    let mut modes = [Mode::Disabled; 10];
    modes[index] = Mode::Correct;
    modes_state.set(modes);
  } else {
    let mut modes = *modes_state.get();
    modes[index] = Mode::Incorrect;
    modes_state.set(modes);
  }
}
