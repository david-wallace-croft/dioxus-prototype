use ::dioxus::prelude::*;

pub const CONTROL_PANEL_DISPLAY_TIME: u64 = 10 * 1_000;

pub const CONTROL_PANEL_FADE_TIME: u64 = 5 * 1_000;

pub static CSS: Asset = asset!("/assets/slideshow/app-slideshow.css");

pub static IMAGE_ASSETS: [Asset; 5] = [
  asset!("/assets/slideshow/nature-a.jpg"),
  asset!("/assets/slideshow/nature-b.jpg"),
  asset!("/assets/slideshow/nature-c.jpg"),
  asset!("/assets/slideshow/nature-d.jpg"),
  asset!("/assets/slideshow/nature-e.jpg"),
];

pub const IMAGE_DISPLAY_TIME: u64 = 15 * 1_000;

pub const POLLING_PERIOD: u64 = 100;
