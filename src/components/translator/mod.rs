use ::dioxus::prelude::*;

use super::language_select::LANGUAGE;

pub struct SharedState {
  pub lang: String,
}

#[allow(non_snake_case)]
#[component]
pub fn Translator(
  en: &'static str,
  es: &'static str,
) -> Element {
  let language_result = LANGUAGE.try_read();

  let language: String =
    language_result.map_or("en".to_string(), |lang| lang.clone());

  if "es".eq(&language) {
    rsx! { "{es}" }
  } else {
    rsx! { "{en}" }
  }
}
