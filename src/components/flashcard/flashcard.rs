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
  let show_button_disabled_state: &UseState<bool> = use_state(cx, || false);
  render! {
    div {
      class: "app-flashcard box",
    ShowButton {
      disabled: *show_button_disabled_state.get(),
      on_click: move |event| on_click_show_button(
        CORRECT_ANSWER_INDEX,
        event,
        modes,
        show_button_disabled_state,
      ),
    }
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
        on_click: move |event| on_click_answer_button(
          CORRECT_ANSWER_INDEX,
          event,
          index,
          modes,
          show_button_disabled_state,
        ),
      }
    }
    }
    }
  }
}

fn on_click_answer_button(
  correct_answer_index: usize,
  event: MouseEvent,
  index: usize,
  modes_state: &UseState<[Mode; 10]>,
  show_button_disabled_state: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  if index == correct_answer_index {
    let modes: [Mode; 10] = *modes_state.get();
    if let Mode::Correct = modes[index] {
      reset(modes_state, show_button_disabled_state);
      return;
    }
    let mut modes = [Mode::Disabled; 10];
    modes[index] = Mode::Correct;
    modes_state.set(modes);
    show_button_disabled_state.set(true);
  } else {
    let mut modes = *modes_state.get();
    modes[index] = Mode::Incorrect;
    modes_state.set(modes);
  }
}

fn on_click_show_button(
  correct_answer_index: usize,
  event: MouseEvent,
  modes_state: &UseState<[Mode; 10]>,
  show_button_disabled_state: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  let mut modes = [Mode::Disabled; 10];
  modes[correct_answer_index] = Mode::Correct;
  modes_state.set(modes);
  show_button_disabled_state.set(true);
}

fn reset(
  modes_state: &UseState<[Mode; 10]>,
  show_button_disabled_state: &UseState<bool>,
) {
  let modes = [Mode::Untouched; 10];
  modes_state.set(modes);
  show_button_disabled_state.set(false);
}
