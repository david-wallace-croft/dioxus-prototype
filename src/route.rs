use crate::components::blog::Blog;
use crate::components::high_five::HighFive;
use crate::components::home::Home;
use crate::components::page_not_found::PageNotFound;
use crate::components::page_template::PageTemplate;
use crate::components::story_listing::StoryListing;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
  #[layout(PageTemplate)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/high-five")]
    HighFive {},
    #[route("/story-listing")]
    StoryListing {},
        // #[nest("/blog")]
        //     #[layout(Blog)]
        //         #[route("/")]
        //         BlogList {},
        //         #[route("/blog/:name")]
        //         BlogPost { name: String },
        //     #[end_layout]
        // #[end_nest]
    #[end_layout]
    // #[nest("/myblog")]
    //     #[redirect("/", || Route::BlogList {})]
    //     #[redirect("/:name", |name: String| Route::BlogPost { name })]
    // #[end_nest]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
  }
