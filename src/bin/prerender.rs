use ::dioxus_fullstack::prelude::*;
use ::dioxus_fullstack::router::FullstackRouterConfig;
use ::std::path::PathBuf;
use dioxus_prototype::route::Route;

const STATIC_DIR: &str = "dist";

#[tokio::main]
async fn main() {
  let incremental_renderer_config = IncrementalRendererConfig::default()
    .map_path(map_path)
    .static_dir(STATIC_DIR);
  let fullstack_router_config = FullstackRouterConfig::<Route>::default();
  let serve_config: ServeConfig<FullstackRouterConfig<Route>> =
    ServeConfigBuilder::new_with_router(fullstack_router_config)
      .assets_path(STATIC_DIR)
      .incremental(incremental_renderer_config)
      .build();
  pre_cache_static_routes_with_props(&serve_config)
    .await
    .unwrap();
}

fn map_path(route: &str) -> PathBuf {
  println!("route: {}", route);
  let query_index_option: Option<usize> = route.find('?');
  let route2: String = match query_index_option {
    Some(query_index) => route[..query_index].to_owned(),
    None => route.to_owned(),
  };
  let mut path = PathBuf::from(STATIC_DIR);
  for segment in route2.split_terminator('/') {
    println!("segment: {}", segment);
    path.push(segment);
  }
  println!("path: {}", path.display());
  path
}
