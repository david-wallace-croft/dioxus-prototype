use ::dioxus::prelude::*;

pub static EN: &'static str = "en";
pub static ES: &'static str = "es";

pub static LANGUAGE: GlobalSignal<&'static str> = Signal::global(|| EN);

#[allow(non_snake_case)]
#[component]
pub fn LanguageSelector() -> Element {
  let selected_en: bool = EN.eq(LANGUAGE());
  let selected_es: bool = ES.eq(LANGUAGE());
  rsx! {
    div {
      class: "app-language-selector",
    select {
      onchange: move |event: FormEvent| {
        *LANGUAGE.write() = if event.value().eq(ES) { ES } else { EN };
      },
      option {
        label: "English",
        selected: selected_en,
        value: EN,
      }
      option {
        label: "Español",
        selected: selected_es,
        value: ES,
      }
    }
  }
  }
}
