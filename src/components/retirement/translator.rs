use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  en: &'a str,
  es: &'a str,
  lang: &'a str,
}

#[allow(non_snake_case)]
pub fn Translator<'a>(cx: Scope<'a, Props<'a>>) -> Element {
  if "es".eq(cx.props.lang) {
    render! { "{cx.props.es}" }
  } else {
    render! { "{cx.props.en}" }
  }
}
