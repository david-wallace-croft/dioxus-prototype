use dioxus::prelude::*;
use dioxus_std::i18n::{use_i18, UseI18};

#[allow(non_snake_case)]
pub fn LanguageSelect(cx: Scope) -> Element {
  let i18: UseI18 = use_i18(cx);
  render! {
    select {
      option {
        onclick: move |_| i18.set_language("en-US".parse().unwrap()),
        "English"
      }
      option {
        onclick: move |_| i18.set_language("es-ES".parse().unwrap()),
        "Español"
      }
    }
  }
}
