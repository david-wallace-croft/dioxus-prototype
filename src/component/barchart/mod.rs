use self::barchart_svg::BarchartSvg;
use ::dioxus::prelude::*;

mod barchart_svg;

static CSS: Asset = asset!("/public/barchart/app-barchart.css");

#[allow(non_snake_case)]
#[component]
pub fn Barchart() -> Element {
  rsx! {
    document::Stylesheet {
      href: CSS,
      rel: "stylesheet",
    }
    div {
      class: "app-barchart app-fade-in-animation box",
    h1 {
      style: "text-align: center",
    "Barchart"
    }
    BarchartSvg {}
    }
  }
}
