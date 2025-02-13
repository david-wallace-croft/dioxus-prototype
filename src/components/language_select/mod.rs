use ::dioxus::prelude::*;

pub static LANGUAGE: GlobalSignal<String> = Signal::global(|| "en".to_string());

#[allow(non_snake_case)]
#[component]
pub fn LanguageSelector() -> Element {
  let selected_en: bool = "en".eq(LANGUAGE().as_str());
  let selected_es: bool = "es".eq(LANGUAGE().as_str());
  rsx! {
    div {
      class: "app-language-selector",
    select {
      onchange: move |event: FormEvent| {
        *LANGUAGE.write() = event.value();
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
