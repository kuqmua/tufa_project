// let links_len = links_temp_naming.len();

////this logic was removed for refactoring reasons. maybe rewrite it but not here
// let twitter_available_providers_links: Vec<String>;
// match option_twitter_providers_names {
//     Some(twitter_providers_names) => {
//         twitter_available_providers_links =
//             rss_check_available_providers(twitter_providers_names);
//         let vec_of_hashmap_parts = rss_divide_to_equal_for_each_provider(
//             twitter_available_providers_links,
//             links_temp_naming,
//             links_len,
//         );
//         let not_ready_processed_posts =
//             Arc::new(Mutex::new(Vec::with_capacity(links_len)));
//         let mut threads_vector = Vec::with_capacity(vec_of_hashmap_parts.len());
//         let mut threads_vec_checker =
//             Vec::<bool>::with_capacity(vec_of_hashmap_parts.len());
//         for element in &mut vec_of_hashmap_parts.into_iter() {
//             let not_ready_processed_posts_handle =
//                 Arc::clone(&not_ready_processed_posts);
//             let provider_kind_clone = provider_kind;
//             let thread = thread::spawn(move || {
//                 let unfiltered_posts_vec_after_fetch_and_parse =
//                     rss_fetch_and_parse_provider_data(
//                         element.clone(),
//                         provider_kind_clone,
//                     );
//                 match not_ready_processed_posts_handle.lock() {
//                     Ok(mut locked_not_ready_processed_posts) => {
//                         for unfiltered_post in
//                             unfiltered_posts_vec_after_fetch_and_parse
//                         {
//                             locked_not_ready_processed_posts.push(unfiltered_post);
//                         }
//                     }
//                     Err(e) => {
//                         print_colorful_message(
//                                 None,
//                                 PrintType::Error,
//                                 file!().to_string(),
//                                 line!().to_string(),
//                                 format!("not_ready_processed_posts_handle.lock() error: {e:#?}"),
//                             );
//                     }
//                 }
//             });
//             threads_vector.push(thread);
//         }
//         for thread in threads_vector {
//             match thread.join() {
//                 Ok(_) => threads_vec_checker.push(true),
//                 Err(e) => {
//                     threads_vec_checker.push(false);
//                     print_colorful_message(
//                         None,
//                         PrintType::Error,
//                         file!().to_string(),
//                         line!().to_string(),
//                         format!("thread.join() error: {e:#?}"),
//                     );
//                 }
//             }
//         }
//         let is_all_elelements_false =
//             &threads_vec_checker.iter().all(|&item| !item);
//         if *is_all_elelements_false {
//             print_colorful_message(
//                             None,
//                             PrintType::Error,
//                             file!().to_string(),
//                             line!().to_string(),
//                             "is_all_elelements_false for threads_vec_checker in twitter_available_providers_links".to_string(),
//                         );
//             return (None, None);
//         } else {
//             match not_ready_processed_posts.lock() {
//                 Ok(not_ready_processed_posts_locked) => {
//                     unfiltered_posts_vec_after_fetch_and_parse =
//                         not_ready_processed_posts_locked.to_vec();
//                 }
//                 Err(e) => {
//                     print_colorful_message(
//                         None,
//                         PrintType::Error,
//                         file!().to_string(),
//                         line!().to_string(),
//                         format!("not_ready_processed_posts.lock() error: {e:#?}"),
//                     );
//                     return (None, None);
//                 }
//             }
//         }
//     }
//     None => {
//         unfiltered_posts_vec_after_fetch_and_parse = Vec::new();
//         print_colorful_message(
//             Some(&provider_kind),
//             PrintType::WarningHigh,
//             file!().to_string(),
//             line!().to_string(),
//             "option_twitter_providers_names is None for Twitter".to_string(),
//         );
//     }
// }
