use ::com_croftsoft_lib_string::to_dollars;
use ::dioxus::prelude::*;
use ::dioxus_charts::BarChart;
// use ::dioxus_elements::optgroup::label;

#[allow(non_snake_case)]
#[component]
pub fn Barchart() -> Element {
  static CSS: Asset = asset!("/assets/barchart/app-barchart.css");

  let labels: Vec<String> = vec![
    "GOAL".into(),
    "TOTAL RAISED".into(),
    "CORPORATE PLEDGES".into(),
    "INDIVIDUAL DONATIONS".into(),
  ];

  fn label_interpolation(v: f32) -> String {
    to_dollars(v as f64)
  }

  let label_interpolation_option: Option<fn(f32) -> String> =
    Some(label_interpolation);

  rsx! {
  document::Stylesheet {
    href: CSS
  }
  div {
    class: "app-barchart app-fade-in-animation box",
  h1 {
    style: "text-align: center",
  "Barchart"
  }
  BarChart {
    // bar_distance: 200.,
    bar_width: "10%",
    // height: "50%",
    highest: 50_000.,
    horizontal_bars: true,
    label_interpolation: label_interpolation_option,
    label_size: 120,
    labels: labels,
    max_ticks: 0,
    // padding_bottom: 10,
    padding_left: 130,
    padding_right: 75,
    // padding_top: 0,
    series: vec![
      vec![50_000., 23_483.78, 11_000., 12_483.78],
    ],
    show_dotted_grid: false,
    show_grid: false,
    show_grid_ticks: false,
    show_labels: true,
    show_series_labels: true,
    viewbox_height: 200,
    // viewbox_width: 1200,
    // width: "80%",
  }
  }
  }
}
