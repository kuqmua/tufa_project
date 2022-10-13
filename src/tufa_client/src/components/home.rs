// use crate::components::ant_design::avatar::Avatar;
// use crate::components::ant_design::avatar::AvatarContent;
// use crate::components::ant_design::avatar::AvatarImage;
// use crate::components::ant_design::button::LoadingProp;
// use crate::components::ant_design::button::Shape;
// use crate::components::ant_design::button::Size;
use crate::components::ant_design::svg::helpers::fill_with::FillWith;
// use crate::components::alert::Alert;
// use super::ant_design::avatar::AvatarShape;
// use super::ant_design::avatar::AvatarSize;
// use super::ant_design::avatar::AvatarSizeType;
use super::ant_design::data_entry::select::Select;
use super::ant_design::svg::helpers::svg_type::SvgType;
use crate::components::ant_design::feedback::alert::Alert;
// use crate::components::ant_design::alert::AlertType;
use crate::components::ant_design::data_display::badge::BadgeStatus;
use crate::components::ant_design::general::button::Button;
use crate::components::ant_design::general::button::ButtonType;
use crate::components::ant_design::general::button::Size;
// use crate::components::ant_design::helpers::offset::Offset;
use crate::components::ant_design::general::icon::Icon;
// use crate::components::ant_design::paragraph::Paragraph;
// use crate::components::ant_design::svg::down::Down;
// use crate::components::ant_design::svg::helpers::svg_type::List;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::theme::Theme;
// use crate::components::ant_design::svg::up::Up;
use crate::components::drawer::component::Drawer;
use crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState;
use crate::components::drawer::position::DrawerPosition;
use crate::components::feed::expander::component::Expander;
use crate::components::feed::expander::expand_more_content::ExpandMoreContent;
use crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState;
use crate::components::feed::expander::share_content::ShareContent;
use crate::components::feed::posts_list::PostsList;
use crate::components::header::component::Header;
// use crate::components::material::pure_material_button_contained::PureMaterialButtonContained;
// use crate::components::material::pure_material_button_outlined::PureMaterialButtonOutlined;
// use crate::components::material::pure_material_button_text::PureMaterialButtonText;
// use crate::components::material::pure_material_checkbox::PureMaterialCheckbox;
// use crate::components::material::pure_material_progress_circular::PureMaterialProgressCircular;
// use crate::components::material::pure_material_progress_linear::PureMaterialProgressLinear;
// use crate::components::material::pure_material_radio::PureMaterialRadio;
// use crate::components::material::pure_material_slider::PureMaterialSlider;
// use crate::components::material::pure_material_switch::PureMaterialSwitch;
// use crate::components::material::pure_material_textfield_filled::PureMaterialTextfieldFilled;
// use crate::components::material::pure_material_textfield_outlined::PureMaterialTextfieldOutlined;
// use crate::components::material::pure_material_textfield_standard::PureMaterialTextfieldStandard;
use crate::constants::HEADER_BORDER_BOTTOM_PX;
use crate::constants::HEADER_HEIGHT_PX;
// use crate::helpers::rotate::Rotate;
// use crate::components::ant_design::back_top::BackTop;
use crate::components::ant_design::data_display::badge::Badge;
use crate::components::rc::rc_animate::util::motion::get_option_style;
// use crate::components::rc::rc_checkbox::component::RcCheckBox;
use crate::components::rc::rc_checkbox::custom_component::CustomCheckBox;
// use crate::components::rc::rc_progress::circle::Circle;
// use crate::components::rc::rc_progress::interface::Percent;
// use crate::components::rc::rc_progress::interface::StrokeColor;
// use crate::components::rc::rc_progress::line::Line;
// use crate::components::rc::rc_switch::component::RcSwitch;
// use crate::components::rc::rc_switch::component::RcSwitchProps;
use crate::components::ant_design::data_entry::switch::custom_component::CustomSwitch;
use colorsys::Hsl;
use gloo::console::log;
use web_sys::MouseEvent;
use yew::html::onselect::Event;
use yew::NodeRef;
use yew::{function_component, html, use_state, Callback};

#[derive(Debug, PartialEq, Eq)]
pub enum ExpanderStatus {
    Closed,
    Share,
    ExpandMore,
}

impl ExpanderStatus {
    pub fn striing(&self) -> String {
        match *self {
            ExpanderStatus::Closed => String::from("Closed"),
            ExpanderStatus::Share => String::from("share"),
            ExpanderStatus::ExpandMore => String::from("expand more"),
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let padding_summary = HEADER_HEIGHT_PX + HEADER_BORDER_BOTTOM_PX;
    let style_handle = format!(
        "
        padding-top: {}px;
      ",
        padding_summary
    );
    let expander_status = use_state(|| ExpanderStatus::Closed);
    let expander_status_clone_for_logic = expander_status.clone();
    let drawer_style_left = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_left_cloned_on_open = drawer_style_left.clone();
    let expander_status_clone_drawer_on_open_left = expander_status.clone();
    let on_open_left = Callback::from(move |_| {
        expander_status_clone_drawer_on_open_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_left_cloned_first_another = drawer_style_left_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_left_cloned_first_another
                .set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_left_cloned_on_close = drawer_style_left.clone();
    let expander_status_clone_drawer_on_close_left = expander_status.clone();
    let on_close_left = Callback::from(move |_| {
        expander_status_clone_drawer_on_close_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_left_cloned_second_another = drawer_style_left_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_left_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_left_enum_handle = &*drawer_style_left;
    ////
    let drawer_style_right = use_state(|| DrawerChangingStyleState::Initial);
    let drawer_style_right_cloned_on_open = drawer_style_right.clone();
    let expander_status_clone_drawer_on_open_right = expander_status.clone();
    let on_open_right = Callback::from(move |_| {
        let _f = get_option_style();
        expander_status_clone_drawer_on_open_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_open.set(DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_right_cloned_first_another = drawer_style_right_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_right_cloned_first_another
                .set(DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_right_cloned_on_close = drawer_style_right.clone();
    let expander_status_clone_drawer_on_close_right = expander_status.clone();
    let on_close_right = Callback::from(move |_| {
        expander_status_clone_drawer_on_close_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_close.set(DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_right_cloned_second_another = drawer_style_right_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_right_cloned_second_another.set(DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_right_enum_handle = &*drawer_style_right;
    let expander_status_cloned_share = expander_status.clone();
    let expander_status_cloned_expand_more = expander_status.clone();
    let expander_status_cloned_close = expander_status;
    let share_inner_html = html! {<ShareContent/>};
    let expand_more_inner_html = html! {<ExpandMoreContent/>};
    // <PureMaterialButtonOutlined/>
    // <PureMaterialProgressCircular/>
    // <PureMaterialProgressLinear/>
    // <PureMaterialTextfieldFilled/>
    // <PureMaterialTextfieldOutlined/>
    // <PureMaterialRadio/>
    // <PureMaterialButtonContained/>
    // <PureMaterialButtonText/>
    // <PureMaterialSwitch/>
    // <PureMaterialCheckbox/>
    // <PureMaterialTextfieldStandard/>
    // <PureMaterialSlider/>
    // let svg_type = ;
    let icon_inner_html = html! {
      <Icon
        svg_type={
          SvgType::Loading(
            SvgProps{
              height: None,
              width: None,
              fill: None,
              spin: Some(()),
              rotate: None,
              theme: None,
            }
          )
        }
      />
    };
    // let rotate = Rotate::new(60).unwrap();
    let _g = html! {
      <Icon
        svg_type={
          SvgType::Up(
            SvgProps{
              height: None,
              width: None,
              fill: Some(FillWith::Hsl(Hsl::new(0.0, 100.0, 50.0, Some(1.0)))),
              spin: Some(()),
              rotate: None,
              theme: Some(Theme::TwoTone),
            }
          )
        }
      />
    };
    let select_callback = Callback::from(|value: (MouseEvent, Option<String>)| {
        log!("eeee", value.0.target());
        log!("kkkkkkekw", value.1);
    });
    let inner_html_left = html! {
      <div
        style="
          display: flex;
          flex-direction: column;
          justify-content: center;
          align-items: center;
          height: 100%;
          width: 100%;
          padding: 10px;
        "
      >
       <Button
         placeholder={String::from("Button")}
        //  disabled={Some(())}
         button_type={ButtonType::Primary}
        //  shape={Shape::Circle}
        icon={Some(icon_inner_html.clone())}
        size={Size::Large}
        // ghost={Some(())}
        // block={Some(())}
        // loading={LoadingProp::Bool(true)}
       />
      // <Paragraph/>
      // <List>
      //   <Down/>
      //   <Up/>
      // </List>

      // <Alert
      //   message={String::from("Error")}
      //   description={String::from("This is an error message about copywriting.")}
      //   type_handle={AlertType::Success}
      //   closable={Some(())}
      //   close_text={String::from("close text")}
      //   show_icon={Some(())}
      //   on_close={Callback::from(|_|{
      //     log!("onclose");
      //   })}
      // />
      // <Avatar
      //   size={AvatarSize::Type(AvatarSizeType::Large)}
      //   shape={AvatarShape::Square}
      //   // content={AvatarContent::Image(AvatarImage{
      //   //   src: String::from("https://avatars.mds.yandex.net/i?id=0baad4e75b583fcb7ce171f1ce863011-5284759-images-thumbs&n=13&exp=1"),
      //   //   alt: String::from("avatar"),
      //   //   on_error: Some(Callback::from(|_: yew::Event| {
      //   //     log!("on_error");
      //   //   }))
      //   // })}
      //   // content={AvatarContent::Icon(SvgType::User)}
      // />
      // <Switch/>
      <Select
        values={vec![String::from("alice"), String::from("bob")]}
        default_value={String::from("bob")}
        id={String::from("09760707")}
        set_choosen_value={select_callback}
      />
      <CustomSwitch
        reference={NodeRef::default()}
        title={Some(String::from("title"))}
      />
      <CustomCheckBox
        on_click={Some(Callback::from(|_|{log!("looog");}))}
        reference={NodeRef::default()}
      />
      <Badge
        count={Some(2)}
        // overflow_count={Some(123)}
        // color={Hsl::new(0.0, 100.0, 66.0, Some(1.0))}
        dot={Some(Some(BadgeStatus::Success(Some(String::from("kekw")))))}
        // offset={Some(Offset{x:20, y:-20})}
        // show_zero={Some(())}
        // status={Some(BadgeStatus::Success)}
        // title={Some(String::from("tittle"))}
      >
        // <a href="" class="head-example">
        // </a>
      </Badge>
      // <BackTop></BackTop>
      // <Circle
      //   percent={Some(Percent::Number(25.0))}
      //   stroke_width={4.0}
      //   stroke_color={
      //     Some(StrokeColor {
      //           colors: vec![String::from("#D3D3D3")],
      //       })
      //   }
      //   trail_color={Some(String::from("#D9D9D9"))}
      // />

      // <svg
      //   class="rc-progress-circle"
      //   viewBox="0 0 100 100"
      // >
      //   <circle
      //     class="rc-progress-circle-trail"
      //     r="48"
      //     cx="50"
      //     cy="50"
      //     stroke="#D9D9D9"
      //     stroke-linecap="round"
      //     stroke-width="1"
      //     style="stroke: rgb(217, 217, 217); stroke-dasharray: 301.593px, 301.593; stroke-dashoffset: 0; transform: rotate(-90deg); transform-origin: 50% 50%; transition: stroke-dashoffset 0.3s ease 0s, stroke-dasharray 0.3s ease 0s, stroke 0.3s ease 0s, stroke-width 0.06s ease 0.3s, opacity 0.3s ease 0s; fill-opacity: 0;"
      //   >
      //   </circle>
      //   <circle
      //     class="rc-progress-circle-path"
      //     r="48"
      //     cx="50"
      //     cy="50"
      //     stroke-linecap="round"
      //     stroke-width="4"
      //     opacity="1"
      //     style="stroke: rgb(211, 211, 211); stroke-dasharray: 301.593px, 301.593; stroke-dashoffset: 228.195; transform: rotate(-90deg); transform-origin: 50% 50%; transition: stroke-dashoffset 0.3s ease 0s, stroke-dasharray 0.3s ease 0s, stroke 0.3s ease 0s, stroke-width 0.06s ease 0.3s, opacity ease 0s; fill-opacity: 0;"
      //   >
      //   </circle>
      // </svg>


        //       <Line
        // percent={Some(Percent::Number(10.0))}
        // stroke_width={4.0}
        // stroke_color={
        //   Some(StrokeColorType::BaseStrokeColorType(
        //         BaseStrokeColorType::String(String::from("#D3D3D3")),
        //     ))
        // }/>
      // <div style="background-color: red; width: 300px; height: 2500px;"/>
      </div>

    };
    let inner_html_right = html! {};
    let expander_style = use_state(|| ExpanderChangingStyleState::Initial);
    let expander_style_clone_open_expand_more = expander_style.clone();
    let drawer_style_right_expand_more = drawer_style_right.clone();
    let drawer_style_left_expand_more = drawer_style_left.clone();
    let expander_on_open_expand_more = Callback::from(move |_| {
        drawer_style_right_expand_more.set(DrawerChangingStyleState::Initial);
        drawer_style_left_expand_more.set(DrawerChangingStyleState::Initial);
        if let ExpanderStatus::Closed = *expander_status_cloned_expand_more {
            expander_status_cloned_expand_more.set(ExpanderStatus::ExpandMore);
            expander_style_clone_open_expand_more
                .set(ExpanderChangingStyleState::OpenedBeforeTimeout);
            let expander_style_clone_open_another = expander_style_clone_open_expand_more.clone();
            gloo::timers::callback::Timeout::new(50, move || {
                expander_style_clone_open_another
                    .set(ExpanderChangingStyleState::OpenedAfterTimeout);
            })
            .forget();
        }
    });
    let expander_style_clone_open_share = expander_style.clone();
    let drawer_style_right_share = drawer_style_right.clone();
    let drawer_style_left_share = drawer_style_left.clone();
    let expander_on_open_share: Callback<MouseEvent> = Callback::from(move |_| {
        drawer_style_right_share.set(DrawerChangingStyleState::Initial);
        drawer_style_left_share.set(DrawerChangingStyleState::Initial);
        if let ExpanderStatus::Closed = *expander_status_cloned_share {
            expander_status_cloned_share.set(ExpanderStatus::Share);
            expander_style_clone_open_share.set(ExpanderChangingStyleState::OpenedBeforeTimeout);
            let expander_style_clone_open_another = expander_style_clone_open_share.clone();
            gloo::timers::callback::Timeout::new(50, move || {
                expander_style_clone_open_another
                    .set(ExpanderChangingStyleState::OpenedAfterTimeout);
            })
            .forget();
        }
    });
    let expander_style_clone_close = expander_style.clone();
    let drawer_style_right_expander_on_close = drawer_style_right.clone();
    let drawer_style_left_expander_on_close = drawer_style_left.clone();
    let expander_on_close = Callback::from(move |_| {
        drawer_style_right_expander_on_close.set(DrawerChangingStyleState::Initial);
        drawer_style_left_expander_on_close.set(DrawerChangingStyleState::Initial);
        expander_style_clone_close.set(ExpanderChangingStyleState::ClosedBeforeTimeout);
        let expander_style_clone_close_another = expander_style_clone_close.clone();
        let expander_status_cloned_close_another = expander_status_cloned_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            expander_style_clone_close_another.set(ExpanderChangingStyleState::Initial);
            expander_status_cloned_close_another.set(ExpanderStatus::Closed);
        })
        .forget();
    });
    let expander_style_clone_close_handle = &*expander_style;
    let expander_inner_html = match *expander_status_clone_for_logic {
        ExpanderStatus::Closed => html! {<Alert/>},
        ExpanderStatus::Share => share_inner_html,
        ExpanderStatus::ExpandMore => expand_more_inner_html,
    };
    html! {
      <>
        <Header
          left_drawer_callback={on_open_left}
          right_drawer_callback={on_open_right}
        />
        <Drawer
          callback={on_close_left}
          style_state={drawer_style_left_enum_handle.clone()}
          drawer_position={DrawerPosition::Left}
          inner_html={inner_html_left}
        />
        <Drawer
          callback={on_close_right}
          style_state={drawer_style_right_enum_handle.clone()}
          drawer_position={DrawerPosition::Right}
          inner_html={inner_html_right}
        />
        <div
          style="
            width: 100%; 
            height: 100%;
            display: flex; 
            justify-content: center; 
            flex-direction: column; 
            align-items: center;
          "
        >
          <div
            style={style_handle}
          >
            <PostsList
              share_callback={expander_on_open_share.clone()}//expander_status_to_share
              expand_more_callback={expander_on_open_expand_more.clone()}//expander_status_to_expand_more
            />
            <Expander
              callback={expander_on_close}
              style_state={expander_style_clone_close_handle.clone()}
              inner_html={expander_inner_html}
            />
          </div>
        </div>
      </>
    }
}
