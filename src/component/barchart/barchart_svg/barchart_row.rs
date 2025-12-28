use ::com_croftsoft_lib_string::to_dollars;
use ::dioxus::prelude::*;

const BAR_HEIGHT: usize = 2 * FONT_SIZE;

const BAR_WIDTH_GOAL: usize = FONT_SIZE * 15;

const BAR_X: usize = TEXT_X + FONT_SIZE / 4;

const FONT_SIZE: usize = 19;

const MARGIN_X: usize = FONT_SIZE / 5;

const MARGIN_Y: usize = FONT_SIZE / 2;

const ROW_HEIGHT: usize = BAR_HEIGHT + MARGIN_Y;

static TEXT_ANCHOR_END: &str = "end";

static TEXT_ANCHOR_START: &str = "start";

static TEXT_FILL: &str = "black";

const TEXT_HEIGHT: usize = FONT_SIZE;

const TEXT_OFFSET_Y0: usize = (BAR_HEIGHT + TEXT_HEIGHT) * 44 / 100;

const TEXT_OFFSET_Y1: usize = TEXT_HEIGHT * 85 / 100;

const TEXT_OFFSET_Y2: usize = TEXT_OFFSET_Y1 + TEXT_HEIGHT;

const TEXT_X: usize = FONT_SIZE * 605 / 100;

#[component]
pub fn BarchartRow(
  amount: f64,
  fill: String,
  maximum: f64,
  row_index: usize,
  s: &'static [&'static str],
) -> Element {
  let bar_width: usize = (BAR_WIDTH_GOAL as f64 * amount / maximum) as usize;

  let dollars: String = to_dollars(amount);

  rsx! {
    if s.len() > 1 {
      text {
        x: TEXT_X,
        y: row_index * ROW_HEIGHT + TEXT_OFFSET_Y1,
        font_size: FONT_SIZE,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
      "{s[0]}"
      }
      text {
        x: TEXT_X,
        y: row_index * ROW_HEIGHT + TEXT_OFFSET_Y2,
        font_size: FONT_SIZE,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
        "{s[1]}"
      }
    } else {
      text {
        x: TEXT_X,
        y: row_index * ROW_HEIGHT + TEXT_OFFSET_Y0,
        font_size: FONT_SIZE,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
      "{s[0]}"
      }
    }
    rect {
      height: BAR_HEIGHT,
      width: bar_width,
      x: BAR_X,
      y: row_index * ROW_HEIGHT,
      fill,
    }
    text {
      x: BAR_X + bar_width + MARGIN_X,
      y: row_index * ROW_HEIGHT + TEXT_OFFSET_Y0,
      font_size: FONT_SIZE,
      text_anchor: TEXT_ANCHOR_START,
      fill: TEXT_FILL,
    "{dollars}"
    }
  }
}
