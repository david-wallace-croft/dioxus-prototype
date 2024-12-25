use ::dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
  Correct,
  Disabled,
  Incorrect,
  Untouched,
}

#[allow(non_snake_case)]
#[component]
pub fn AnswerButton(label: String, mode: Mode, on_click: EventHandler<MouseEvent>) -> Element {
  let background_color: &str = match mode {
    Mode::Correct => "#19dc60",
    Mode::Disabled => "#FFF",
    Mode::Incorrect => "#f04141",
    Mode::Untouched => "#3880FF",
  };

  let color: &str = match mode {
    Mode::Disabled => "black",
    _ => "white",
  };

  let cursor: &str = match mode {
    Mode::Correct | Mode::Untouched => "pointer",
    _ => "default",
  };

  let disabled: bool = matches!(mode, Mode::Disabled | Mode::Incorrect);

  let opacity: &str = match mode {
    Mode::Disabled | Mode::Incorrect => "0.5",
    _ => "1.0",
  };

  rsx! {
  button {
    background_color: background_color,
    class: "app-answer-button",
    color: color,
    cursor: cursor,
    disabled: disabled,
    onclick: move |event| on_click.call(event),
    opacity: opacity,
    "{label}"
  }
  }
}
