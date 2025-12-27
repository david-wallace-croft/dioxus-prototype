use self::barchart_row::BarchartRow;
use ::com_croftsoft_lib_string::to_dollars;
use ::dioxus::prelude::*;
use ::tracing::info;

mod barchart_row;

const BAR_HEIGHT: usize = 2 * FONT_SIZE;

const BAR_WIDTH_GOAL: usize = FONT_SIZE * 15;

const BAR_WIDTH_TOTAL_RAISED: usize =
  (BAR_WIDTH_GOAL as f64 * DOLLARS_TOTAL_RAISED / DOLLARS_GOAL) as usize;

const BAR_WIDTH_CORPORATE_PLEDGES: usize =
  (BAR_WIDTH_GOAL as f64 * DOLLARS_CORPORATE_PLEDGES / DOLLARS_GOAL) as usize;

const BAR_WIDTH_INDIVIDUAL_DONATIONS: usize = (BAR_WIDTH_GOAL as f64
  * DOLLARS_INDIVIDUAL_DONATIONS
  / DOLLARS_GOAL) as usize;

const BAR_X: usize = TEXT_X + FONT_SIZE / 4;

const DOLLARS_GOAL: f64 = 50_000.;

const DOLLARS_CORPORATE_PLEDGES: f64 = 17_000.;

const DOLLARS_INDIVIDUAL_DONATIONS: f64 = 15_677.49;

const DOLLARS_TOTAL_RAISED: f64 =
  DOLLARS_CORPORATE_PLEDGES + DOLLARS_INDIVIDUAL_DONATIONS;

// TODO: Convert everything to f32 or f64
const FONT_SIZE: usize = 20;

const MARGIN_X: usize = FONT_SIZE / 5;

const MARGIN_Y: usize = FONT_SIZE / 2;

const SVG_HEIGHT: usize = 4 * BAR_HEIGHT + 3 * MARGIN_Y;

const SVG_WIDTH: usize =
  BAR_X + BAR_WIDTH_GOAL + MARGIN_X + FONT_SIZE * 390 / 100;

const TEXT_X: usize = FONT_SIZE * 605 / 100;

#[allow(non_snake_case)]
#[component]
pub fn BarchartSvg() -> Element {
  rsx! {
    svg {
      height: SVG_HEIGHT,
      width: SVG_WIDTH,
      // TODO: Scale everything based on SVG width
      onresize: move |cx| {
        let data = cx.data();

        let border_box_size = data.get_border_box_size();
        info!("border_box_size={:?}", border_box_size);

        let content_box_size = data.get_content_box_size();
        info!("content_box_size={:?}", content_box_size);
      },
    BarchartRow {
      amount: to_dollars(DOLLARS_GOAL),
      bar_width: BAR_WIDTH_GOAL,
      fill: "rgb(56, 182, 255)",
      row_index: 0,
      s: &["GOAL"],
    }
    BarchartRow {
      amount: to_dollars(DOLLARS_TOTAL_RAISED),
      bar_width: BAR_WIDTH_TOTAL_RAISED,
      fill: "rgb(92, 134, 214)",
      row_index: 1,
      s: &["TOTAL", "RAISED"],
    }
    BarchartRow {
      amount: to_dollars(DOLLARS_CORPORATE_PLEDGES),
      bar_width: BAR_WIDTH_CORPORATE_PLEDGES,
      fill: "rgb(103, 89, 162)",
      row_index: 2,
      s: &["CORPORATE", "PLEDGES"],
    }
    BarchartRow {
      amount: to_dollars(DOLLARS_INDIVIDUAL_DONATIONS),
      bar_width: BAR_WIDTH_INDIVIDUAL_DONATIONS,
      fill: "rgb(96, 45, 105)",
      row_index: 3,
      s: &["INDIVIDUAL", "DONATIONS"],
    }
    }
  }
}
