use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
    footer {
      class: "box",
      dangerous_inner_html: "&copy;",
    " 2023-2024 "
    a {
      href: "https://www.CroftSoft.com/",
      target: "_blank",
      "CroftSoft Inc",
    }
    }
  }
}
