use crate::components::ant_design::svg::helpers::fill_with::FillWith;
use crate::components::ant_design::svg::helpers::get_color::get_color;
use crate::components::ant_design::svg::helpers::svg_props::SvgProps;
use crate::components::ant_design::svg::helpers::svg_wrapper::SvgWrapper;
use crate::components::ant_design::svg::helpers::theme::Theme;
use yew::{function_component, html};

#[function_component(ExclamationCircle)]
pub fn exclamation_circle(props: &SvgProps) -> Html {
    let outlined = html! {
        <>
          <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z">
          </path>
          <path d="M464 688a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm24-112h48c4.4 0 8-3.6 8-8V296c0-4.4-3.6-8-8-8h-48c-4.4 0-8 3.6-8 8v272c0 4.4 3.6 8 8 8z">
          </path>
        </>
    };
    let fill = get_color(props.fill.clone());
    let inner_html = match &props.theme {
        None => outlined,
        Some(theme) => match theme {
            Theme::Outlined => outlined,
            Theme::Filled => html! {
              <path d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm-32 232c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V296zm32 440a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z">
              </path>
            },
            Theme::TwoTone => {
                match props.fill.clone() {
                    //second time match. maybe rewrite somehow
                    None => html! {
                        <>
                          <path fill={fill.into_string_color()} d="M512 64C264.6 64 64 264.6 64 512s200.6 448 448 448 448-200.6 448-448S759.4 64 512 64zm0 820c-205.4 0-372-166.6-372-372s166.6-372 372-372 372 166.6 372 372-166.6 372-372 372z">
                          </path>
                          <path fill={fill.into_string_color()} d="M512 140c-205.4 0-372 166.6-372 372s166.6 372 372 372 372-166.6 372-372-166.6-372-372-372zm32 588c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V456c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272zm-32-344a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z">
                          </path>
                          <path fill={fill.into_string_color()} d="M464 336a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm72 112h-48c-4.4 0-8 3.6-8 8v272c0 4.4 3.6 8 8 8h48c4.4 0 8-3.6 8-8V456c0-4.4-3.6-8-8-8z">
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
                            <path fill={second_color} d="M512 140c-205.4 0-372 166.6-372 372s166.6 372 372 372 372-166.6 372-372-166.6-372-372-372zm32 588c0 4.4-3.6 8-8 8h-48c-4.4 0-8-3.6-8-8V456c0-4.4 3.6-8 8-8h48c4.4 0 8 3.6 8 8v272zm-32-344a48.01 48.01 0 0 1 0-96 48.01 48.01 0 0 1 0 96z">
                            </path>
                            <path fill={fill.into_string_color()} d="M464 336a48 48 0 1 0 96 0 48 48 0 1 0-96 0zm72 112h-48c-4.4 0-8 3.6-8 8v272c0 4.4 3.6 8 8 8h48c4.4 0 8-3.6 8-8V456c0-4.4-3.6-8-8-8z">
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
