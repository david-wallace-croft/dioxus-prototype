use super::language_select::{ES, LANGUAGE};
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Translator(
  en: &'static str,
  es: &'static str,
) -> Element {
  if ES.eq(LANGUAGE()) {
    rsx! { "{es}" }
  } else {
    rsx! { "{en}" }
  }
}
