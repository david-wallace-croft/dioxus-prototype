use ::web_sys::js_sys::Math::random;

#[derive(Clone, Copy)]
pub struct Color {
  pub blue: u8,
  pub green: u8,
  pub red: u8,
}

impl Color {
  pub fn as_fill_style_string(&self) -> String {
    let &Color {
      blue,
      green,
      red,
    } = self;

    format!("rgb({red}, {green}, {blue})")
  }

  pub fn drift(
    &mut self,
    maximum_drift: u8,
  ) {
    self.blue = drift_primary_color(self.blue, maximum_drift);
    self.green = drift_primary_color(self.green, maximum_drift);
    self.red = drift_primary_color(self.red, maximum_drift);
  }

  pub fn generate_random_color() -> Self {
    let red: u8 = (random() * 256.) as u8;

    let green: u8 = (random() * 256.) as u8;

    let blue: u8 = (random() * 256.) as u8;

    Self {
      blue,
      green,
      red,
    }
  }
}

fn drift_primary_color(
  primary_color: u8,
  maximum_drift: u8,
) -> u8 {
  let drift: i16 = generate_random_drift(maximum_drift);

  let primary_color_plus_drift: i16 = primary_color as i16 + drift;

  primary_color_plus_drift.clamp(0, 255) as u8
}

/// Generates a random integer between -maximum_drift and +maximum_drift
fn generate_random_drift(maximum_drift: u8) -> i16 {
  let maximum_drift_as_i16: i16 = maximum_drift as i16;

  // if maximum is 0, width is 1
  // if maximum is 1, width is 3
  // if maximum is 2, width is 5
  // if maximum is 3, width is 7
  // ...
  // if maximum is 254, width is 509
  // if maximum is 255, width is 511
  let range_width: i16 = 2 * maximum_drift_as_i16 + 1;

  // if maximum is 0, 0.0 <= drift < 1.0
  // if maximum is 1, 0.0 <= drift < 3.0
  // if maximum is 2, 0.0 <= drift < 5.0
  // if maximum is 3, 0.0 <= drift < 7.0
  // ...
  // if maximum is 254, 0.0 <= drift < 509.0
  // if maximum is 255, 0.0 <= drift < 511.0
  let shifted_drift_as_f64: f64 = random() * (range_width as f64);

  // if maximum is 0, drift is 0
  // if maximum is 1, drift is 0, 1, or 2
  // if maximum is 2, drift is 0, 1, 2, 3, or 4
  // if maximum is 3, drift is 0, 1, 2, 3, 4, 5, or 6
  // ...
  // if maximum is 254, drift is 0, 1, 2, ..., 507, or 508
  // if maximum is 255, drift is 0, 1, 2, ..., 509, or 510
  let shifted_drift_as_i16: i16 = shifted_drift_as_f64 as i16;

  // if maximum is 0, drift is 0
  // if maximum is 1, drift is -1, 0, or 1
  // if maximum is 2, drift is -2, -1, 0, 1, 2, or 2
  // if maximum is 3, drift is -3, -2, -1, 0, 1, 2, or 3
  // ...
  // if maximum is 254, drift is -254, -253, ..., 253, or 254
  // if maximum is 255, drift is -255, -254, ..., 254, or 255
  shifted_drift_as_i16 - maximum_drift_as_i16
}
