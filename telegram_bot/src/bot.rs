#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    // let id  = "b129a42c-5f99-4a3e-ab25-154f3549f638";
    let api_location = std::string::String::from("http://127.0.0.1:8080");
    let limit = 1000;
    let offset = 0;
    //
    println!("--------------try_create_many-----------------");//todo add try_create_many
    let primary_keys = match common::repositories_types::tufa_server::routes::api::cats::try_create_many(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::CreateManyParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::CreateManyPayload(vec![
                common::repositories_types::tufa_server::routes::api::cats::CreateManyPayloadElement{
                    name: std::string::String::from("try_create_many_name1"),
                    color: std::string::String::from("try_create_many_color1"),
                },
                common::repositories_types::tufa_server::routes::api::cats::CreateManyPayloadElement{
                    name: std::string::String::from("try_create_many_name2"),
                    color: std::string::String::from("try_create_many_color2"),
                },
            ])
        },
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            value
        },
        Err(e) => {
            panic!("{e}");
        }
    };
    println!("--------------try_read_many-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_read_many(
        &api_location,
        //todo - builder pattern?
        common::repositories_types::tufa_server::routes::api::cats::ReadManyParameters{ 
            payload: common::repositories_types::tufa_server::routes::api::cats::ReadManyPayload { 
                select: common::repositories_types::tufa_server::routes::api::cats::DogColumnSelect::IdNameColor,
                id: Some(
                    primary_keys.clone()
                    // vec![
                    //     common::server::postgres::uuid_wrapper::UuidWrapper::try_from(
                    //         common::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(id)
                    //     ).unwrap()
                    // ]
                ),
                name: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,//or and support
                color: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,
                order_by: common::server::postgres::order_by::OrderBy {
                    column: common::repositories_types::tufa_server::routes::api::cats::DogColumn::Name,
                    order: Some(common::server::postgres::order::Order::Desc),
                },
                limit: common::server::postgres::postgres_bigint::PostgresBigint(limit),
                offset: common::server::postgres::postgres_bigint::PostgresBigint(offset),
            } 
        },
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            // let vec_cat_id: Vec<
            //     common::repositories_types::tufa_server::routes::api::cats::DogId,
            // > = value
            //     .into_iter()
            //     .filter_map(|value| match value.id {
            //         Some(id) => Some(
            //             common::repositories_types::tufa_server::routes::api::cats::DogId {
            //                 id,
            //             },
            //         ),
            //         None => None,
            //     })
            //     .collect();
            // println!("{vec_cat_id:#?}");
        }
        Err(e) => {
            panic!("{e}");
        }
    }
    println!("--------------try_update_many------------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_update_many(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::UpdateManyParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::UpdateManyPayload(
                primary_keys.clone().into_iter().map(|element| {
                    common::repositories_types::tufa_server::routes::api::cats::UpdateManyPayloadElement {
                        id: element,  
                        name: std::string::String::from("name"), //todo make sure name and color both are not None(make it option<value>, not just a value)
                        color: std::string::String::from("color"), 
                    }
                }).collect()
            )
        }
    )
    .await
    {
        Ok(value) => println!("{value:#?}"),
        Err(e) => {
            panic!("{e}");
        },
    }
    println!("--------------try_read_many-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_read_many(
        &api_location,
        //todo - builder pattern?
        common::repositories_types::tufa_server::routes::api::cats::ReadManyParameters{ 
            payload: common::repositories_types::tufa_server::routes::api::cats::ReadManyPayload { 
                select: common::repositories_types::tufa_server::routes::api::cats::DogColumnSelect::IdNameColor,
                id: Some(
                    primary_keys.clone()
                    // vec![
                    //     common::server::postgres::uuid_wrapper::UuidWrapper::try_from(
                    //         common::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(id)
                    //     ).unwrap()
                    // ]
                ),
                name: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,//or and support
                color: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,
                order_by: common::server::postgres::order_by::OrderBy {
                    column: common::repositories_types::tufa_server::routes::api::cats::DogColumn::Name,
                    order: Some(common::server::postgres::order::Order::Desc),
                },
                limit: common::server::postgres::postgres_bigint::PostgresBigint(limit),
                offset: common::server::postgres::postgres_bigint::PostgresBigint(offset),
            } 
        },
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            // let vec_cat_id: Vec<
            //     common::repositories_types::tufa_server::routes::api::cats::DogId,
            // > = value
            //     .into_iter()
            //     .filter_map(|value| match value.id {
            //         Some(id) => Some(
            //             common::repositories_types::tufa_server::routes::api::cats::DogId {
            //                 id,
            //             },
            //         ),
            //         None => None,
            //     })
            //     .collect();
            // println!("{vec_cat_id:#?}");
        }
        Err(e) => {
            panic!("{e}");
        }
    }
    println!("--------------try_delete_many-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_delete_many(
        &api_location,
        //todo - builder pattern?
        common::repositories_types::tufa_server::routes::api::cats::DeleteManyParameters{ 
            payload: common::repositories_types::tufa_server::routes::api::cats::DeleteManyPayload { 
                id: Some(
                    primary_keys.clone()
                    // vec![
                    //     common::server::postgres::uuid_wrapper::UuidWrapper::try_from(
                    //         common::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(id)
                    //     ).unwrap()
                    // ]
                ),
                name: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,//or and support
                color: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,
            } 
        },
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            // let vec_cat_id: Vec<
            //     common::repositories_types::tufa_server::routes::api::cats::DogId,
            // > = value
            //     .into_iter()
            //     .filter_map(|value| match value.id {
            //         Some(id) => Some(
            //             common::repositories_types::tufa_server::routes::api::cats::DogId {
            //                 id,
            //             },
            //         ),
            //         None => None,
            //     })
            //     .collect();
            // println!("{vec_cat_id:#?}");
        }
        Err(e) => {
            println!("{e}");
        }
    }
    println!("--------------try_read_many-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_read_many(
        &api_location,
        //todo - builder pattern?
        common::repositories_types::tufa_server::routes::api::cats::ReadManyParameters{ 
            payload: common::repositories_types::tufa_server::routes::api::cats::ReadManyPayload { 
                select: common::repositories_types::tufa_server::routes::api::cats::DogColumnSelect::IdNameColor,
                id: Some(
                    primary_keys.clone()
                    // vec![
                    //     common::server::postgres::uuid_wrapper::UuidWrapper::try_from(
                    //         common::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(id)
                    //     ).unwrap()
                    // ]
                ),
                name: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,//or and support
                color: None
                // Some(vec![common::server::postgres::regex_filter::RegexFilter {
                //     regex: std::string::String::from("test"),
                //     conjuctive_operator: common::server::postgres::conjuctive_operator::ConjunctiveOperator::Or,
                // }])
                ,
                order_by: common::server::postgres::order_by::OrderBy {
                    column: common::repositories_types::tufa_server::routes::api::cats::DogColumn::Name,
                    order: Some(common::server::postgres::order::Order::Desc),
                },
                limit: common::server::postgres::postgres_bigint::PostgresBigint(limit),
                offset: common::server::postgres::postgres_bigint::PostgresBigint(offset),
            } 
        },
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            // let vec_cat_id: Vec<
            //     common::repositories_types::tufa_server::routes::api::cats::DogId,
            // > = value
            //     .into_iter()
            //     .filter_map(|value| match value.id {
            //         Some(id) => Some(
            //             common::repositories_types::tufa_server::routes::api::cats::DogId {
            //                 id,
            //             },
            //         ),
            //         None => None,
            //     })
            //     .collect();
            // println!("{vec_cat_id:#?}");
        }
        Err(e) => {
            println!("{e}");
        }
    }    
    //
    println!("--------------try_create_one-----------------");//todo add try_create_many
    let primary_key = match common::repositories_types::tufa_server::routes::api::cats::try_create_one(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::CreateOneParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::CreateOnePayload {
                name: std::string::String::from("try_create_one_name"),
                color: std::string::String::from("try_create_one_color"),
            }
        },
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            value
        },
        Err(e) => {
            panic!("{e}");
        }
    };
    println!("--------------try_read_one-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_read_one(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::ReadOneParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::ReadOnePayload {
                id: primary_key.clone(),
                select: common::repositories_types::tufa_server::routes::api::cats::DogColumnSelect::IdNameColor
            }
        },
    )
    .await
    {
        Ok(value) => println!("{value:#?}"),
        Err(e) => {
            panic!("{e}");
        }
    }
    println!("--------------try_update_one------------------");//todo try_update_many
    let primary_key = match common::repositories_types::tufa_server::routes::api::cats::try_update_one(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::UpdateOneParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::UpdateOnePayload {
                id: primary_key.clone(),
                name: Some(std::string::String::from("name")), 
                color: Some(std::string::String::from("color")), 
            }
        }
    )
    .await
    {
        Ok(value) => {
            println!("{value:#?}");
            value
        },
        Err(e) => panic!("{e}"),
    };
    println!("--------------try_read_one-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_read_one(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::ReadOneParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::ReadOnePayload {
                id: primary_key.clone(),
                select: common::repositories_types::tufa_server::routes::api::cats::DogColumnSelect::IdNameColor
            }
        },
    )
    .await
    {
        Ok(value) => println!("{value:#?}"),
        Err(e) => {
            panic!("{e}");
        }
    }
    println!("--------------try_delete_one------------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_delete_one(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::DeleteOneParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::DeleteOnePayload {
                id: primary_key.clone()
            }
        },
    )
    .await
    {
        Ok(value) => println!("{value:#?}"),
        Err(e) => panic!("{e}"),
    }
    println!("--------------try_read_one-----------------");
    match common::repositories_types::tufa_server::routes::api::cats::try_read_one(
        &api_location,
        common::repositories_types::tufa_server::routes::api::cats::ReadOneParameters { 
            payload: common::repositories_types::tufa_server::routes::api::cats::ReadOnePayload {
                id: primary_key,
                select: common::repositories_types::tufa_server::routes::api::cats::DogColumnSelect::IdNameColor 
            }
        },
    )
    .await
    {
        Ok(value) => println!("{value:#?}"),
        Err(e) => {
            println!("{e}");
        }
    }    
    // let bot = teloxide::Bot::from_env();
    // teloxide::commands_repl(bot, answer, {
    //     use teloxide::utils::command::BotCommands;
    //     Command::ty()
    // })
    // .await;
}

// #[derive(teloxide::utils::command::BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     #[command(description = "display this text.")]
//     Help,
//     #[command(description = "handle a username.")]
//     Username(String),
//     #[command(description = "handle a username and an age.", parse_with = "split")]
//     UsernameAndAge { username: std::string::String, age: u8 },
//     #[command(description = "show bot source code info ")]
//     GitInfo,
// }

// async fn answer(
//     bot: teloxide::Bot,
//     msg: teloxide::types::Message,
//     cmd: Command,
// ) -> teloxide::requests::ResponseResult<()> {
//     log::info!("answer");
//     match cmd {
//         Command::Help => {
//             use teloxide::prelude::Requester;
//             bot.send_message(
//                 msg.chat.id,
//                 {
//                     use teloxide::utils::command::BotCommands;
//                     Command::descriptions()
//                 }
//                 .to_string(),
//             )
//             .await?
//         }
//         Command::Username(username) => {
//             use teloxide::prelude::Requester;
//             bot.send_message(msg.chat.id, format!("Your username is @{username}."))
//                 .await?
//         }
//         Command::UsernameAndAge { username, age } => {
//             use teloxide::prelude::Requester;
//             bot.send_message(
//                 msg.chat.id,
//                 format!("Your username is @{username} and age is {age}."),
//             )
//             .await?
//         }
//         Command::GitInfo => {
//             use teloxide::prelude::Requester;
//             bot.send_message(msg.chat.id, {
//                 use common::common::git::get_git_commit_link::GetGitCommitLink;
//                 crate::global_variables::compile_time::git_info::GIT_INFO.get_git_commit_link()
//             })
//             .await?
//         }
//     };

//     Ok(())
// }
