use self::card::Card;
use self::mode::Mode;
use super::super::components::flashcard::answer_button::AnswerButton;
use super::super::components::flashcard::link_button::LinkButton;
use super::super::components::flashcard::question_text::QuestionText;
use super::super::components::flashcard::show_button::ShowButton;
use ::dioxus::prelude::*;
use ::tracing::info;
use ::web_sys::window;

mod answer_button;
mod card;
mod link_button;
mod mode;
mod question_text;
mod show_button;

#[allow(non_snake_case)]
#[component]
pub fn Flashcard() -> Element {
  let card = Card::default();

  let mut link_button_disabled_signal: Signal<bool> = use_signal(|| true);

  let mut modes_signal: Signal<Vec<Mode>> =
    use_signal(|| vec![Mode::Untouched; card.answers.len()]);

  let mut show_button_disabled_signal: Signal<bool> = use_signal(|| false);

  let mut message_signal: Signal<String> = use_signal(|| "".to_owned());

  rsx! {
  div {
    class: "app-flashcard box",
    onclick: move |_event| on_click_flashcard(&mut message_signal),
  div {
    display: "flex",
    flex_wrap: "wrap",
    gap: "1rem",
  ShowButton {
    disabled: *show_button_disabled_signal.read(),
    on_click: move |event| on_click_show_button(
      card.correct_answer_index,
      event,
      &mut link_button_disabled_signal,
      &mut modes_signal,
      &mut show_button_disabled_signal,
    ),
  }
  LinkButton {
    disabled: *link_button_disabled_signal.read(),
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
        mode: modes_signal.with(|modes| modes[index]),
        on_click: move |event| on_click_answer_button(
          card.correct_answer_index,
          event,
          index,
          &mut link_button_disabled_signal,
          &mut message_signal,
          &mut modes_signal,
          &mut show_button_disabled_signal,
        ),
      }
    }
  }
  p {
    "{message_signal}"
  }
  }
  }
}

fn on_click_answer_button(
  correct_answer_index: usize,
  event: MouseEvent,
  index: usize,
  link_button_disabled_signal: &mut Signal<bool>,
  message_signal: &mut Signal<String>,
  modes_signal: &mut Signal<Vec<Mode>>,
  show_button_disabled_signal: &mut Signal<bool>,
) {
  info!("AnswerButton clicked: {event:?}");

  if index == correct_answer_index {
    if let Mode::Correct = modes_signal.with(|modes| modes[index]) {
      reset(
        link_button_disabled_signal,
        message_signal,
        modes_signal,
        show_button_disabled_signal,
      );

      return;
    }

    modes_signal.with_mut(|modes| {
      modes.fill(Mode::Disabled);

      modes[index] = Mode::Correct;
    });

    link_button_disabled_signal.set(false);

    show_button_disabled_signal.set(true);

    message_signal.set("Correct.".to_owned());
  } else {
    modes_signal.with_mut(|modes| modes[index] = Mode::Incorrect);

    message_signal.set("".to_owned());
  }
}

fn on_click_flashcard(message_signal: &mut Signal<String>) {
  message_signal.set("Click on an answer button to continue.".to_owned());
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
  link_button_disabled_signal: &mut Signal<bool>,
  modes_signal: &mut Signal<Vec<Mode>>,
  show_button_disabled_signal: &mut Signal<bool>,
) {
  info!("Show button clicked: {event:?}");

  modes_signal.with_mut(|modes| {
    modes.fill(Mode::Disabled);

    modes[correct_answer_index] = Mode::Correct;
  });

  link_button_disabled_signal.set(false);

  show_button_disabled_signal.set(true);
}

fn reset(
  link_button_disabled_signal: &mut Signal<bool>,
  message_signal: &mut Signal<String>,
  modes_signal: &mut Signal<Vec<Mode>>,
  show_button_disabled_signal: &mut Signal<bool>,
) {
  link_button_disabled_signal.set(true);

  message_signal.set("".to_owned());

  modes_signal.with_mut(|modes| modes.fill(Mode::Untouched));

  show_button_disabled_signal.set(false);
}
