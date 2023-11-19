use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  on_change: EventHandler<'a, FormEvent>,
  selected: &'a str,
}

#[allow(non_snake_case)]
pub fn LanguageSelect<'a>(cx: Scope<'a, Props<'a>>) -> Element {
  let selected_en: bool = "en".eq(cx.props.selected);
  let selected_es: bool = "es".eq(cx.props.selected);
  render! {
    select {
      onchange: move |event| cx.props.on_change.call(event),
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
