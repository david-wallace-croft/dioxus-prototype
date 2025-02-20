use ::dioxus::prelude::*;
use ::dioxus_free_icons::icons::fa_solid_icons::{
  FaCompress, FaExpand, FaForwardStep,
};
use ::dioxus_free_icons::Icon;

#[allow(non_snake_case)]
#[component]
pub fn ControlPanel(
  fading: bool,
  fullscreen: bool,
  on_click_fullscreen: EventHandler<MouseEvent>,
  on_click_skip: EventHandler<MouseEvent>,
) -> Element {
  rsx! {
    div {
      class: if fading {
        "app-control-panel app-fading"
        } else {
          "app-control-panel"
        },
      text_align: "center",
    button {
      class: "app-fullscreen-button",
      onclick: move |event| on_click_fullscreen.call(event),
      title: "Fullscreen",
    if fullscreen {
    Icon {
      class: "app-skip-icon",
      icon: FaCompress,
    }
    } else {
    Icon {
      class: "app-fullscreen-icon",
      icon: FaExpand,
    }
    }
    }
    button {
      class: "app-skip-button",
      onclick: move |event| on_click_skip.call(event),
      title: "Skip",
    Icon {
      class: "app-skip-icon",
      icon: FaForwardStep,
    }
    }
    }
  }
}
