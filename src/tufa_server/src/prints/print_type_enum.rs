use crate::lazy_static::config::CONFIG;
use ansi_term::Colour;
use ansi_term::Colour::RGB;
use tufa_common::traits::print_type_trait::PrintTypeTrait;

pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
    TimeMeasurement,
    Info,
}

impl PrintTypeTrait for PrintType {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn is_prints_enabled(&self) -> bool {
        match *self {
            PrintType::Error => CONFIG.is_error_prints_enabled,
            PrintType::WarningHigh => CONFIG.is_warning_high_prints_enabled,
            PrintType::WarningLow => CONFIG.is_warning_low_prints_enabled,
            PrintType::Success => CONFIG.is_success_prints_enabled,
            PrintType::PartialSuccess => CONFIG.is_partial_success_prints_enabled,
            PrintType::TimeMeasurement => CONFIG.is_time_measurement_prints_enabled,
            PrintType::Info => CONFIG.is_info_prints_enabled,
        }
    }
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn get_color(&self) -> Colour {
        match *self {
            PrintType::Error => RGB(CONFIG.error_red, CONFIG.error_green, CONFIG.error_blue),
            PrintType::WarningHigh => RGB(
                CONFIG.warning_high_red,
                CONFIG.warning_high_green,
                CONFIG.warning_high_blue,
            ),
            PrintType::WarningLow => RGB(
                CONFIG.warning_low_red,
                CONFIG.warning_low_green,
                CONFIG.warning_low_blue,
            ),
            PrintType::Success => RGB(
                CONFIG.success_red,
                CONFIG.success_green,
                CONFIG.success_blue,
            ),
            PrintType::PartialSuccess => RGB(
                CONFIG.partial_success_red,
                CONFIG.partial_success_green,
                CONFIG.partial_success_blue,
            ),
            PrintType::TimeMeasurement => RGB(
                CONFIG.time_measurement_red,
                CONFIG.time_measurement_green,
                CONFIG.time_measurement_blue,
            ),
            PrintType::Info => RGB(CONFIG.info_red, CONFIG.info_green, CONFIG.info_blue),
        }
    }
}
