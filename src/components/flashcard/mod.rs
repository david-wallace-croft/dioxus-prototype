mod answer_button;
mod question_text;
mod show_button;

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
  let modes: &UseRef<Vec<Mode>> =
    use_ref(cx, || vec![Mode::Untouched; card.answers.len()]);
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
    // TODO: LinkButton
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
        mode: modes.with(|modes| modes[index]),
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
  modes_state: &UseRef<Vec<Mode>>,
  show_button_disabled_state: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  if index == correct_answer_index {
    if let Mode::Correct = modes_state.with(|modes| modes[index]) {
      reset(modes_state, show_button_disabled_state);
      return;
    }
    modes_state.with_mut(|modes| {
      for i in 0..modes.len() {
        modes[i] = Mode::Disabled;
      }
      modes[index] = Mode::Correct;
    });
    show_button_disabled_state.set(true);
  } else {
    modes_state.with_mut(|modes| modes[index] = Mode::Incorrect);
  }
}

fn on_click_show_button(
  correct_answer_index: usize,
  event: MouseEvent,
  modes_state: &UseRef<Vec<Mode>>,
  show_button_disabled_state: &UseState<bool>,
) {
  log::info!("Clicked! {event:?}");
  // TODO: Necessary?
  event.stop_propagation();
  modes_state.with_mut(|modes| {
    for i in 0..modes.len() {
      modes[i] = Mode::Disabled;
    }
    modes[correct_answer_index] = Mode::Correct;
  });
  show_button_disabled_state.set(true);
}

fn reset(
  modes_state: &UseRef<Vec<Mode>>,
  show_button_disabled_state: &UseState<bool>,
) {
  modes_state.with_mut(|modes| {
    for i in 0..modes.len() {
      modes[i] = Mode::Untouched;
    }
  });
  show_button_disabled_state.set(false);
}
