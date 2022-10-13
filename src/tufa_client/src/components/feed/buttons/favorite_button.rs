use crate::components::feed::buttons::button_wrapper::ButtonWrapper;
use crate::components::material::svg::favorite::Favorite;
use crate::components::material::svg::favorite_border::FavoriteBorder;
use crate::constants::FEED_ICONS_COLOR;
use yew::{function_component, html, use_state, Callback};

#[function_component(FavoriteButton)]
pub fn favorite_button() -> Html {
    let is_liked = use_state(|| false);
    let is_liked_cloned = is_liked.clone();
    let change_is_liked = Callback::from(move |_| {
        is_liked_cloned.set(!*is_liked_cloned);
    });
    let icon_size = "24px".to_owned();
    let html_handle = if *is_liked {
        html! {<Favorite height={icon_size.clone()} width={icon_size} fill={"#ffffa2".to_owned()}/>}
    } else {
        html! {<FavoriteBorder height={icon_size.clone()} width={icon_size} fill={FEED_ICONS_COLOR.to_owned()}/>}
    };
    html! {
      <ButtonWrapper inner_html={html_handle} callback={change_is_liked}/>
    }
}
