#[derive(PartialEq, Eq)]
pub enum ExpanderStatus {
    Closed,
    Share,
    ExpandMore,
}

impl ExpanderStatus {
    pub fn striing(&self) -> std::string::String {
        match *self {
            ExpanderStatus::Closed => std::string::String::from("Closed"),
            ExpanderStatus::Share => std::string::String::from("share"),
            ExpanderStatus::ExpandMore => std::string::String::from("expand more"),
        }
    }
}

#[yew::function_component(Home)]
pub fn home() -> Html {
    let padding_summary = crate::global_variables::hardcode::HEADER_HEIGHT_PX
        + crate::global_variables::hardcode::HEADER_BORDER_BOTTOM_PX;
    let style_handle = format!(
        "
        padding-top: {}px;
      ",
        padding_summary
    );
    let expander_status = yew::use_state(|| ExpanderStatus::Closed);
    let expander_status_clone_for_logic = expander_status.clone();
    let drawer_style_left = yew::use_state(|| {
        crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial
    });
    let drawer_style_left_cloned_on_open = drawer_style_left.clone();
    let expander_status_clone_drawer_on_open_left = expander_status.clone();
    let on_open_left = yew::Callback::from(move |_| {
        expander_status_clone_drawer_on_open_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_open.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_left_cloned_first_another = drawer_style_left_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_left_cloned_first_another
                .set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_left_cloned_on_close = drawer_style_left.clone();
    let expander_status_clone_drawer_on_close_left = expander_status.clone();
    let on_close_left = yew::Callback::from(move |_| {
        expander_status_clone_drawer_on_close_left.set(ExpanderStatus::Closed);
        drawer_style_left_cloned_on_close.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_left_cloned_second_another = drawer_style_left_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_left_cloned_second_another.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_left_enum_handle = &*drawer_style_left;
    ////
    let drawer_style_right = yew::use_state(|| {
        crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial
    });
    let drawer_style_right_cloned_on_open = drawer_style_right.clone();
    let expander_status_clone_drawer_on_open_right = expander_status.clone();
    let on_open_right = yew::Callback::from(move |_| {
        let _f = crate::components::rc::rc_animate::util::motion::get_option_style();
        expander_status_clone_drawer_on_open_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_open.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::OpenedBeforeTimeout);
        let drawer_style_right_cloned_first_another = drawer_style_right_cloned_on_open.clone();
        gloo::timers::callback::Timeout::new(50, move || {
            drawer_style_right_cloned_first_another
                .set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::OpenedAfterTimeout);
        })
        .forget();
    });
    let drawer_style_right_cloned_on_close = drawer_style_right.clone();
    let expander_status_clone_drawer_on_close_right = expander_status.clone();
    let on_close_right = yew::Callback::from(move |_| {
        expander_status_clone_drawer_on_close_right.set(ExpanderStatus::Closed);
        drawer_style_right_cloned_on_close.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::ClosedBeforeTimeout);
        let drawer_style_right_cloned_second_another = drawer_style_right_cloned_on_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            drawer_style_right_cloned_second_another.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
        })
        .forget();
    });
    let drawer_style_right_enum_handle = &*drawer_style_right;
    let expander_status_cloned_share = expander_status.clone();
    let expander_status_cloned_expand_more = expander_status.clone();
    let expander_status_cloned_close = expander_status;
    let share_inner_html =
        yew::html! {<crate::components::feed::expander::share_content::ShareContent/>};
    let expand_more_inner_html =
        yew::html! {<crate::components::feed::expander::expand_more_content::ExpandMoreContent/>};
    // <crate::components::material::pure_material_button_outlined::PureMaterialButtonOutlined/>
    // <crate::components::material::pure_material_progress_circular::PureMaterialProgressCircular/>
    // <crate::components::material::pure_material_progress_linear::PureMaterialProgressLinear/>
    // <crate::components::material::pure_material_textfield_filled::PureMaterialTextfieldFilled/>
    // <crate::components::material::pure_material_textfield_outlined::PureMaterialTextfieldOutlined/>
    // <crate::components::material::pure_material_radio::PureMaterialRadio/>
    // <crate::components::material::pure_material_button_contained::PureMaterialButtonContained/>
    // <crate::components::material::pure_material_button_text::PureMaterialButtonText/>
    // <crate::components::material::pure_material_switch::PureMaterialSwitch/>
    // <crate::components::material::pure_material_checkbox::PureMaterialCheckbox/>
    // <crate::components::material::pure_material_textfield_standard::PureMaterialTextfieldStandard/>
    // <crate::components::material::pure_material_slider::PureMaterialSlider/>
    // let svg_type = ;
    let icon_inner_html = yew::html! {
      <crate::components::ant_design::general::icon::Icon
        svg_type={
          super::ant_design::svg::helpers::svg_type::SvgType::Loading(
            crate::components::ant_design::svg::helpers::svg_props::SvgProps{
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
    // let rotate = crate::helpers::rotate::Rotate::new(60).unwrap();
    let _g = yew::html! {
      <crate::components::ant_design::general::icon::Icon
        svg_type={
          super::ant_design::svg::helpers::svg_type::SvgType::Up(
            crate::components::ant_design::svg::helpers::svg_props::SvgProps{
              height: None,
              width: None,
              fill: Some(crate::components::ant_design::svg::helpers::fill_with::FillWith::Hsl(colorsys::Hsl::new(0.0, 100.0, 50.0, Some(1.0)))),
              spin: Some(()),
              rotate: None,
              theme: Some(crate::components::ant_design::svg::helpers::theme::Theme::TwoTone),
            }
          )
        }
      />
    };
    let select_callback = yew::Callback::from(|value: (web_sys::MouseEvent, Option<String>)| {
        gloo::console::log!("eeee", value.0.target());
        gloo::console::log!("kkkkkkekw", value.1);
    });
    let inner_html_left = yew::html! {
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
       <crate::components::ant_design::general::button::Button
         placeholder={String::from("Button")}
        //  disabled={Some(())}
         button_type={crate::components::ant_design::general::button::ButtonType::Primary}
        //  shape={crate::components::ant_design::button::Shape::Circle}
        icon={Some(icon_inner_html.clone())}
        size={crate::components::ant_design::button::Size::Large}
        // ghost={Some(())}
        // block={Some(())}
        // loading={crate::components::ant_design::button::LoadingProp::Bool(true)}
       />
      // <crate::components::ant_design::paragraph::Paragraph/>
      // <crate::components::ant_design::svg::helpers::svg_type::List>
      //   <crate::components::ant_design::svg::down::Down/>
      //   <crate::components::ant_design::svg::up::Up/>
      // </crate::components::ant_design::svg::helpers::svg_type::List>

      // <crate::components::ant_design::feedback::alert::Alert
      //   message={String::from("Error")}
      //   description={String::from("This is an error message about copywriting.")}
      //   type_handle={crate::components::ant_design::alert::AlertType::Success}
      //   closable={Some(())}
      //   close_text={String::from("close text")}
      //   show_icon={Some(())}
      //   on_close={yew::Callback::from(|_|{
      //     gloo::console::log!("onclose");
      //   })}
      // />
      // <crate::components::ant_design::avatar::Avatar
      //   size={super::ant_design::avatar::AvatarSize::Type(super::ant_design::avatar::AvatarSizeType::Large)}
      //   shape={super::ant_design::avatar::AvatarShape::Square}
      //   // content={crate::components::ant_design::avatar::AvatarContent::Image(crate::components::ant_design::avatar::AvatarImage{
      //   //   src: std::string::String::from("https://avatars.mds.yandex.net/i?id=0baad4e75b583fcb7ce171f1ce863011-5284759-images-thumbs&n=13&exp=1"),
      //   //   alt: std::string::String::from("avatar"),
      //   //   on_error: Some(yew::Callback::from(|_: yew::Event| {
      //   //     gloo::console::log!("on_error");
      //   //   }))
      //   // })}
      //   // content={crate::components::ant_design::avatar::AvatarContent::Icon(super::ant_design::svg::helpers::svg_type::SvgType::User)}
      // />
      // <Switch/>
      <super::ant_design::data_entry::select::Select
        values={vec![String::from("alice"), std::string::String::from("bob")]}
        default_value={String::from("bob")}
        id={String::from("09760707")}
        set_choosen_value={select_callback}
      />
      <crate::components::ant_design::data_entry::switch::custom_component::CustomSwitch
        reference={yew::NodeRef::default()}
        title={Some(String::from("title"))}
      />
      <crate::components::rc::rc_checkbox::custom_component::CustomCheckBox
        on_click={Some(yew::Callback::from(|_|{gloo::console::log!("looog");}))}
        reference={yew::NodeRef::default()}
      />
      <crate::components::ant_design::data_display::badge::Badge
        count={Some(2)}
        // overflow_count={Some(123)}
        // color={colorsys::Hsl::new(0.0, 100.0, 66.0, Some(1.0))}
        dot={Some(Some(crate::components::ant_design::data_display::badge::BadgeStatus::Success(Some(String::from("kekw")))))}
        // offset={Some(crate::components::ant_design::helpers::offset::Offset{x:20, y:-20})}
        // show_zero={Some(())}
        // status={Some(crate::components::ant_design::data_display::badge::BadgeStatus::Success)}
        // title={Some(String::from("tittle"))}
      >
        // <a href="" class="head-example">
        // </a>
      </crate::components::ant_design::data_display::badge::Badge>
      // <crate::components::ant_design::back_top::BackTop></crate::components::ant_design::back_top::BackTop>
      // <crate::components::rc::rc_progress::circle::Circle
      //   percent={Some(crate::components::rc::rc_progress::interface::Percent::Number(25.0))}
      //   stroke_width={4.0}
      //   stroke_color={
      //     Some(crate::components::rc::rc_progress::interface::StrokeColor {
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


        //       <crate::components::rc::rc_progress::line::Line
        // percent={Some(crate::components::rc::rc_progress::interface::Percent::Number(10.0))}
        // stroke_width={4.0}
        // stroke_color={
        //   Some(StrokeColorType::BaseStrokeColorType(
        //         BaseStrokeColorType::String(String::from("#D3D3D3")),
        //     ))
        // }/>
      // <div style="background-color: red; width: 300px; height: 2500px;"/>
      </div>

    };
    let inner_html_right = yew::html! {};
    let expander_style = yew::use_state(|| {
        crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::Initial
    });
    let expander_style_clone_open_expand_more = expander_style.clone();
    let drawer_style_right_expand_more = drawer_style_right.clone();
    let drawer_style_left_expand_more = drawer_style_left.clone();
    let expander_on_open_expand_more = yew::Callback::from(move |_| {
        drawer_style_right_expand_more.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
        drawer_style_left_expand_more.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
        if let ExpanderStatus::Closed = *expander_status_cloned_expand_more {
            expander_status_cloned_expand_more.set(ExpanderStatus::ExpandMore);
            expander_style_clone_open_expand_more
                .set(crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::OpenedBeforeTimeout);
            let expander_style_clone_open_another = expander_style_clone_open_expand_more.clone();
            gloo::timers::callback::Timeout::new(50, move || {
                expander_style_clone_open_another
                    .set(crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::OpenedAfterTimeout);
            })
            .forget();
        }
    });
    let expander_style_clone_open_share = expander_style.clone();
    let drawer_style_right_share = drawer_style_right.clone();
    let drawer_style_left_share = drawer_style_left.clone();
    let expander_on_open_share: yew::Callback<web_sys::MouseEvent> = yew::Callback::from(
        move |_| {
            drawer_style_right_share.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
            drawer_style_left_share.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
            if let ExpanderStatus::Closed = *expander_status_cloned_share {
                expander_status_cloned_share.set(ExpanderStatus::Share);
                expander_style_clone_open_share.set(crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::OpenedBeforeTimeout);
                let expander_style_clone_open_another = expander_style_clone_open_share.clone();
                gloo::timers::callback::Timeout::new(50, move || {
                expander_style_clone_open_another
                    .set(crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::OpenedAfterTimeout);
            })
            .forget();
            }
        },
    );
    let expander_style_clone_close = expander_style.clone();
    let drawer_style_right_expander_on_close = drawer_style_right.clone();
    let drawer_style_left_expander_on_close = drawer_style_left.clone();
    let expander_on_close = yew::Callback::from(move |_| {
        drawer_style_right_expander_on_close.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
        drawer_style_left_expander_on_close.set(crate::components::drawer::drawer_changing_style_state::DrawerChangingStyleState::Initial);
        expander_style_clone_close.set(crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::ClosedBeforeTimeout);
        let expander_style_clone_close_another = expander_style_clone_close.clone();
        let expander_status_cloned_close_another = expander_status_cloned_close.clone();
        gloo::timers::callback::Timeout::new(350, move || {
            expander_style_clone_close_another.set(crate::components::feed::expander::expander_changing_style_state::ExpanderChangingStyleState::Initial);
            expander_status_cloned_close_another.set(ExpanderStatus::Closed);
        })
        .forget();
    });
    let expander_style_clone_close_handle = &*expander_style;
    let expander_inner_html = match *expander_status_clone_for_logic {
        ExpanderStatus::Closed => {
            yew::html! {<crate::components::ant_design::feedback::alert::Alert/>}
        }
        ExpanderStatus::Share => share_inner_html,
        ExpanderStatus::ExpandMore => expand_more_inner_html,
    };
    yew::html! {
      <>
        <crate::components::header::component::Header
          left_drawer_callback={on_open_left}
          right_drawer_callback={on_open_right}
        />
        <crate::components::drawer::component::Drawer
          callback={on_close_left}
          style_state={drawer_style_left_enum_handle.clone()}
          drawer_position={crate::components::drawer::position::DrawerPosition::Left}
          inner_html={inner_html_left}
        />
        <crate::components::drawer::component::Drawer
          callback={on_close_right}
          style_state={drawer_style_right_enum_handle.clone()}
          drawer_position={crate::components::drawer::position::DrawerPosition::Right}
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
            <crate::components::feed::posts_list::PostsList
              share_callback={expander_on_open_share.clone()}//expander_status_to_share
              expand_more_callback={expander_on_open_expand_more.clone()}//expander_status_to_expand_more
            />
            <crate::components::feed::expander::component::Expander
              callback={expander_on_close}
              style_state={expander_style_clone_close_handle.clone()}
              inner_html={expander_inner_html}
            />
          </div>
        </div>
      </>
    }
}
