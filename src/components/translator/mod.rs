use dioxus::prelude::*;

pub struct SharedState {
  pub lang: String,
}

#[derive(Props)]
pub struct Props<'a> {
  en: &'a str,
  es: &'a str,
}

#[allow(non_snake_case)]
pub fn Translator<'a>(cx: Scope<'a, Props<'a>>) -> Element {
  let use_shared_state_option = use_shared_state::<SharedState>(cx);
  let shared_state_lang: String = match use_shared_state_option {
    Some(use_shared_state) => use_shared_state.read().lang.clone(),
    None => "en".to_string(),
  };
  if "es".eq(&shared_state_lang) {
    render! { "{cx.props.es}" }
  } else {
    render! { "{cx.props.en}" }
  }
}
