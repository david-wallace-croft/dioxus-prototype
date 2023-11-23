use crate::components::translator::SharedState;
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn LanguageSelector(cx: Scope) -> Element {
  let use_shared_state_option = use_shared_state::<SharedState>(cx);
  let shared_state_lang: String = match use_shared_state_option {
    Some(use_shared_state) => use_shared_state.read().lang.clone(),
    None => "en".to_string(),
  };
  let selected_en: bool = "en".eq(&shared_state_lang);
  let selected_es: bool = "es".eq(&shared_state_lang);
  render! {
    div {
      class: "app-language-selector",
    select {
      onchange: move |event: FormEvent| {
        if let Some(use_shared_state) = use_shared_state_option {
          *use_shared_state.write() = SharedState { lang: event.value.clone() };
        }
      },
      option {
        label: "English",
        selected: selected_en,
        value: "en",
      }
      option {
        label: "Español",
        selected: selected_es,
        value: "es",
      }
    }
  }
  }
}
