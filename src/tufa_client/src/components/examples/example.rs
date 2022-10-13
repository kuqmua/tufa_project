use crate::components::examples::counter::Counter;
use crate::components::examples::display_count::DisplayCount;
use crate::components::examples::display_credentials::DisplayCredentials;
use crate::components::examples::get_data_from_server_button::GetDataFromServerButton;
use crate::components::examples::post_data_to_server_button::PostDataToServerButton;
use crate::components::examples::set_timeout_example::SetTimeoutExample;
use crate::components::examples::without_html_tag_example::WithoutHtmlTagExample;
use crate::components::examples::yewdux_functional_component_example::YewduxFunctionalComponentExample;
use crate::store::YewduxStore;
use yew::html;
use yew::prelude::*;
use yew::Html;
use yewdux::prelude::*;

pub struct Example {}

impl Component for Example {
    type Message = ();
    type Properties = DispatchProps<PersistentStore<YewduxStore>>;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}
    fn destroy(&mut self, _ctx: &Context<Self>) {}
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div
            style=""
          >
              <WithoutHtmlTagExample/>
              <SetTimeoutExample/>
              <PostDataToServerButton/>
              <GetDataFromServerButton/>
              <YewduxFunctionalComponentExample/>
              <WithDispatch<DisplayCount>/>
              <WithDispatch<Counter>/>
              <WithDispatch<DisplayCredentials>/>
          </div>
        }
    }
}
