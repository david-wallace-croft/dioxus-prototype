use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaCompress;
use dioxus_free_icons::icons::fa_solid_icons::FaExpand;
use dioxus_free_icons::icons::fa_solid_icons::FaForwardStep;
use dioxus_free_icons::Icon;

#[derive(Props)]
pub struct Props<'a> {
  on_click_fullscreen: EventHandler<'a, MouseEvent>,
  on_click_skip: EventHandler<'a, MouseEvent>,
}

#[allow(non_snake_case)]
pub fn ControlPanel<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
  render! {
    div {
      text_align: "center",
    button {
      class: "app-fullscreen-button",
      onclick: move |event| cx.props.on_click_fullscreen.call(event),
      title: "Fullscreen",
    if web_sys::window().unwrap().document().unwrap().fullscreen_element().is_some() {
      render!{
    Icon {
      class: "app-skip-icon",
      icon: FaCompress,
    }
    }
    } else {
      render!{
    Icon {
      class: "app-fullscreen-icon",
      icon: FaExpand,
    }
    }
    }
    }
    button {
      class: "app-skip-button",
      onclick: move |event| cx.props.on_click_skip.call(event),
      title: "Skip",
    Icon {
      class: "app-skip-icon",
      icon: FaForwardStep,
    }
    }
    }
  }
}
