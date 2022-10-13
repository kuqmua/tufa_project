use super::interface::{StrokeColor, StrokeLinecapType};
use crate::components::rc::rc_progress::hooks::use_id::use_id;
use crate::components::rc::rc_progress::interface::GapPositionType;
use crate::components::rc::rc_progress::interface::Percent;
use crate::components::rc::rc_progress::interface::ProgressProps;
use std::fmt;
use yew::Callback;
use yew::Html;
use yew::{function_component, html};

pub fn strip_percent_to_number(percent: String) -> String {
    percent.replace('%', "")
}

pub fn percent_to_array(value: Percent) -> Vec<f64> {
    match value {
        Percent::Number(n) => vec![n],
        Percent::NumberVec(vec) => vec,
    }
}

pub const VIEW_BOX_SIZE: f64 = 100.0;

#[derive(Debug, PartialEq, Clone)]
pub struct CircleStyle {
    pub stroke: Option<String>,
    pub stroke_dash_array: String,
    pub stroke_dash_offset: f64,
    pub transform: String,
    pub transform_origin: String,
    pub transition: String,
    pub fill_opacity: f64,
}

impl fmt::Display for CircleStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "stroke: {}; stroke-dasharray: {}; stroke-dashoffset: {}; transform: {}; transform-origin: {}; transition: {}; fill-opacity: {};",
            self.stroke.clone().unwrap_or_else(|| String::from("")),
            self.stroke_dash_array,
            self.stroke_dash_offset,
            self.transform,
            self.transform_origin,
            self.transition,
            self.fill_opacity,
        )
    }
}

#[allow(clippy::too_many_arguments)]
pub fn get_circle_style(
    perimeter: f64,
    perimeter_without_gap: f64,
    offset: f64,
    percent: f64,
    rotate_deg: f64,
    gap_degree: f64,
    gap_position: GapPositionType,
    stroke_color: String,
    stroke_linecap: StrokeLinecapType,
    stroke_width: f64,
    step_space: Option<f64>,
) -> CircleStyle {
    let step_space = step_space.unwrap_or(0.0);
    let offset_deg = (offset / 100.0) * 360.0 * ((360.0 - gap_degree) / 360.0);
    let position_deg = if gap_degree == 0.0 {
        0.0
    } else {
        match gap_position {
            GapPositionType::Top => 0.0,
            GapPositionType::Right => 180.0,
            GapPositionType::Bottom => 90.0,
            GapPositionType::Left => -90.0,
        }
    };
    let mut stroke_dash_offset = ((100.0 - percent) / 100.0) * perimeter_without_gap;
    if let StrokeLinecapType::Round = stroke_linecap {
        if percent != 100.0 {
            stroke_dash_offset += stroke_width / 2.0;
            if stroke_dash_offset >= perimeter_without_gap {
                stroke_dash_offset = perimeter_without_gap - 0.01;
            }
        }
    }
    CircleStyle {
      stroke: Some(stroke_color),
      stroke_dash_array: format!("{}px {}", perimeter_without_gap, perimeter),
      stroke_dash_offset: stroke_dash_offset + step_space,
      transform: format!("rotate({}deg)", rotate_deg + offset_deg + position_deg),
      transform_origin: String::from("50% 50%"),
      transition: String::from("stroke-dashoffset .3s ease 0s, stroke-dasharray .3s ease 0s, stroke .3s, stroke-width .06s ease .3s, opacity .3s ease 0s"),
      fill_opacity: 0.0
    }
}

#[function_component(Circle)]
pub fn circle(props: &ProgressProps) -> Html {
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
        None => StrokeColor {
                colors: vec![String::from("#2db7f5")],
            },
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
    let gap_degree = props.gap_degree.unwrap_or(0.0);
    let trail_width = props.trail_width.unwrap_or(1.0);
    let gap_position = match props.gap_position.clone() {
        Some(gp) => gp,
        None => GapPositionType::Bottom,
    };
    let gradient = props.gradient.clone().unwrap_or_default();
    let merged_id = use_id(props.id.clone());
    let gradient_id = format!("{}-gradient", merged_id);
    let radius = VIEW_BOX_SIZE / 2.0 - stroke_width / 2.0;
    let perimeter = std::f64::consts::PI * 2.0 * radius;
    let rotate_deg = match gap_degree > 0.0 {
        true => 90.0 + gap_degree / 2.0,
        false => -90.0,
    };
    let perimeter_without_gap = perimeter * ((360.0 - gap_degree) / 360.0);
    let (step_count, step_space) = match props.steps.clone() {
        None => (0.0, 0.0),
        Some(steps_type) => match steps_type {
            super::interface::Steps::Number(n) => (n, 2.0),
            super::interface::Steps::CountSpace(count_space) => {
                (count_space.count, count_space.space)
            }
        },
    };
    let circle_style = get_circle_style(
        perimeter,
        perimeter_without_gap,
        0.0,
        100.0,
        rotate_deg,
        gap_degree,
        gap_position.clone(),
        trail_color.clone(),
        stroke_linecap.clone(),
        stroke_width,
        None,
    );
    let percent_list = percent_to_array(percent);
    let stroke_color_list = stroke_color.colors;
    
    // for color in stroke_color_list.clone() {
    //     match color {
    //         BaseStrokeColorType::String(_) => (),
    //         BaseStrokeColorType::Record(hm) => {
    //             gradient = Some(hm);
    //             break;
    //         }
    //     }
    // }
    // let paths = use_transition_duration();
    let get_stoke_list = || {
        let mut stack_ptg = 0.0;
        let mut percent_list_cloned = percent_list.clone();
        percent_list_cloned.reverse();
        let html_list = percent_list_cloned.clone().iter().enumerate().map(|(index, ptg)| {
          let color = match (stroke_color_list.clone().get(index), stroke_color_list.last()) {
            (None, None) => None,
            (None, Some(c)) => Some(c.clone()),
            (Some(c), None) => Some(c.clone()),
            (Some(c), Some(_)) => Some(c.clone()),
          };
          let stroke = color.clone().map(|_| format!("url(#{})", gradient_id));
          let color_handle = color.unwrap_or_else(|| String::from("#2db7f5"));
          let circle_style_for_stack = get_circle_style(
            perimeter,
            perimeter_without_gap,
            stack_ptg,
            *ptg,
            rotate_deg,
            gap_degree,
            gap_position.clone(),
            color_handle,
            stroke_linecap.clone(),
            stroke_width,
            None
          );
          stack_ptg += ptg;
          let opacity = if *ptg == 0.0 {
            "0"
          } 
          else {
            "1"
          };
          html!{
            <circle
              key={index}
              class={format!("{}-circle-path", prefix_cls)}
              r={radius.to_string()}
              cx={(VIEW_BOX_SIZE / 2.0).to_string()}
              cy={(VIEW_BOX_SIZE / 2.0).to_string()}
              stroke={stroke}
              stroke-linecap={props.stroke_linecap.clone().unwrap_or(StrokeLinecapType::Round).get_value()}
              stroke-width={stroke_width.to_string()}
              opacity={opacity}
              style={circle_style_for_stack.to_string()}
              // ref={(elem) => {
              //   // https://reactjs.org/docs/refs-and-the-dom.html#callback-refs
              //   // React will call the ref callback with the DOM element when the component mounts,
              //   // and call it with `null` when it unmounts.
              //   // Refs are guaranteed to be up-to-date before componentDidMount or componentDidUpdate fires.

              //   paths[index] = elem;
              // }}
            />
          }
        }).collect::<Vec<Html>>();
        html_list
    };

    let get_step_stoke_list = || {
        let percent_list_cloned = percent_list.clone();
        let current = step_count * percent_list_cloned[0] / 100.0;
        let step_ptg = 100.0 / step_count;
        let mut stack_ptg = 0.0;
        let html_vec: Vec<Html> = vec![html! {}; step_count as usize];
        let bbn = html_vec
            .iter()
            .enumerate()
            .map(|(index, _element)| {
                let index_as_f64 = index as f64;
                let _color = match index_as_f64 < current {
                    true => stroke_color_list[0].clone(),
                    false => trail_color.clone(),
                };//todo
                let stroke = format!("url(#{})", gradient_id);//for gradient
                let circle_style_for_stack = get_circle_style(
                    perimeter,
                    perimeter_without_gap,
                    stack_ptg,
                    step_ptg,
                    rotate_deg,
                    gap_degree,
                    gap_position.clone(),
                    trail_color.clone(),//todo
                    StrokeLinecapType::Butt,
                    stroke_width,
                    Some(step_space),
                );
                stack_ptg += ((perimeter_without_gap - circle_style_for_stack.stroke_dash_offset
                    + step_space)
                    * 100.0)
                    / perimeter_without_gap;
                html! {
                    <circle
                      key={index}
                      class={format!("{}-circle-path", prefix_cls)}
                      r={radius.to_string()}
                      cx={(VIEW_BOX_SIZE / 2.0).to_string()}
                      cy={(VIEW_BOX_SIZE / 2.0).to_string()}
                      stroke={stroke}
                      stroke-width={stroke_width.to_string()}
                      opacity={1}
                      style={circle_style_for_stack.to_string()}
                      //   ref={(elem) => {
                      //     paths[index] = elem;
                      //   }}
                    />
                }
            })
            .collect::<Vec<Html>>();
        bbn
    };
    let linear_gradient =  {
        let mut prep_vec = gradient
                .into_iter()
                .map(|(key, value)| (key, value))
                .collect::<Vec<(u32, String)>>();
            prep_vec.sort_by(|(key1, _), (key2, _)| key2.cmp(key1));
            let inner_content = prep_vec
                .iter()
                .enumerate()
                .map(|(index, (key, value))| {
                    html! {
                      <stop
                        key={index}
                        offset={key.clone().to_string()}
                        stop_color={value.clone()}
                      />
                    }
                })
                .collect::<Html>();
            html! {
                <defs>
                  <linear_gradient
                    id={gradient_id.clone()}
                    x1={String::from("100%")}
                    y1={String::from("0%")}
                    x2={String::from("0%")}
                    y2={String::from("0%")}
                  >
                  {inner_content}
                  </linear_gradient>
                </defs>
            }
    };
    let stroke_width_common = if trail_width == 0.0 && stroke_width == 0.0 {
        String::from("0")
    } else if stroke_width == 0.0 {
        trail_width.to_string()
    } else if trail_width == 0.0 {
        stroke_width.to_string()
    } else {
        trail_width.to_string()
    };
    let stroke_list = get_stoke_list();
    let step_stroke_list = get_step_stoke_list();
    html! {
        <svg
          class={format!("{}-circle {}", prefix_cls, class_name)}
          viewBox={format!("0 0 {} {}", VIEW_BOX_SIZE, VIEW_BOX_SIZE)}
          style={style}
          id={props.id.clone()}
          transition={props.transition.clone()}
          onclick={props.on_click.clone().unwrap_or_else(|| Callback::from(|_|{}))}
        >
          {linear_gradient}
          if step_count == 0.0 {
            <circle
              class={format!("{}-circle-trail", prefix_cls)}
              r={radius.to_string()}
              cx={(VIEW_BOX_SIZE / 2.0).to_string()}
              cy={(VIEW_BOX_SIZE / 2.0).to_string()}
              stroke={trail_color}
              stroke-linecap={props.stroke_linecap.clone().unwrap_or(StrokeLinecapType::Round).get_value()}
              stroke-width={stroke_width_common}
              style={circle_style.to_string()}
            />
            {stroke_list}
          }
          else{
            {step_stroke_list}
          }
        </svg>
    }
}
// Circle.displayName = 'Circle';
