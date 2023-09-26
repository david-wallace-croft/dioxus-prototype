use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  correct: bool,
  incorrect: bool,
  label: &'a str,
  on_click: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn AnswerButton<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
  render! {
  button {
    background_color: if cx.props.incorrect { "#f04141" }
      else if cx.props.correct { "#19dc60" }
      else { "#3880FF" },
    cursor: if cx.props.incorrect { "default" } else { "pointer" },
    disabled: cx.props.incorrect,
    onclick: move |event| cx.props.on_click.call(event),
    opacity: if cx.props.incorrect { "0.5" } else { "1.0" },
    style: r#"
    appearance: "none";
    border-radius: 0.3rem;
    box-shadow:
      rgba(0, 0, 0, 0.2) 0 3px 1px -2px,
      rgba(0, 0, 0, 0.14) 0 2px 2px 0,
      rgba(0, 0, 0, 0.12) 0 1px 5px 0;
    box-sizing: border-box;
    color: white;
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
