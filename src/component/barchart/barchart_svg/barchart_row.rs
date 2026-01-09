use ::com_croftsoft_lib_string::to_dollars;
use ::dioxus::prelude::*;

static TEXT_ANCHOR_END: &str = "end";

static TEXT_ANCHOR_START: &str = "start";

static TEXT_FILL: &str = "black";

#[component]
pub fn BarchartRow(
  amount: f64,
  fill: String,
  font_size_signal: Signal<f64>,
  maximum: f64,
  row_index: usize,
  s: &'static [&'static str],
) -> Element {
  let font_size: f64 = *font_size_signal.read();

  let bar_height: f64 = 2. * font_size;

  let bar_width_goal: f64 = font_size * 14.;

  let text_x: f64 = font_size * 6.5;

  let bar_x: f64 = text_x + font_size / 4.;

  let margin_x: f64 = font_size / 5.;

  let margin_y: f64 = font_size / 2.;

  let row_height: f64 = bar_height + margin_y;

  let text_height: f64 = font_size;

  let text_offset_y0: f64 = (bar_height + text_height) * 0.44;

  let text_offset_y1: f64 = text_height * 0.85;

  let text_offset_y2: f64 = text_offset_y1 + text_height;

  let bar_width: f64 = bar_width_goal * amount / maximum;

  let dollars: String = to_dollars(amount);

  let row_index_f64: f64 = row_index as f64;

  rsx! {
    if s.len() > 1 {
      text {
        x: text_x,
        y: row_index_f64 * row_height + text_offset_y1,
        font_size: font_size as f64,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
      "{s[0]}"
      }
      text {
        x: text_x,
        y: row_index_f64 * row_height + text_offset_y2,
        font_size,
        text_anchor: TEXT_ANCHOR_END,
        fill: TEXT_FILL,
        "{s[1]}"
      }
    } else {
      text {
        x: text_x,
        y: row_index_f64 * row_height + text_offset_y0,
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
      y: row_index_f64 * row_height,
      fill,
    }
    text {
      x: bar_x + bar_width + margin_x,
      y: row_index_f64 * row_height + text_offset_y0,
      font_size,
      text_anchor: TEXT_ANCHOR_START,
      fill: TEXT_FILL,
    "{dollars}"
    }
  }
}
