use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
  disabled: bool,
  on_click: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn LinkButton<'a>(cx: Scope<'a, Props<'a>>) -> Element {
  render! {
  button {
    class: "app-link-button",
    cursor: if cx.props.disabled { "default" } else { "pointer" },
    disabled: cx.props.disabled,
    onclick: move |event| cx.props.on_click.call(event),
    opacity: if cx.props.disabled { "0.5" } else { "1.0" },
    "LINK"
  }
  }
}
