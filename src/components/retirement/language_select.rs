use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  on_change: EventHandler<'a, FormEvent>,
}

#[allow(non_snake_case)]
pub fn LanguageSelect<'a>(cx: Scope<'a, Props<'a>>) -> Element {
  render! {
    select {
      onchange: move |event| cx.props.on_change.call(event),
      option {
        label: "English",
        value: "en",
      }
      option {
        label: "Español",
        value: "es",
      }
    }
  }
}
