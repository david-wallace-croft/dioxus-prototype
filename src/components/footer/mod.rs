use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Footer() -> Element {
  rsx! {
    footer {
      class: "box",
      dangerous_inner_html: "&copy;",
    " 2023-2025 "
    a {
      href: "https://www.CroftSoft.com/",
      target: "_blank",
      "CroftSoft Inc",
    }
    }
  }
}
