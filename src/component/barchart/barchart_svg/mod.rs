use self::barchart_row::BarchartRow;
use super::barchart_data::BarchartData;
use ::dioxus::html::geometry::Pixels;
use ::dioxus::html::geometry::euclid::Size2D;
use ::dioxus::prelude::*;
use ::tracing::info;

mod barchart_row;

#[allow(non_snake_case)]
#[component]
pub fn BarchartSvg(barchart_data: BarchartData) -> Element {
  let BarchartData {
    corporate_pledges,
    goal,
    individual_donations,
  } = barchart_data;

  let total_raised: f64 = corporate_pledges + individual_donations;

  let maximum: f64 = goal.max(total_raised);

  let mut font_size_signal: Signal<usize> = use_signal(|| 19);

  rsx! {
    svg {
      onresize: move |cx| {
        let data = cx.data();

        let content_box_size: Result<Size2D<f64, Pixels>, ResizeError>
          = data.get_content_box_size();

        let Ok(size2d) = content_box_size else { return; };

        let Size2D { height, width, .. } = size2d;

        font_size_signal.set((width / 25.) as usize);

        info!("height: {height}, width: {width}, font_size: {font_size_signal}");
      },
    BarchartRow {
      amount: goal,
      fill: "rgb(56, 182, 255)",
      font_size_signal,
      maximum,
      row_index: 0,
      s: &["GOAL"],
    }
    BarchartRow {
      amount: total_raised,
      fill: "rgb(92, 134, 214)",
      font_size_signal,
      maximum,
      row_index: 1,
      s: &["TOTAL", "RAISED"],
    }
    BarchartRow {
      amount: corporate_pledges,
      fill: "rgb(103, 89, 162)",
      font_size_signal,
      maximum,
      row_index: 2,
      s: &["CORPORATE", "PLEDGES"],
    }
    BarchartRow {
      amount: individual_donations,
      fill: "rgb(96, 45, 105)",
      font_size_signal,
      maximum,
      row_index: 3,
      s: &["INDIVIDUAL", "DONATIONS"],
    }
    }
  }
}
