use dioxus_prototype::route::Route;
use dioxus_router::prelude::*;
use dioxus_ssr::incremental::{DefaultRenderer, IncrementalRendererConfig};

#[tokio::main]
async fn main() {
  let mut renderer = IncrementalRendererConfig::new()
    .static_dir("./static")
    .build();

  println!(
    "SITE MAP:\n{}",
    Route::SITE_MAP
      .iter()
      .flat_map(|route| route.flatten().into_iter())
      .map(|route| {
        route
          .iter()
          .map(|segment| segment.to_string())
          .collect::<Vec<_>>()
          .join("")
      })
      .collect::<Vec<_>>()
      .join("\n")
  );

  pre_cache_static_routes::<Route, _>(
    &mut renderer,
    &DefaultRenderer {
      before_body: r#"<!DOCTYPE html>
        <html
          lang="en">
        <head>
          <meta
            charset="UTF-8">
          <title>
            CroftSoft Dioxus Prototype
          </title>
          <link
            href="/stylesheet.css"
            rel="stylesheet">
        </head>
        <body>"#
        .to_string(),
      after_body: r#"</body>
        </html>"#
        .to_string(),
    },
  )
  .await
  .unwrap();
}
