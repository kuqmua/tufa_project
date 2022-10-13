use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::get_color::get_color;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_wrapper::SvgWrapper;
use crate::components::ant_design::svg::helpers::theme::Theme;
use yew::{function_component, html};

#[function_component(CheckCircle)]
pub fn check_circle(props: &SvgProps) -> Html {
    let outlined = html! {
        <>
          <path d="M699 353h-46.9c-10.2 0-19.9 4.9-25.9 13.3L469 584.3l-71.2-98.8c-6-8.3-15.6-13.3-25.9-13.3H325c-6.5 0-10.3 7.4-6.5 12.7l124.6 172.8a31.8 31.8 0 0 0 51.7 0l210.6-292c3.9-5.3.1-12.7-6.4-12.7z">
          </path>
          <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z">
          </path>
        </>
    };
    let fill = get_color(props.fill.clone());
    let inner_html = match &props.theme {
        None => outlined,
        Some(theme) => match theme {
            Theme::Outlined => outlined,
            Theme::Filled => html! {
              <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm193.5 301.7l-210.6 292a31.8 31.8 0 0 1-51.7 0L318.5 484.9c-3.8-5.3 0-12.7 6.5-12.7h46.9c10.2 0 19.9 4.9 25.9 13.3l71.2 98.8 157.2-218c6-8.3 15.6-13.3 25.9-13.3H699c6.5 0 10.3 7.4 6.5 12.7z">
              </path>
            },
            Theme::TwoTone => {
                match props.fill.clone() {
                    //second time match. maybe rewrite somehow
                    None => html! {
                        <>
                          <path fill={fill.into_string_color()} d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z">
                          </path>
                          <path fill={fill.into_string_color()} d="M512 140c-205.4 0-372 166.6-372 372s166.6 372 372 372 372-166.6 372-372-166.6-372-372-372zm193.4 225.7l-210.6 292a31.8 31.8 0 0 1-51.7 0L318.5 484.9c-3.8-5.3 0-12.7 6.5-12.7h46.9c10.3 0 19.9 5 25.9 13.3l71.2 98.8 157.2-218c6-8.4 15.7-13.3 25.9-13.3H699c6.5 0 10.3 7.4 6.4 12.7z">
                          </path>
                          <path fill={fill.into_string_color()} d="M699 353h-46.9c-10.2 0-19.9 4.9-25.9 13.3L469 584.3l-71.2-98.8c-6-8.3-15.6-13.3-25.9-13.3H325c-6.5 0-10.3 7.4-6.5 12.7l124.6 172.8a31.8 31.8 0 0 0 51.7 0l210.6-292c3.9-5.3.1-12.7-6.4-12.7z">
                          </path>
                        </>
                    },
                    Some(fill) => {
                        let second_color = match fill.clone() {
                            FillWith::Hsl(fill) => {
                                format!("hsl({}, {}%, 95%)", fill.hue(), fill.saturation())
                            }
                            FillWith::CurrentColor => String::from("currentColor"), //what should i do with currect color?
                        };
                        html! {
                          <>
                            <path fill={fill.into_string_color()} d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z">
                            </path>
                            <path fill={second_color} d="M512 140c-205.4 0-372 166.6-372 372s166.6 372 372 372 372-166.6 372-372-166.6-372-372-372zm193.4 225.7l-210.6 292a31.8 31.8 0 0 1-51.7 0L318.5 484.9c-3.8-5.3 0-12.7 6.5-12.7h46.9c10.3 0 19.9 5 25.9 13.3l71.2 98.8 157.2-218c6-8.4 15.7-13.3 25.9-13.3H699c6.5 0 10.3 7.4 6.4 12.7z">
                            </path>
                            <path fill={fill.into_string_color()} d="M699 353h-46.9c-10.2 0-19.9 4.9-25.9 13.3L469 584.3l-71.2-98.8c-6-8.3-15.6-13.3-25.9-13.3H325c-6.5 0-10.3 7.4-6.5 12.7l124.6 172.8a31.8 31.8 0 0 0 51.7 0l210.6-292c3.9-5.3.1-12.7-6.4-12.7z">
                            </path>
                          </>
                        }
                    }
                }
            }
        },
    };
    html! {
      <SvgWrapper
        width={props.width.clone()}
        height={props.height.clone()}
        fill={fill}
        spin={props.spin}
        rotate={props.rotate.clone()}
        inner_html={inner_html}
      />
    }
}
