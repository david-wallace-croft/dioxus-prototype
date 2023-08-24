use dioxus::prelude::*;
// https://docs.rs/dioxus-hooks/latest/dioxus_hooks/
// use dioxus::hooks::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/useeffect.rs
  use_effect(cx, (), |()| async {
    log::info!("logging test");
  });
  render! {
    div {
      class: "box",
    h1 {
      class: "app-home",
      "CroftSoft Dioxus Prototype"
    }
    p {
      class: "app-home",
      "This is the home page."
    }
    canvas {
      height: "600",
      style: "cursor: crosshair",
      width: "600",
    }
    }
  }
}
