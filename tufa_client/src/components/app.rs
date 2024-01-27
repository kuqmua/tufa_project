#[yew::function_component(App)]
pub fn app() -> yew::html::Html {
    yew::html! {
      <yew_router::BrowserRouter>
        <crate::components::common_style::CommonStyle/>
        <yew_router::Switch<crate::routes::routes::Routes> render={yew_router::Switch::render(crate::routes::switch_routes::switch_routes)} />
      </yew_router::BrowserRouter>
    }
}
