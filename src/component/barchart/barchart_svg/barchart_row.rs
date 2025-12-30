use ::com_croftsoft_lib_string::to_dollars;
use ::dioxus::prelude::*;

static TEXT_ANCHOR_END: &str = "end";

static TEXT_ANCHOR_START: &str = "start";

static TEXT_FILL: &str = "black";

#[component]
pub fn BarchartRow(
  amount: f64,
  fill: String,
  font_size_signal: Signal<usize>,
  maximum: f64,
  row_index: usize,
  s: &'static [&'static str],
) -> Element {
  let font_size: usize = *font_size_signal.read();

  let bar_height: usize = 2 * font_size;

  let bar_width_goal: usize = font_size * 14;

  let text_x: usize = font_size * 650 / 100;

  let bar_x: usize = text_x + font_size / 4;

  let margin_x: usize = font_size / 5;

  let margin_y: usize = font_size / 2;

  let row_height: usize = bar_height + margin_y;

  let text_height: usize = font_size;

  let text_offset_y0: usize = (bar_height + text_height) * 44 / 100;

  let text_offset_y1: usize = text_height * 85 / 100;

  let text_offset_y2: usize = text_offset_y1 + text_height;

  let bar_width: usize = (bar_width_goal as f64 * amount / maximum) as usize;

  let dollars: String = to_dollars(amount);

  rsx! {
    if s.len() > 1 {
      text {
        x: text_x,
        y: row_index * row_height + text_offset_y1,
        font_size,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
      "{s[0]}"
      }
      text {
        x: text_x,
        y: row_index * row_height + text_offset_y2,
        font_size,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
        "{s[1]}"
      }
    } else {
      text {
        x: text_x,
        y: row_index * row_height + text_offset_y0,
        font_size,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
      "{s[0]}"
      }
    }
    rect {
      height: bar_height,
      width: bar_width,
      x: bar_x,
      y: row_index * row_height,
      fill,
    }
    text {
      x: bar_x + bar_width + margin_x,
      y: row_index * row_height + text_offset_y0,
      font_size,
      text_anchor: TEXT_ANCHOR_START,
      fill: TEXT_FILL,
    "{dollars}"
    }
  }
}
