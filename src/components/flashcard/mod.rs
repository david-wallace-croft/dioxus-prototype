mod answer_button;
mod question_text;
mod show_button;

use crate::components::flashcard::answer_button::{AnswerButton, Mode};
use crate::components::flashcard::question_text::QuestionText;
use crate::components::flashcard::show_button::ShowButton;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  let answers: [&str; 10] = [
    "0", "4", "12", "14", "42", "44", "48", "55", "84", "99",
  ];
  const CORRECT_ANSWER_INDEX: usize = 6;
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
  modes: &UseState<[Mode; 10]>,
) {
  log::info!("Clicked! {event:?}");
  event.stop_propagation();
  if index == correct_answer_index {
    let mut modes_copy = *modes.get();
    for index in 0..modes_copy.len() {
      modes_copy[index] = Mode::Disabled;
    }
    modes_copy[index] = Mode::Correct;
    modes.set(modes_copy);
  } else {
    let mut modes_copy = *modes.get();
    modes_copy[index] = Mode::Incorrect;
    modes.set(modes_copy);
  }
}
