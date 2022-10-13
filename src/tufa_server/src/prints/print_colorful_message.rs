use super::print_wrapper::print_wrapper;
use crate::lazy_static::config::CONFIG;
use crate::prints::print_type_enum::PrintType;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfigTrait;
use tufa_common::traits::print_type_trait::PrintTypeTrait;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn print_colorful_message(
    pk: Option<&ProviderKind>,
    pt: PrintType,
    sources: Vec<String>,
    github_sources: Vec<String>,
    message: String,
) {
    if CONFIG.is_prints_enabled {
        let mut sources_track = String::from("");
        for source in sources {
            sources_track.push_str(&format!("{source}\n"))
        }
        let mut github_sources_track = String::from("");
        for github_source in github_sources {
            github_sources_track.push_str(&format!("{github_source}\n"))
        }
        match pk {
            Some(pk) => {
                if pk.is_prints_enabled() && pk.is_prints_for_print_type_enabled(&pt) {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
            None => {
                if pt.is_prints_enabled() {
                    print_wrapper(pt.get_color(), sources_track, github_sources_track, message);
                }
            }
        }
    }
}
