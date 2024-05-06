pub trait ErrorLog {
    fn error_log<
        ConfigGeneric: app_state::GetSourcePlaceType
            + app_state::GetTimezone
            + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    );
}

// impl<'a, SelfGeneric> ErrorLog for SelfGeneric
// where
//     SelfGeneric: crate::to_string_with_config::ToStringWithConfig<'a>,
// {
//     fn error_log<
//         ConfigGeneric: app_state::GetSourcePlaceType
//             + app_state::GetTimezone
//             + ?Sized,
//     >(
//         &self,
//         config: &ConfigGeneric,
//     ) {
//         eprintln!("{}", self.to_string_with_config(config));
//     }
// }
