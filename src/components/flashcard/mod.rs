mod answer_button;
mod question_text;
mod show_button;

use crate::components::flashcard::answer_button::AnswerButton;
use crate::components::flashcard::question_text::QuestionText;
use crate::components::flashcard::show_button::ShowButton;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  let answers: [&str; 10] = [
    "0", "4", "12", "14", "42", "44", "48", "55", "84", "99",
  ];
  const CORRECT_ANSWER_INDEX: usize = 6;
  let corrects: &UseState<[bool; 10]> = use_state(cx, || [false; 10]);
  let incorrects: &UseState<[bool; 10]> = use_state(cx, || [false; 10]);
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
        correct: corrects[index],
        incorrect: incorrects[index],
        label: answer,
        on_click: move |event| on_click(CORRECT_ANSWER_INDEX, corrects, event, incorrects, index),
      }
    }
    }
    }
  }
}

fn on_click(
  correct_answer_index: usize,
  corrects: &UseState<[bool; 10]>,
  event: MouseEvent,
  incorrects: &UseState<[bool; 10]>,
  index: usize,
) {
  log::info!("Clicked! {event:?}");
  event.stop_propagation();
  if index == correct_answer_index {
    let mut corrects_copy = *corrects.get();
    corrects_copy[index] = true;
    corrects.set(corrects_copy);
  } else {
    let mut incorrects_copy = *incorrects.get();
    incorrects_copy[index] = true;
    incorrects.set(incorrects_copy);
  }
}
