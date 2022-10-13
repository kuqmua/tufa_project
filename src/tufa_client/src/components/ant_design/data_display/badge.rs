use crate::components::ant_design::helpers::offset::Offset;
use colorsys::Hsl;
use gloo::console::error;
use tufa_common::helpers::numeric::Numeric;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BadgeStatus {
    Success(Option<String>),
    Processing(Option<String>),
    Default(Option<String>),
    Error(Option<String>),
    Warning(Option<String>),
}

impl BadgeStatus {
    pub fn get_sup_class(&self) -> String {
        match self {
            BadgeStatus::Success(_) => String::from("ant-badge-status-success"),
            BadgeStatus::Processing(_) => String::from("ant-badge-status-processing"),
            BadgeStatus::Default(_) => String::from("ant-badge-status-default"),
            BadgeStatus::Error(_) => String::from("ant-badge-status-error"),
            BadgeStatus::Warning(_) => String::from("ant-badge-status-warning"),
        }
    }
    pub fn get_span_class(&self) -> String {
        String::from("ant-badge-status")
    }
    pub fn get_class(&self) -> String {
        String::from("ant-badge-status-dot")
    }
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub color: Option<Hsl>,               // Customize Badge dot color	string	-	3.16.0
    pub count: Option<u64>,               //	Number to show in badge	ReactNode
    pub dot: Option<Option<BadgeStatus>>, // Whether to display a red dot instead of count	boolean	false
    pub offset: Option<Offset>,           //	set offset of the badge dot, like[x, y]	[number, number]	-
    pub overflow_count: Option<u64>, //dont think it would be usefull//	Max count to show	number	99
    pub show_zero: Option<()>,       //	Whether to show badge when count is zero	boolean	false
    // pub status: Option<BadgeStatus>, //	Set Badge as a status dot	success | processing | default | error | warning	''
    // pub text: Option<String>, //	If status is set, text sets the display text of the status dot	string	''
    pub title: Option<String>, //	Text to show when hovering over the badge	string	count
    #[prop_or_default]
    pub children: Children,
}

//todo ScrollNumber and animation
#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let offset_style = match props.offset.clone() {
        None => String::from(""),
        Some(badge_offset) => {
            let right_or_left = match badge_offset.x.is_positive() {
                true => format!("right: -{}px;", badge_offset.x),
                false => format!("right: {}px;", badge_offset.x.abs()),
            };
            let margin_top_or_bottom = match badge_offset.y.is_positive() {
                true => format!("margin-top: {}px;", badge_offset.y),
                false => format!("margin-top: {}px;", badge_offset.y),
            };
            format!("{} {}", right_or_left, margin_top_or_bottom)
        }
    };
    let title = match (props.title.clone(), props.count) {
        (None, None) => String::from(""),
        (None, Some(count)) => count.to_string(),
        (Some(title), None) => title,
        (Some(title), Some(_)) => title,
    };
    let color_style = match props.color.clone() {
        None => String::from(""),
        Some(color) => format!("background: {};", color.to_css_string()),
    };
    let sup_style = format!("{} {}", color_style, offset_style);
    let count_class = match props.count {
        None => String::from(""),
        Some(count) => match (props.dot.clone(), count == 0, props.show_zero.clone()) {
            (None, true, None) => String::from(""),
            (None, true, Some(_)) => String::from("ant-badge-count"),
            (None, false, None) => String::from("ant-badge-count"),
            (None, false, Some(_)) => String::from("ant-badge-count"),
            (Some(_), true, None) => String::from(""),
            (Some(_), true, Some(_)) => String::from(""),
            (Some(_), false, None) => String::from(""),
            (Some(_), false, Some(_)) => String::from(""),
        },
    };
    let scroll_class = match props.count {
        None => String::from(""),
        Some(count) => match (props.dot.clone(), count == 0, props.show_zero.clone()) {
            (None, true, None) => String::from(""),
            (None, true, Some(_)) => String::from("ant-scroll-number"),
            (None, false, None) => String::from("ant-scroll-number"),
            (None, false, Some(_)) => String::from("ant-scroll-number"),
            (Some(_), true, None) => String::from(""),
            (Some(_), true, Some(_)) => String::from(""),
            (Some(_), false, None) => String::from(""),
            (Some(_), false, Some(_)) => String::from(""),
        },
    };
    let (status_sup_class, status_span_class, dot_class) = match props.dot.clone() {
        None => (String::from(""), String::from(""), String::from("")),
        Some(option_badge_status) => match option_badge_status {
            None => (
                String::from(""),
                String::from(""),
                String::from("ant-badge-dot"),
            ),
            Some(status) => (
                status.get_sup_class(),
                status.get_span_class(),
                status.get_class(),
            ),
        },
    };
    let multiple_words_class = match props.count.clone() {
        None => String::from(""),
        Some(count) => match count.to_string().len() > 1 {
            false => String::from(""),
            true => String::from("ant-badge-multiple-words"),
        },
    };
    let not_a_wrapper_class = match props.children.len() {
        0 => String::from("ant-badge-not-a-wrapper"),
        _ => String::from(""),
    };
    let sup_class = format!(
        "{} {} {} {} {}",
        dot_class, status_sup_class, count_class, scroll_class, multiple_words_class
    );
    let span_class = format!("ant-badge {} {}", status_span_class, not_a_wrapper_class);
    let dot = match &props.dot {
        None => html! {},
        Some(option_badge_status) => match option_badge_status {
            None => {
                html! { <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup> }
            }
            Some(status) => match status {
                BadgeStatus::Success(option_text) => match option_text {
                    None => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text"></span>
                        </>
                    },
                    Some(text) => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text">{text}</span>
                        </>
                    },
                },
                BadgeStatus::Processing(option_text) => match option_text {
                    None => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text"></span>
                        </>
                    },
                    Some(text) => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text">{text}</span>
                        </>
                    },
                },
                BadgeStatus::Default(option_text) => match option_text {
                    None => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text"></span>
                        </>
                    },
                    Some(text) => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text">{text}</span>
                        </>
                    },
                },
                BadgeStatus::Error(option_text) => match option_text {
                    None => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text"></span>
                        </>
                    },
                    Some(text) => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text">{text}</span>
                        </>
                    },
                },
                BadgeStatus::Warning(option_text) => match option_text {
                    None => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text"></span>
                        </>
                    },
                    Some(text) => html! {
                        <>
                          <sup data-show="true" class={sup_class.clone()} style={sup_style.clone()} title={title.clone()}></sup>
                          <span class="ant-badge-status-text">{text}</span>
                        </>
                    },
                },
            },
        },
    };
    let sup = match props.count {
        None => dot,
        Some(count) => match props.dot {
            Some(_) => dot,
            None => {
                let max_count_number = props.overflow_count.unwrap_or(99);
                let count_to_show = count.to_string();
                let max_count_number_text = format!("{}+", max_count_number);
                let should_render = match (count == 0, props.show_zero) {
                    (true, None) => false,
                    (true, Some(_)) => false,
                    (false, None) => true,
                    (false, Some(_)) => true,
                };
                let numbers = count_to_show
                    .chars()
                    .map(|char| match Numeric::try_from(char) {
                        Err(char) => {
                            error!("badge component char is not a numeric: ", char.to_string());
                            html! {}
                        }
                        Ok(numeric) => html! {
                            <BadgeNumbers numeric={numeric}/>
                        },
                    })
                    .collect::<Vec<Html>>();
                let is_max_count_number_less = count > max_count_number;
                if count == 0 && props.show_zero.is_none() {
                    html! {}
                } else if is_max_count_number_less {
                    html! {
                      <sup
                        data-show="true"
                        class={sup_class.clone()}
                        title={title.clone()}
                        style={sup_style.clone()}
                      >
                        {max_count_number_text}
                      </sup>
                    }
                } else {
                    html! {
                      <>
                        if should_render {
                          <sup
                            data-show="true"
                            class={sup_class}
                            title={title}
                            style={sup_style}
                          >
                            {for numbers}
                          </sup>
                        }
                      </>
                    }
                }
            }
        },
    };
    html! {
      <span class={span_class}>
        { for props.children.iter() }
        {sup}
        // {text_html}
      </span>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct BadgeNumbersProps {
    pub numeric: Numeric,
}

#[function_component(BadgeNumbers)]
pub fn badge_numbers(props: &BadgeNumbersProps) -> Html {
    let numbers = html! {
        <>
          <p class="ant-scroll-number-only-unit">{"0"}</p>
          <p class="ant-scroll-number-only-unit">{"1"}</p>
          <p class="ant-scroll-number-only-unit">{"2"}</p>
          <p class="ant-scroll-number-only-unit">{"3"}</p>
          <p class="ant-scroll-number-only-unit">{"4"}</p>
          <p class="ant-scroll-number-only-unit">{"5"}</p>
          <p class="ant-scroll-number-only-unit">{"6"}</p>
          <p class="ant-scroll-number-only-unit">{"7"}</p>
          <p class="ant-scroll-number-only-unit">{"8"}</p>
          <p class="ant-scroll-number-only-unit">{"9"}</p>
        </>
    };
    let current = String::from("current");
    let p_class = String::from("ant-scroll-number-only-unit");
    let mut zero_class = p_class.clone();
    let mut one_class = p_class.clone();
    let mut two_class = p_class.clone();
    let mut three_class = p_class.clone();
    let mut four_class = p_class.clone();
    let mut five_class = p_class.clone();
    let mut six_class = p_class.clone();
    let mut seven_class = p_class.clone();
    let mut eight_class = p_class.clone();
    let mut nine_class = p_class.clone();
    match props.numeric.clone() {
        Numeric::Zero => {
            zero_class = format!("{} {}", p_class, current);
        }
        Numeric::One => {
            one_class = format!("{} {}", p_class, current);
        }
        Numeric::Two => {
            two_class = format!("{} {}", p_class, current);
        }
        Numeric::Three => {
            three_class = format!("{} {}", p_class, current);
        }
        Numeric::Four => {
            four_class = format!("{} {}", p_class, current);
        }
        Numeric::Five => {
            five_class = format!("{} {}", p_class, current);
        }
        Numeric::Six => {
            six_class = format!("{} {}", p_class, current);
        }
        Numeric::Seven => {
            seven_class = format!("{} {}", p_class, current);
        }
        Numeric::Eight => {
            eight_class = format!("{} {}", p_class, current);
        }
        Numeric::Nine => {
            nine_class = format!("{} {}", p_class, current);
        }
    };
    html! {
       <span class="ant-scroll-number-only" style={format!("transition: none 0s ease 0s; transform: translateY(-1{}00%);", props.numeric.clone())}>
         {numbers.clone()}
         <p class={zero_class}>{"0"}</p>
         <p class={one_class}>{"1"}</p>
         <p class={two_class}>{"2"}</p>
         <p class={three_class}>{"3"}</p>
         <p class={four_class}>{"4"}</p>
         <p class={five_class}>{"5"}</p>
         <p class={six_class}>{"6"}</p>
         <p class={seven_class}>{"7"}</p>
         <p class={eight_class}>{"8"}</p>
         <p class={nine_class}>{"9"}</p>
         {numbers.clone()}
       </span>
    }
}
