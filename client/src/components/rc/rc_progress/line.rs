use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
// use crate::components::rc::rc_progress::common::use_transition_duration;
use super::interface::Percent;
use super::interface::StrokeLinecapType;
use crate::components::rc::rc_progress::interface::ProgressProps;
use crate::components::rc::rc_progress::interface::StrokeColor;

#[function_component(Line)]
pub fn line(props: &ProgressProps) -> Html {
    let class_name = match props.class_name.clone() {
        Some(cn) => cn,
        None => String::from(""),
    };
    let percent = match props.percent.clone() {
        Some(p) => p,
        None => Percent::Number(0.0),
    };
    let prefix_cls = match props.prefix_cls.clone() {
        Some(pc) => pc,
        None => String::from("rc-progress"),
    };
    let stroke_color = match props.stroke_color.clone() {
        Some(sc) => sc,
        None => StrokeColor { colors: vec![String::from("#2db7f5")] },
    };
    let stroke_linecap = match props.stroke_linecap.clone() {
        Some(sl) => sl,
        None => StrokeLinecapType::Round,
    };
    let stroke_width = props.stroke_width.unwrap_or(1.0);
    let style = match props.style.clone() {
        Some(s) => s,
        None => String::from(""),
    };
    let trail_color = match props.trail_color.clone() {
        Some(tc) => tc,
        None => String::from("#D9D9D9"),
    };
    let trail_width = props.trail_width.unwrap_or(1.0);
    let mut default_props = props.default();
    default_props.gap_position = None;
    let percent_list = match percent {
        super::interface::Percent::Number(n) => vec![n],
        super::interface::Percent::NumberVec(vec) => vec,
    };
    let stroke_color_list = stroke_color.colors;
    // let paths = use_transition_duration();
    let center = stroke_width / 2.0;
    let right = 100.0 - stroke_width / 2.0;
    let first_part = match stroke_linecap {
        super::interface::StrokeLinecapType::Round => center,
        super::interface::StrokeLinecapType::Butt => 0.0,
        super::interface::StrokeLinecapType::Square => 0.0,
    };
    let second_part = match stroke_linecap {
        super::interface::StrokeLinecapType::Round => right,
        super::interface::StrokeLinecapType::Butt => 100.0,
        super::interface::StrokeLinecapType::Square => 100.0,
    };
    let path_string = format!("M {}{} L {}{}", first_part, center, second_part, center);
    let view_box_string = format!("0 0 100 {}", stroke_width);
    let mut stack_ptg = 0.0;
    let gap_position = match props.clone().gap_position {
        None => String::from("bottom"),
        Some(gp) => gp.get_value(),
    };
    let on_click = match props.clone().on_click {
        None => Callback::from(|_: MouseEvent| {}),
        Some(oc) => oc,
    };
    let percent_list_mapped = percent_list.into_iter().enumerate().map(|(index, ptg)| {
        let stroke_width = props.clone().stroke_width.unwrap_or(1.0);
        let dash_percent = 
                    
                    match stroke_linecap {
                        super::interface::StrokeLinecapType::Round => {
                            1.0 - stroke_width / 100.0
                        },
                        super::interface::StrokeLinecapType::Butt => {
                            1.0 - stroke_width / 100.0
                        },
                        super::interface::StrokeLinecapType::Square => {
                            1.0 - stroke_width / 100.0
                        },
                    };
        let stroke_dash_array =  format!("{}px, 100px", ptg * dash_percent);
        let stroke_dash_offset = format!("-{}px", stack_ptg);
        let transition = match props.transition.clone() {
            None => String::from("stroke-dashoffset 0.3s ease 0s, stroke-dasharray .3s ease 0s, stroke 0.3s linear"),
            Some(t) => t,
        };
        let path_style = format!("
            stroke_dash_array: {};
            stroke_dash_offset: {};
            transition: {};
        ",
          stroke_dash_array, 
          stroke_dash_offset, 
          transition
        );
        let color = match stroke_color_list.get(index) {
            None => match stroke_color_list.is_empty() {
                true => String::from("#2db7f5"),//default
                false => match stroke_color_list.last() {
                    None => String::from("#2db7f5"),//default
                    Some(element) => element.to_string(),
                },
            },
            Some(element) => element.to_string(),
        };
        stack_ptg += ptg;
        html!{
            <path
                key={index}
                class={format!("{}-line-path", prefix_cls)}
                d={path_string.clone()}
                stroke_linecap={stroke_linecap.clone().get_value()}
                stroke={color.clone()}
                stroke_width={stroke_width.clone().to_string()}
                fill_opacity="0"
                // ref={|elem} -> {
                //   // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
                //   // React will call the ref callback with the DOM element when the component mounts,
                //   // and call it with `null` when it unmounts.
                //   // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

                //   paths[index] = elem;
                // }}
                style={path_style.clone()}
            />
        }
    }).collect::<Vec<Html>>();
    let stroke_width_common = if trail_width == 0.0 && stroke_width == 0.0 {
        String::from("0")
    } else if stroke_width == 0.0 {
        trail_width.to_string()
    } else if trail_width == 0.0 {
        stroke_width.to_string()
    } else {
        trail_width.to_string()
    };
    html! {
        <svg
          class={format!("{}-line {}", prefix_cls, class_name)}
          viewBox={view_box_string}
          preserve-aspect-ratio="none"
          style={style}
          id={props.clone().id}
        //   gap_degree={props.clone().gap_degree}//todo
          gap-position={gap_position}
          onclick={on_click}
        //   steps={props.clone().steps}//todo
        >
          <path
            class={format!("{}-line-trail", prefix_cls)}
            d={path_string}
            stroke_linecap={stroke_linecap.clone().get_value()}
            stroke={trail_color}
            stroke-width={stroke_width_common}
            fill-opacity="0"
          />
          {percent_list_mapped}
        </svg>
    }
}
// Line.displayName = 'Line';
