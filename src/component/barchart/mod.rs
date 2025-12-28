use self::barchart_data::BarchartData;
use self::barchart_svg::BarchartSvg;
use ::dioxus::prelude::*;

mod barchart_data;
mod barchart_svg;

const BARCHART_DATA: BarchartData = BarchartData {
  corporate_pledges: 17_000.,
  goal: 50_000.,
  individual_donations: 15_677.49,
};

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
    BarchartSvg {
      barchart_data: BARCHART_DATA,
    }
    }
  }
}
