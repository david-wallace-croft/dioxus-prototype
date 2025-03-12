use ::dioxus::prelude::*;
use ::dioxus_free_icons::Icon;
use ::dioxus_free_icons::icons::fa_solid_icons::FaLink;

#[allow(non_snake_case)]
#[component]
pub fn LinkButton(
  disabled: bool,
  on_click: EventHandler<MouseEvent>,
) -> Element {
  rsx! {
  button {
    class: "app-link-button",
    cursor: if disabled { "default" } else { "pointer" },
    disabled: disabled,
    onclick: move |event| on_click.call(event),
    opacity: if disabled { "0.5" } else { "1.0" },
  Icon {
    class: "app-link-icon",
    icon: FaLink,
  }
  "LINK"
  }
  }
}
