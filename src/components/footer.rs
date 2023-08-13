use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
      footer { class: "app-footer",
          "Copyright 2023 "
          a { href: "https://www.CroftSoft.com/", target: "_blank", "CroftSoft Inc" }
      }
  }
}
