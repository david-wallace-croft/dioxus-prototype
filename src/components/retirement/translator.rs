use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  en: &'a str,
  es: &'a str,
  use_es: bool,
}

#[allow(non_snake_case)]
pub fn Translator<'a>(cx: Scope<'a, Props<'a>>) -> Element {
  if cx.props.use_es {
    render! { "{cx.props.es}" }
  } else {
    render! { "{cx.props.en}" }
  }
}
