use super::super::translator::Translator;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn ResetButton(
  disabled: bool,
  on_click: EventHandler<MouseEvent>,
) -> Element {
  rsx! {
  button {
    class: "app-reset",
    cursor: if disabled { "default" } else { "pointer" },
    disabled: disabled,
    onclick: move |event| on_click.call(event),
    opacity: if disabled { "0.5" } else { "1.0" },
    Translator {
      en: "RESET",
      es: "RESTABLECER",
    }
  }
  }
}
