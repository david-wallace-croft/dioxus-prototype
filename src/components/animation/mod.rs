use self::color::Color;
use ::dioxus::html::geometry::WheelDelta::{self, Lines, Pages, Pixels};
use ::dioxus::prelude::*;
use ::std::time::Duration;
use ::tracing::info;
use ::web_sys::wasm_bindgen::JsCast;
use ::web_sys::{
  window, CanvasRenderingContext2d, Document, HtmlCanvasElement, Window,
};

mod color;

const CANVAS_ID: &str = "home-page-canvas";

const MESSAGE_CONTROLS: &str = "Hold a key or scroll the mouse wheel";

const MESSAGE_START: &str = "Click on or tab to the canvas";

#[allow(non_snake_case)]
#[component]
pub fn Animation() -> Element {
  static CSS: Asset = asset!("/assets/app-animation.css");

  let mut click_count_signal: Signal<i32> = use_signal(|| 0);

  let mut color_signal: Signal<Color> =
    use_signal(Color::generate_random_color);

  let mut message_signal: Signal<&str> = use_signal(|| MESSAGE_START);

  let mut running_signal: Signal<bool> = use_signal(|| true);

  let mut update_signal: Signal<bool> = use_signal(|| true);

  // https://github.com/DioxusLabs/dioxus/discussions/999
  // https://github.com/DioxusLabs/dioxus/blob/master/packages/hooks/src/use_effect.rs
  use_future(move || {
    to_owned![color_signal];
    to_owned![message_signal];
    to_owned![running_signal];
    to_owned![update_signal];
    async move {
      let window: Window = window().expect("global window does not exists");

      let document: Document =
        window.document().expect("expecting a document on window");

      let html_canvas_element = document
        .get_element_by_id(CANVAS_ID)
        .expect("expecting a canvas in the document")
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

      let canvas_context: CanvasRenderingContext2d = html_canvas_element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

      let canvas_height: f64 = html_canvas_element.height() as f64;

      let canvas_width: f64 = html_canvas_element.width() as f64;

      let square_size: f64 =
        100.0_f64.min(canvas_width / 2.).min(canvas_height / 2.);

      let mut delta_x: f64 = 1.;

      let mut delta_y: f64 = 1.;

      let mut x: f64 = -delta_x;

      let mut y: f64 = -delta_y;

      loop {
        if *running_signal.read() || *update_signal.read() {
          update_signal.set(false);

          if delta_x > 0. {
            if x + delta_x + square_size > canvas_width {
              delta_x = -delta_x;
            }
          } else if x + delta_x < 0. {
            delta_x = -delta_x;
          }

          x += delta_x;

          if delta_y > 0. {
            if y + delta_y + square_size > canvas_height {
              delta_y = -delta_y;
            }
          } else if y + delta_y < 0. {
            delta_y = -delta_y;
          }

          y += delta_y;

          let color: Color = *color_signal.read();

          let drift_color: Color = color.drift();

          color_signal.set(drift_color);

          let fill_style: String = color_signal.read().as_fill_style_string();

          paint(
            &canvas_context,
            canvas_height,
            canvas_width,
            &fill_style,
            *message_signal.read(),
            square_size,
            x,
            y,
          );
        }

        async_std::task::sleep(Duration::from_millis(17u64)).await;
      }
    }
  });
  rsx! {
    document::Stylesheet {
      href: CSS
    }
    div {
      class: "app-animation box",
    h1 {
      "Animation"
    }
    canvas {
      background_color: "black",
      cursor: "crosshair",
      id: CANVAS_ID,
      // https://docs.rs/dioxus/latest/dioxus/events/index.html
      onblur: move |event| on_blur(event, &mut message_signal, &mut running_signal),
      onclick: move |event| on_click(event, &mut click_count_signal, &mut color_signal, &mut update_signal),
      onfocus: move |event| on_focus(event, &mut message_signal, &mut running_signal, &mut update_signal),
      onkeydown: move |event| on_key_down(event, &mut color_signal, &mut update_signal),
      onwheel: move |event| on_wheel(event, &mut color_signal, &mut update_signal),
      tabindex: 0,
      width: "600",
    }
    }
  }
}

fn on_blur(
  _event: Event<FocusData>,
  message_signal: &mut Signal<&str>,
  running_signal: &mut Signal<bool>,
) {
  // log::info!("onblur Event: {event:?}");

  message_signal.set(MESSAGE_START);

  running_signal.set(true);
}

fn on_click(
  _event: Event<MouseData>,
  click_count_signal: &mut Signal<i32>,
  _color_signal: &mut Signal<Color>,
  _update_signal: &mut Signal<bool>,
) {
  // log::info!("onclick Event: {event:?}");

  let click_count: i32 = *click_count_signal.read();

  click_count_signal.set(click_count + 1);

  let current_value: i32 = *click_count_signal.read();

  info!("click count: {current_value:?}");

  // color_signal.set(generate_random_color());

  // update_signal.set(true);
}

fn on_focus(
  _event: Event<FocusData>,
  message_signal: &mut Signal<&str>,
  running_signal: &mut Signal<bool>,
  update_signal: &mut Signal<bool>,
) {
  // log::info!("onfocus Event: {event:?}");

  message_signal.set(MESSAGE_CONTROLS);

  running_signal.set(false);

  update_signal.set(true);
}

fn on_key_down(
  _event: Event<KeyboardData>,
  color_signal: &mut Signal<Color>,
  update_signal: &mut Signal<bool>,
) {
  // log::info!("onkeydown Event: {event:?}");

  let color: Color = *color_signal.read();

  let drift_color: Color = color.drift();

  color_signal.set(drift_color);

  update_signal.set(true);
}

fn on_wheel(
  event: Event<WheelData>,
  color_signal: &mut Signal<Color>,
  update_signal: &mut Signal<bool>,
) {
  // log::info!("onwheel Event: {event:?}");

  let wheel_delta: WheelDelta = event.delta();

  let delta: f64 = match wheel_delta {
    Lines(lines_vector) => lines_vector.y,
    Pages(pages_vector) => pages_vector.y,
    Pixels(pixels_vector) => pixels_vector.y,
  };

  let delta: i8 = delta.clamp(-1., 1.) as i8;

  let color: Color = *color_signal.read();

  color_signal.set(color.shift(delta));

  update_signal.set(true);
}

fn paint(
  canvas_context: &CanvasRenderingContext2d,
  canvas_height: f64,
  canvas_width: f64,
  fill_style: &str,
  message: &str,
  square_size: f64,
  x: f64,
  y: f64,
) {
  canvas_context.set_fill_style_str("black");

  canvas_context.fill_rect(0., 0., canvas_width, canvas_height);

  canvas_context.set_fill_style_str(fill_style);

  canvas_context.fill_rect(x, y, square_size, square_size);

  canvas_context.set_font("30px Verdana");

  canvas_context.set_fill_style_str("white");

  let _ = canvas_context.fill_text(message, 4., 30.);
}
