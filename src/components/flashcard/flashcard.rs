use crate::components::flashcard::answer_button::{AnswerButton, Mode};
use crate::components::flashcard::question_text::QuestionText;
use crate::components::flashcard::show_button::ShowButton;
use dioxus::prelude::*;

struct Card {
  answers: Vec<String>,
  correct_answer_index: usize,
}

impl Default for Card {
  fn default() -> Self {
    let answers: Vec<String> = vec![
      "0", "4", "12", "14", "42", "44", "48", "55", "84", "99",
    ]
    .into_iter()
    .map(|s| s.to_owned())
    .collect();
    let correct_answer_index = 6;
    Self {
      answers,
      correct_answer_index,
    }
  }
}

#[allow(non_snake_case)]
pub fn Flashcard(cx: Scope) -> Element {
  let card = Card::default();
  let modes: &UseState<Vec<Mode>> =
    use_state(cx, || vec![Mode::Untouched; card.answers.len()]);
  let show_button_disabled_state: &UseState<bool> = use_state(cx, || false);
  render! {
    div {
      class: "app-flashcard box",
    ShowButton {
      disabled: *show_button_disabled_state.get(),
      on_click: move |event| on_click_show_button(
        card.correct_answer_index,
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
    for (index, answer) in card.answers.into_iter().enumerate() {
      AnswerButton {
        label: answer,
        mode: modes[index],
        on_click: move |event| on_click_answer_button(
          card.correct_answer_index,
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
  modes_state: &UseState<Vec<Mode>>,
  show_button_disabled_state: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  if index == correct_answer_index {
    if let Mode::Correct = modes_state.get()[index] {
      reset(modes_state, show_button_disabled_state);
      return;
    }
    let mut modes = vec![Mode::Disabled; modes_state.get().len()];
    modes[index] = Mode::Correct;
    modes_state.set(modes);
    show_button_disabled_state.set(true);
  } else {
    let mut modes: Vec<Mode> =
      modes_state.get().iter().map(|mode| *mode).collect();
    modes[index] = Mode::Incorrect;
    modes_state.set(modes);
  }
}

fn on_click_show_button(
  correct_answer_index: usize,
  event: MouseEvent,
  modes_state: &UseState<Vec<Mode>>,
  show_button_disabled_state: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  let mut modes = vec![Mode::Disabled; modes_state.get().len()];
  modes[correct_answer_index] = Mode::Correct;
  modes_state.set(modes);
  show_button_disabled_state.set(true);
}

fn reset(
  modes_state: &UseState<Vec<Mode>>,
  show_button_disabled_state: &UseState<bool>,
) {
  let modes = vec![Mode::Untouched; modes_state.get().len()];
  modes_state.set(modes);
  show_button_disabled_state.set(false);
}
