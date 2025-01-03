use ::rand::distributions::Uniform;
use ::rand::prelude::*;
use ::std::ops::RangeInclusive;

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

  pub fn drift(&self) -> Color {
    Color {
      blue: Color::drift_primary_color(self.blue),
      green: Color::drift_primary_color(self.green),
      red: Color::drift_primary_color(self.red),
    }
  }

  pub fn drift_primary_color(primary_color: u8) -> u8 {
    let range: RangeInclusive<i8> = -6..=6;

    let die: Uniform<i8> = Uniform::from(range);

    let mut rng: ThreadRng = rand::thread_rng();

    let delta: i8 = die.sample(&mut rng);

    primary_color.saturating_add_signed(delta)
  }

  pub fn generate_random_color() -> Color {
    let mut rng: ThreadRng = rand::thread_rng();

    let d256: Uniform<u8> = Uniform::from(0..=255);

    let red: u8 = d256.sample(&mut rng);

    let green: u8 = d256.sample(&mut rng);

    let blue: u8 = d256.sample(&mut rng);

    Color {
      blue,
      green,
      red,
    }
  }

  pub fn shift(
    &self,
    delta: i8,
  ) -> Color {
    Color {
      blue: Color::shift_primary_color(self.blue, delta),
      green: Color::shift_primary_color(self.green, delta),
      red: Color::shift_primary_color(self.red, delta),
    }
  }

  pub fn shift_primary_color(
    primary_color: u8,
    delta: i8,
  ) -> u8 {
    primary_color.saturating_add_signed(delta)
  }
}
