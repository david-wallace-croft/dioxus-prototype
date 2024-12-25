use ::dioxus::prelude::*;
use ::dioxus_free_icons::icons::fa_regular_icons::FaCircleQuestion;
use ::dioxus_free_icons::Icon;

#[allow(non_snake_case)]
#[component]
pub fn ShowButton(
  disabled: bool,
  on_click: EventHandler<MouseEvent>,
) -> Element {
  rsx! {
  button {
    class: "app-show-button",
    cursor: if disabled { "default" } else { "pointer" },
    disabled,
    onclick: move |event| on_click.call(event),
    opacity: if disabled { "0.5" } else { "1.0" },
    Icon {
      class: "app-show-icon",
      icon: FaCircleQuestion,
    }
    "SHOW"
  }
  }
}
