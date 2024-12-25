mod answer_button;
mod link_button;
mod question_text;
mod show_button;

use super::super::components::flashcard::answer_button::{AnswerButton, Mode};
use super::super::components::flashcard::link_button::LinkButton;
use super::super::components::flashcard::question_text::QuestionText;
use super::super::components::flashcard::show_button::ShowButton;
use ::dioxus::prelude::*;
use ::tracing::info;
use ::web_sys::window;

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
#[component]
pub fn Flashcard() -> Element {
  let card = Card::default();
  let mut link_button_disabled_state: Signal<bool> = use_signal(|| true);
  let mut modes: Signal<Vec<Mode>> = use_signal(|| vec![Mode::Untouched; card.answers.len()]);
  let mut show_button_disabled_state: Signal<bool> = use_signal(|| false);
  let mut message_state: Signal<String> = use_signal(|| "".to_owned());
  rsx! {
  div {
    class: "app-flashcard box",
    onclick: move |_event| on_click_flashcard(&mut message_state),
  div {
    display: "flex",
    flex_wrap: "wrap",
    gap: "1rem",
  ShowButton {
    disabled: *show_button_disabled_state.read(),
    on_click: move |event| on_click_show_button(
      card.correct_answer_index,
      event,
      &mut link_button_disabled_state,
      &mut modes,
      &mut show_button_disabled_state,
    ),
  }
  LinkButton {
    disabled: *link_button_disabled_state.read(),
    on_click: on_click_link_button,
  }
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
      mode: modes.with(|modes| modes[index]),
      on_click: move |event| on_click_answer_button(
        card.correct_answer_index,
        event,
        index,
        &mut link_button_disabled_state,
        &mut message_state,
        &mut modes,
        &mut show_button_disabled_state,
      ),
    }
  }
  }
  p {
    "{message_state}"
  }
  }
  }
}

fn on_click_answer_button(
  correct_answer_index: usize,
  event: MouseEvent,
  index: usize,
  link_button_disabled_state: &mut Signal<bool>,
  message_state: &mut Signal<String>,
  modes_state: &mut Signal<Vec<Mode>>,
  show_button_disabled_state: &mut Signal<bool>,
) {
  tracing::info!("AnswerButton clicked: {event:?}");
  if index == correct_answer_index {
    if let Mode::Correct = modes_state.with(|modes| modes[index]) {
      reset(
        link_button_disabled_state,
        message_state,
        modes_state,
        show_button_disabled_state,
      );
      return;
    }
    modes_state.with_mut(|modes| {
      modes.fill(Mode::Disabled);
      modes[index] = Mode::Correct;
    });
    link_button_disabled_state.set(false);
    show_button_disabled_state.set(true);
    message_state.set("Correct.".to_owned());
  } else {
    modes_state.with_mut(|modes| modes[index] = Mode::Incorrect);
    message_state.set("".to_owned());
  }
}

fn on_click_flashcard(message_state: &mut Signal<String>) {
  message_state.set("Click on an answer button to continue.".to_owned());
}

fn on_click_link_button(event: MouseEvent) {
  info!("Clicked! {event:?}");
  let _ = window().unwrap().open_with_url_and_target(
    "https://en.wikipedia.org/wiki/Rust_(programming_language)",
    "_blank",
  );
}

fn on_click_show_button(
  correct_answer_index: usize,
  event: MouseEvent,
  link_button_disabled_state: &mut Signal<bool>,
  modes_state: &mut Signal<Vec<Mode>>,
  show_button_disabled_state: &mut Signal<bool>,
) {
  info!("Show button clicked: {event:?}");
  modes_state.with_mut(|modes| {
    modes.fill(Mode::Disabled);
    modes[correct_answer_index] = Mode::Correct;
  });
  link_button_disabled_state.set(false);
  show_button_disabled_state.set(true);
}

fn reset(
  link_button_disabled_state: &mut Signal<bool>,
  message_state: &mut Signal<String>,
  modes_state: &mut Signal<Vec<Mode>>,
  show_button_disabled_state: &mut Signal<bool>,
) {
  link_button_disabled_state.set(true);
  message_state.set("".to_owned());
  modes_state.with_mut(|modes| modes.fill(Mode::Untouched));
  show_button_disabled_state.set(false);
}
