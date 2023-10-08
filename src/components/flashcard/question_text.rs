use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  text: &'a str,
}

#[allow(non_snake_case)]
pub fn QuestionText<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
  render! {
    span {
      style: r#"
        color: rgb(34, 36, 40);
        font-family: 'Roboto', 'Helvetica Neue', sans-serif;
        font-size: 1.5rem;
        font-weight: 700;
        overflow-wrap: break-word;
        text-rendering: optimizelegibility;
        "#,
    "{cx.props.text}"
    }
  }
}
