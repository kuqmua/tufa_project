// use crate::authorization::reddit::reddit_authorization;
// use crate::lazy_static::config::CONFIG;

// //what should i do with authorization?
// let is_reddit_authorized = reddit_authorization::reddit_authorization(
//     &CONFIG.reddit_authorization.reddit_user_agent,
//     &CONFIG.reddit_authorization.reddit_client_id,
//     &CONFIG.reddit_authorization.reddit_client_secret,
//     &CONFIG.reddit_authorization.reddit_username,
//     &CONFIG.reddit_authorization.reddit_password,
// );
// if is_reddit_authorized {
//     print_colorful_message(
//         Some(&provider_kind),
//         PrintType::Success,
//         file!().to_string(),
//         line!().to_string(),
//         "success reddit authorization".to_string(),
//     );
//     unfiltered_posts_vec_after_fetch_and_parse =
//         rss_fetch_and_parse_provider_data(links_temp_naming, provider_kind);
// } else {
//     unfiltered_posts_vec_after_fetch_and_parse = Vec::new(); //rethink this
//     print_colorful_message(
//                 Some(&provider_kind),
//                 PrintType::Error,
//                 file!().to_string(),
//                 line!().to_string(),
//                 "cannot authorize reddit(cannot put here authorization_info for future security reasons".to_string(),
//             );
// }
