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
    color: color,
    cursor: cursor,
    disabled: disabled,
    onclick: move |event| cx.props.on_click.call(event),
    opacity: opacity,
    style: r#"
    appearance: "none";
    border-radius: 0.3rem;
    box-shadow:
      rgba(0, 0, 0, 0.2) 0 3px 1px -2px,
      rgba(0, 0, 0, 0.14) 0 2px 2px 0,
      rgba(0, 0, 0, 0.12) 0 1px 5px 0;
    box-sizing: border-box;
    contain: layout style;
    font-family: "Roboto", "Helvetica Neue", san-serif;
    font-kerning: auto;
    font-size: 1.5rem;
    font-style: normal;
    font-weight: 500;
    height: 36px;
    letter-spacing: 0.84px;
    line-height: 19.6px;
    margin: 0;
    outline-color: white;
    outline-style: none;
    outline-width: 0;
    overflow-wrap: break-word;
    padding-bottom: 0;
    /* padding-inline-end: 1rem; */
    /* padding-line-start: 15.4px; */
    padding-left: 1rem;
    padding-right: 1rem;
    padding-top: 0;
    pointer-events: auto;
    position: relative;
    text-align: center;
    text-decoration-color: white;
    text-decoration-line: none;
    text-decoration-style: solid;
    text-overflow: ellipsis;
    text-rendering: optimizelegibility;
    white-space: nowrap;
    "#,
    "{cx.props.label}"
  }
  }
}
