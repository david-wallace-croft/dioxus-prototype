use std::sync::LazyLock;

use self::barchart_row::BarchartRow;
use self::barchart_row_data::BarchartRowData;
use ::dioxus::html::geometry::Pixels;
use ::dioxus::html::geometry::euclid::Size2D;
use ::dioxus::prelude::*;

mod barchart_row;
mod barchart_row_data;

const AMOUNT_GOAL: f64 = 50_000.;

const AMOUNT_CORPORATE_PLEDGES: f64 = 17_000.;

const AMOUNT_INDIVIDUAL_DONATIONS: f64 = 15_677.49;

const AMOUNT_TOTAL_RAISED: f64 =
  AMOUNT_CORPORATE_PLEDGES + AMOUNT_INDIVIDUAL_DONATIONS;

const BARCHART_ROW_STATIC_DATA: [BarchartRowData; 4] = [
  BarchartRowData {
    amount: AMOUNT_GOAL,
    fill: "rgb(56, 182, 255)",
    s: &["GOAL"],
  },
  BarchartRowData {
    amount: AMOUNT_TOTAL_RAISED,
    fill: "rgb(92, 134, 214)",
    s: &[
      "TOTAL", "RAISED",
    ],
  },
  BarchartRowData {
    amount: AMOUNT_CORPORATE_PLEDGES,
    fill: "rgb(103, 89, 162)",
    s: &[
      "CORPORATE",
      "PLEDGES",
    ],
  },
  BarchartRowData {
    amount: AMOUNT_INDIVIDUAL_DONATIONS,
    fill: "rgb(96, 45, 105)",
    s: &[
      "INDIVIDUAL",
      "DONATIONS",
    ],
  },
];

const AMOUNT_MAXIMUM: LazyLock<f64> = LazyLock::new(|| {
  BARCHART_ROW_STATIC_DATA
    .into_iter()
    .max_by(|x: &BarchartRowData, y: &BarchartRowData| {
      x.amount.total_cmp(&y.amount)
    })
    .unwrap()
    .amount
});

#[allow(non_snake_case)]
#[component]
pub fn BarchartSvg() -> Element {
  let mut font_size_signal: Signal<f64> = use_signal(|| 19.);

  let font_size: f64 = *font_size_signal.read();

  let height: f64 = font_size * 9.4;

  let amount_maximum: f64 = *AMOUNT_MAXIMUM;

  rsx! {
    svg {
      height,
      onresize: move |cx| {
        let data = cx.data();

        let content_box_size: Result<Size2D<f64, Pixels>, ResizeError>
          = data.get_content_box_size();

        let Ok(size2d) = content_box_size else { return; };

        font_size_signal.set(size2d.width / 25.);
      },
      for (row_index, data) in BARCHART_ROW_STATIC_DATA.iter().enumerate() {
        BarchartRow {
          amount: data.amount,
          amount_maximum,
          fill: data.fill,
          font_size,
          row_index,
          s: data.s,
        }
      }
    }
  }
}
