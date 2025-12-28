use self::barchart_row::BarchartRow;
use super::barchart_data::BarchartData;
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

  rsx! {
    svg {
      onresize: move |cx| {
        // TODO: Scale everything based on SVG width

        let data = cx.data();

        let border_box_size = data.get_border_box_size();
        info!("border_box_size={:?}", border_box_size);

        let content_box_size = data.get_content_box_size();
        info!("content_box_size={:?}", content_box_size);
      },
    BarchartRow {
      amount: goal,
      fill: "rgb(56, 182, 255)",
      maximum,
      row_index: 0,
      s: &["GOAL"],
    }
    BarchartRow {
      amount: total_raised,
      fill: "rgb(92, 134, 214)",
      maximum,
      row_index: 1,
      s: &["TOTAL", "RAISED"],
    }
    BarchartRow {
      amount: corporate_pledges,
      fill: "rgb(103, 89, 162)",
      maximum,
      row_index: 2,
      s: &["CORPORATE", "PLEDGES"],
    }
    BarchartRow {
      amount: individual_donations,
      fill: "rgb(96, 45, 105)",
      maximum,
      row_index: 3,
      s: &["INDIVIDUAL", "DONATIONS"],
    }
    }
  }
}
