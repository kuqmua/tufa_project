use crate::components::feed::post::Post;
use crate::constants::FEED_WIDTH_PX;
use crate::constants::INTERFACE_LINES_COLOR;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
    pub share_callback: Callback<MouseEvent>,
    pub expand_more_callback: Callback<MouseEvent>,
}

#[function_component(PostsList)]
pub fn posts_list(props: &PostsListProps) -> Html {
    let posts_vec = vec![
        html! {
          <Post
            share_callback={props.share_callback.clone()}
            expand_more_callback={props.expand_more_callback.clone()}
          />
        },
        html! {
          <Post
            share_callback={props.share_callback.clone()}
            expand_more_callback={props.expand_more_callback.clone()}
          />
        },
        html! {
          <Post
            share_callback={props.share_callback.clone()}
            expand_more_callback={props.expand_more_callback.clone()}
          />
        },
    ];
    let style_handle = format!(
        "
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: flex-start;
          width: {}px;
          min-width: {}px;
          padding-left: 8px;
          padding-right: 8px;
          border-right: 1px solid {};
          border-left: 1px solid {};
        ",
        FEED_WIDTH_PX, FEED_WIDTH_PX, INTERFACE_LINES_COLOR, INTERFACE_LINES_COLOR
    );
    html! {
      <div
        style={style_handle}
      >
        {posts_vec}
      </div>
    }
}
