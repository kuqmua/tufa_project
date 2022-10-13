// import { placements as rcPlacements } from 'rc-tooltip/lib/placements';

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AutoAdjustOverflowHandle {
    pub adjust_x: u8,
    pub adjust_y: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AutoAdjustOverflowEnabled {
    adjust_x: u8,
    adjust_y: u8,
}

impl Default for AutoAdjustOverflowEnabled {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoAdjustOverflowEnabled {
    pub fn new() -> Self {
        AutoAdjustOverflowEnabled {
            adjust_x: 1,
            adjust_y: 1,
        }
    }
}

// const autoAdjustOverflowEnabled = {
//   adjustX: 1,
//   adjustY: 1,
// };

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AutoAdjustOverflowDisabled {
    adjust_x: u8,
    adjust_y: u8,
}
impl AutoAdjustOverflowDisabled {
    pub fn new() -> Self {
        AutoAdjustOverflowDisabled {
            adjust_x: 0,
            adjust_y: 0,
        }
    }
}

// const autoAdjustOverflowDisabled = {
//   adjustX: 0,
//   adjustY: 0,
// };

static TARGET_OFFSET: &'static [i32; 2] = &[0, 0];

// const targetOffset = [0, 0];

#[derive(Debug, PartialEq, Clone)]
pub enum ZeroOrOne {
    Zero,
    One,
}

impl ZeroOrOne {
    pub fn get_value(&self) -> u8 {
        match *self {
            ZeroOrOne::Zero => 0,
            ZeroOrOne::One => 1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdjustOverflow {
    adjust_x: Option<ZeroOrOne>,
    adjust_y: Option<ZeroOrOne>,
}

// export interface AdjustOverflow {
//   adjustX?: 0 | 1;
//   adjustY?: 0 | 1;
// }

#[derive(Debug, PartialEq, Clone)]
pub enum AdjustOverflowOrBool {
    AdjustOverflow(AdjustOverflow),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlacementsConfig {
    pub arrow_width: Option<i32>,
    pub horizontal_arrow_shift: Option<i32>,
    pub vertical_arrow_shift: Option<i32>,
    pub arrow_point_at_center: Option<bool>,
    pub auto_adjust_overflow: Option<AdjustOverflowOrBool>,
}

// export interface PlacementsConfig {
//   arrowWidth?: number;
//   horizontalArrowShift?: number;
//   verticalArrowShift?: number;
//   arrowPointAtCenter?: boolean;
//   autoAdjustOverflow?: boolean | AdjustOverflow;
// }

pub fn get_overflow_options(
    auto_adjust_overflow: AdjustOverflowOrBool,
) -> AutoAdjustOverflowHandle {
    match auto_adjust_overflow {
        AdjustOverflowOrBool::Boolean(bool_value) => match bool_value {
            true => {
                let enabled = AutoAdjustOverflowEnabled::new();
                AutoAdjustOverflowHandle {
                    adjust_x: enabled.adjust_x,
                    adjust_y: enabled.adjust_y,
                }
            }
            false => {
                let disabled = AutoAdjustOverflowDisabled::new();
                AutoAdjustOverflowHandle {
                    adjust_x: disabled.adjust_x,
                    adjust_y: disabled.adjust_y,
                }
            }
        },
        AdjustOverflowOrBool::AdjustOverflow(adjust_overflow) => {
            let disabled = AutoAdjustOverflowDisabled::new();
            let x = match adjust_overflow.adjust_x {
                Some(handle) => handle.get_value(),
                None => disabled.adjust_x,
            };
            let y = match adjust_overflow.adjust_y {
                Some(handle) => handle.get_value(),
                None => disabled.adjust_y,
            };
            AutoAdjustOverflowHandle {
                adjust_x: x,
                adjust_y: y,
            }
        }
    }
}

// export function getOverflowOptions(autoAdjustOverflow: boolean | AdjustOverflow) {
//   if (typeof autoAdjustOverflow === 'boolean') {
//     return autoAdjustOverflow ? autoAdjustOverflowEnabled : autoAdjustOverflowDisabled;
//   }
//   return {
//     ...autoAdjustOverflowDisabled,
//     ...autoAdjustOverflow,
//   };
// }

#[derive(Debug, PartialEq, Clone)]
pub struct PointsOffset {
    pub points: [PointsValue; 2],
    pub offset: [i32; 2],
    pub overflow: Option<AutoAdjustOverflowHandle>,
    pub target_offset: Option<[i32; 2]>,
    pub ignore_shake: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PointsValue {
    Cr,
    Cl,
    Bc,
    Tc,
    Bl,
    Tr,
    Br,
    Tl,
}

impl PointsValue {
    pub fn get_string(&self) -> String {
        match self {
            PointsValue::Cr => String::from("cr"),
            PointsValue::Cl => String::from("cl"),
            PointsValue::Bc => String::from("bc"),
            PointsValue::Tc => String::from("tc"),
            PointsValue::Bl => String::from("bl"),
            PointsValue::Tr => String::from("tr"),
            PointsValue::Br => String::from("br"),
            PointsValue::Tl => String::from("tl"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PositionType {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    LeftTop,
    TopRight,
    RightTop,
    BottomRight,
    RightBottom,
    BottomLeft,
    LeftBottom,
}

impl PositionType {
    pub fn get_string(&self) -> String {
        match *self {
            PositionType::Left => String::from("left"),
            PositionType::Right => String::from("right"),
            PositionType::Top => String::from("top"),
            PositionType::Bottom => String::from("bottom"),
            PositionType::TopLeft => String::from("topLeft"),
            PositionType::LeftTop => String::from("leftTop"),
            PositionType::TopRight => String::from("topRight"),
            PositionType::RightTop => String::from("rightTop"),
            PositionType::BottomRight => String::from("bottomRight"),
            PositionType::RightBottom => String::from("rightBottom"),
            PositionType::BottomLeft => String::from("bottomLeft"),
            PositionType::LeftBottom => String::from("leftBottom"),
        }
    }
}

pub fn get_placements(config_option: Option<PlacementsConfig>) -> HashMap<String, PointsOffset> {
    let config = config_option.unwrap_or(PlacementsConfig {
        arrow_width: None,
        horizontal_arrow_shift: None,
        vertical_arrow_shift: None,
        arrow_point_at_center: None,
        auto_adjust_overflow: None,
    });
    let arrow_width = match config.arrow_width {
        Some(value) => value,
        None => 5,
    };
    let horizontal_arrow_shift = match config.horizontal_arrow_shift {
        Some(value) => value,
        None => 16,
    };
    let vertical_arrow_shift = match config.vertical_arrow_shift {
        Some(value) => value,
        None => 12,
    };
    let auto_adjust_overflow = match config.auto_adjust_overflow.clone() {
        Some(value) => value,
        None => AdjustOverflowOrBool::Boolean(true),
    };
    let placement_map = HashMap::from([
        (
            PositionType::Left.get_string(),
            PointsOffset {
                points: [PointsValue::Cr, PointsValue::Cl],
                offset: [-4, 0],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::Right.get_string(),
            PointsOffset {
                points: [PointsValue::Cl, PointsValue::Cr],
                offset: [4, 0],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::Top.get_string(),
            PointsOffset {
                points: [PointsValue::Bc, PointsValue::Tc],
                offset: [0, -4],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::Bottom.get_string(),
            PointsOffset {
                points: [PointsValue::Tc, PointsValue::Bc],
                offset: [0, 4],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::TopLeft.get_string(),
            PointsOffset {
                points: [PointsValue::Bl, PointsValue::Tc],
                offset: [-(horizontal_arrow_shift + arrow_width), -4],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::LeftTop.get_string(),
            PointsOffset {
                points: [PointsValue::Tr, PointsValue::Cl],
                offset: [-4, -(vertical_arrow_shift + arrow_width)],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::TopRight.get_string(),
            PointsOffset {
                points: [PointsValue::Br, PointsValue::Tc],
                offset: [horizontal_arrow_shift + arrow_width, -4],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::RightTop.get_string(),
            PointsOffset {
                points: [PointsValue::Tl, PointsValue::Cr],
                offset: [4, -(vertical_arrow_shift + arrow_width)],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::BottomRight.get_string(),
            PointsOffset {
                points: [PointsValue::Tr, PointsValue::Bc],
                offset: [horizontal_arrow_shift + arrow_width, 4],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::RightBottom.get_string(),
            PointsOffset {
                points: [PointsValue::Bl, PointsValue::Cr],
                offset: [4, vertical_arrow_shift + arrow_width],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::BottomLeft.get_string(),
            PointsOffset {
                points: [PointsValue::Tl, PointsValue::Bc],
                offset: [-(horizontal_arrow_shift + arrow_width), 4],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
        (
            PositionType::LeftBottom.get_string(),
            PointsOffset {
                points: [PointsValue::Br, PointsValue::Cl],
                offset: [-4, vertical_arrow_shift + arrow_width],
                overflow: None,
                target_offset: None,
                ignore_shake: None,
            },
        ),
    ]);
    let mut second_placement_map =
        HashMap::<String, PointsOffset>::with_capacity(placement_map.len());
    for (key, value) in placement_map {
        match config.arrow_point_at_center {
            Some(_) => {
                second_placement_map.insert(
                    key,
                    PointsOffset {
                        points: value.points,
                        offset: value.offset,
                        overflow: Some(get_overflow_options(auto_adjust_overflow.clone())),
                        target_offset: Some(*TARGET_OFFSET),
                        ignore_shake: Some(true),
                    },
                );
            }
            None => {
                second_placement_map.insert(
                    key,
                    PointsOffset {
                        points: value.points,
                        offset: value.offset,
                        overflow: Some(get_overflow_options(auto_adjust_overflow.clone())),
                        target_offset: value.target_offset,
                        ignore_shake: Some(true),
                    },
                );
            }
        }
    }
    second_placement_map
}

// export default function getPlacements(config: PlacementsConfig = {}) {
//   const {
//     arrowWidth = 5,
//     horizontalArrowShift = 16,
//     verticalArrowShift = 12,
//     autoAdjustOverflow = true,
//   } = config;
//   const placementMap: any = {
//     left: {
//       points: ['cr', 'cl'],
//       offset: [-4, 0],
//     },
//     right: {
//       points: ['cl', 'cr'],
//       offset: [4, 0],
//     },
//     top: {
//       points: ['bc', 'tc'],
//       offset: [0, -4],
//     },
//     bottom: {
//       points: ['tc', 'bc'],
//       offset: [0, 4],
//     },
//     topLeft: {
//       points: ['bl', 'tc'],
//       offset: [-(horizontalArrowShift + arrowWidth), -4],
//     },
//     leftTop: {
//       points: ['tr', 'cl'],
//       offset: [-4, -(verticalArrowShift + arrowWidth)],
//     },
//     topRight: {
//       points: ['br', 'tc'],
//       offset: [horizontalArrowShift + arrowWidth, -4],
//     },
//     rightTop: {
//       points: ['tl', 'cr'],
//       offset: [4, -(verticalArrowShift + arrowWidth)],
//     },
//     bottomRight: {
//       points: ['tr', 'bc'],
//       offset: [horizontalArrowShift + arrowWidth, 4],
//     },
//     rightBottom: {
//       points: ['bl', 'cr'],
//       offset: [4, verticalArrowShift + arrowWidth],
//     },
//     bottomLeft: {
//       points: ['tl', 'bc'],
//       offset: [-(horizontalArrowShift + arrowWidth), 4],
//     },
//     leftBottom: {
//       points: ['br', 'cl'],
//       offset: [-4, verticalArrowShift + arrowWidth],
//     },
//   };
//   Object.keys(placementMap).forEach(key => {
//     placementMap[key] = config.arrowPointAtCenter
//       ? {
//           ...placementMap[key],
//           overflow: getOverflowOptions(autoAdjustOverflow),
//           targetOffset,
//         }
//       : {
//           ...rcPlacements[key],
//           overflow: getOverflowOptions(autoAdjustOverflow),
//         };

//     placementMap[key].ignoreShake = true;
//   });
//   return placementMap;
// }
