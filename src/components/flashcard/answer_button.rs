use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub enum Mode {
  Correct,
  Disabled,
  Incorrect,
  Untouched,
}

#[derive(Props)]
pub struct Props<'a> {
  label: String,
  mode: Mode,
  on_click: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn AnswerButton<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
  let background_color = match cx.props.mode {
    Mode::Correct => "#19dc60",
    Mode::Disabled => "#FFF",
    Mode::Incorrect => "#f04141",
    Mode::Untouched => "#3880FF",
  };
  let color = match cx.props.mode {
    Mode::Disabled => "black",
    _ => "white",
  };
  let cursor = match cx.props.mode {
    Mode::Correct | Mode::Untouched => "pointer",
    _ => "default",
  };
  let disabled = matches!(cx.props.mode, Mode::Disabled | Mode::Incorrect);
  let opacity = match cx.props.mode {
    Mode::Disabled | Mode::Incorrect => "0.5",
    _ => "1.0",
  };
  render! {
  button {
    background_color: background_color,
    class: "app-answer-button",
    color: color,
    cursor: cursor,
    disabled: disabled,
    onclick: move |event| cx.props.on_click.call(event),
    opacity: opacity,
    "{cx.props.label}"
  }
  }
}
