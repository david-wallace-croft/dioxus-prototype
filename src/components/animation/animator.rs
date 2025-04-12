use super::color::Color;
use ::com_croftsoft_lib_animation::web_sys::LoopUpdater;
use ::std::sync::Arc;
use ::std::sync::atomic::AtomicI8;
use ::std::sync::atomic::{AtomicBool, Ordering};
// use ::tracing::{debug, info};
use ::web_sys::wasm_bindgen::JsCast;
use ::web_sys::{
  CanvasRenderingContext2d, Document, HtmlCanvasElement, Window, window,
};

const MESSAGE_CONTROLS: &str = "Hold a key or scroll the mouse wheel";

const MESSAGE_START: &str = "Click on or tab to the canvas";

pub struct Animator {
  canvas_height: f64,
  canvas_rendering_context_2d: CanvasRenderingContext2d,
  canvas_width: f64,
  blur: Arc<AtomicBool>,
  color: Color,
  delta_x: f64,
  delta_y: f64,
  drift: Arc<AtomicI8>,
  focus: Arc<AtomicBool>,
  frame_count: usize,
  maximum_drift: u8,
  message: &'static str,
  running: bool,
  square_size: f64,
  stop: Arc<AtomicBool>,
  update: Arc<AtomicBool>,
  x: f64,
  y: f64,
}

impl Animator {
  pub fn new(
    blur: Arc<AtomicBool>,
    canvas_id: &str,
    drift: Arc<AtomicI8>,
    focus: Arc<AtomicBool>,
    stop: Arc<AtomicBool>,
    update: Arc<AtomicBool>,
  ) -> Self {
    let window: Window = window().expect("global window does not exists");

    let document: Document =
      window.document().expect("expecting a document on window");

    let html_canvas_element = document
      .get_element_by_id(canvas_id)
      .expect("expecting a canvas in the document")
      .dyn_into::<HtmlCanvasElement>()
      .unwrap();

    let canvas_rendering_context_2d: CanvasRenderingContext2d =
      html_canvas_element
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let canvas_height: f64 = html_canvas_element.height() as f64;

    let canvas_width: f64 = html_canvas_element.width() as f64;

    Self {
      blur,
      canvas_height,
      canvas_rendering_context_2d,
      canvas_width,
      color: Color::random(),
      delta_x: 1.,
      delta_y: 1.,
      drift,
      focus,
      frame_count: 0,
      maximum_drift: 0,
      message: MESSAGE_START,
      running: true,
      square_size: 100.0_f64.min(canvas_width / 2.).min(canvas_height / 2.),
      stop,
      update,
      x: -1.,
      y: -1.,
    }
  }

  fn paint(&self) {
    self.canvas_rendering_context_2d.set_fill_style_str("black");

    self.canvas_rendering_context_2d.fill_rect(
      0.,
      0.,
      self.canvas_width,
      self.canvas_height,
    );

    let fill_style: String = self.color.as_fill_style_string();

    self
      .canvas_rendering_context_2d
      .set_fill_style_str(&fill_style);

    self.canvas_rendering_context_2d.fill_rect(
      self.x,
      self.y,
      self.square_size,
      self.square_size,
    );

    self.canvas_rendering_context_2d.set_font("30px Verdana");

    self.canvas_rendering_context_2d.set_fill_style_str("white");

    let _ = self
      .canvas_rendering_context_2d
      .fill_text(self.message, 4., 30.);
  }

  fn adjust_maximum_drift(
    &mut self,
    delta: i8,
  ) {
    self.maximum_drift = self.maximum_drift.saturating_add_signed(delta);
  }

  fn set_message(
    &mut self,
    message: &'static str,
  ) {
    self.message = message;
  }

  fn update(&mut self) {
    if self.delta_x > 0. {
      if self.x + self.delta_x + self.square_size > self.canvas_width {
        self.delta_x = -self.delta_x;
      }
    } else if self.x + self.delta_x < 0. {
      self.delta_x = -self.delta_x;
    }

    self.x += self.delta_x;

    if self.delta_y > 0. {
      if self.y + self.delta_y + self.square_size > self.canvas_height {
        self.delta_y = -self.delta_y;
      }
    } else if self.y + self.delta_y < 0. {
      self.delta_y = -self.delta_y;
    }

    self.y += self.delta_y;

    if self.maximum_drift == 0 {
      self.maximum_drift = 1;
    }

    self.color.drift(self.maximum_drift);
  }
}

impl LoopUpdater for Animator {
  fn update_loop(
    &mut self,
    _update_time: f64,
  ) -> bool {
    // TODO: Use update_time

    // TODO: Maintain keypress state

    self.frame_count += 1;

    // TODO: Display frame_count
    // TODO: Display frames per second

    // info!("{update_time} {}", self.frame_count);

    let mut repaint = false;
    let mut update = false;

    if self.blur.load(Ordering::SeqCst) {
      // debug!("blur");

      self.blur.store(false, Ordering::SeqCst);

      self.set_message(MESSAGE_START);

      self.running = true;
    }

    let delta: i8 = self.drift.load(Ordering::SeqCst);

    if delta != 0 {
      // debug!("delta: {delta}");

      self.drift.store(0, Ordering::SeqCst);

      self.adjust_maximum_drift(delta);

      update = true;
    }

    if self.focus.load(Ordering::SeqCst) {
      self.focus.store(false, Ordering::SeqCst);

      self.set_message(MESSAGE_CONTROLS);

      repaint = true;

      self.running = false;
    }

    if self.update.load(Ordering::SeqCst) {
      // debug!("update requested");

      self.update.store(false, Ordering::SeqCst);

      update = true;
    }

    if self.running || update {
      // debug!("running: {running}, update: {update}");

      self.update();

      repaint = true;
    }

    if repaint {
      self.paint();
    }

    self.stop.load(Ordering::SeqCst)
  }
}
