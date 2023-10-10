use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  text: &'a str,
}

#[allow(non_snake_case)]
pub fn QuestionText<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
  render! {
    span {
      class: "app-question-text",
    "{cx.props.text}"
    }
  }
}
