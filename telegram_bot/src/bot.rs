#[tokio::main]
pub async fn start_bot() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    // let id  = "b129a42c-5f99-4a3e-ab25-154f3549f638";
    //
        let api_location = std::string::String::from("http://127.0.0.1:8080");
        let limit = 1000;
        let offset = 0;
        println!("-------trycreate_many start-------");
        let primary_keys = match try_create_many(
            &api_location,
            CreateManyParameters {
                payload: CreateManyPayload(vec![CreateManyPayloadElement {
                    sqlx_types_time_time_as_postgresql_time:
                        postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime::default(),
                }]),
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------trycreate_many end-------");
        println!("-------tryread_many start-------");
        match
        try_read_many(& api_location, ReadManyParameters
        {
            payload : ReadManyPayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : Some(primary_keys.clone()),
                sqlx_types_time_time_as_postgresql_time : None, select :
                DogColumnSelect ::
                StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeySqlxTypesTimeTimeAsPostgresqlTime,
                order_by : crate :: server :: postgres :: order_by :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(crate :: server :: postgres :: order :: Order :: Desc),
                }, limit : crate :: server :: postgres :: postgres_bigint ::
                PostgresBigint(limit), offset : crate :: server :: postgres ::
                postgres_bigint :: PostgresBigint(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
            ("{}", e)
        } ;
        println!("-------tryread_many end-------");
        println!("-------tryupdate_many start-------");
        match try_update_many(
            &api_location,
            UpdateManyParameters {
                payload: UpdateManyPayload(
                    primary_keys
                        .clone()
                        .into_iter()
                        .map(|element| UpdateManyPayloadElement {
                            std_primitive_i64_as_postgresql_big_serial_not_null_primary_key:
                                element,
                            sqlx_types_time_time_as_postgresql_time:
                                postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime::default(),
                        })
                        .collect(),
                ),
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------tryupdate_many end-------");
        println!("-------tryread_many start-------");
        match
        try_read_many(& api_location, ReadManyParameters
        {
            payload : ReadManyPayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : Some(primary_keys.clone()),
                sqlx_types_time_time_as_postgresql_time : None, select :
                DogColumnSelect ::
                StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeySqlxTypesTimeTimeAsPostgresqlTime,
                order_by : crate :: server :: postgres :: order_by :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(crate :: server :: postgres :: order :: Order :: Desc),
                }, limit : crate :: server :: postgres :: postgres_bigint ::
                PostgresBigint(limit), offset : crate :: server :: postgres ::
                postgres_bigint :: PostgresBigint(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
            ("{}", e)
        } ;
        println!("-------tryread_many end-------");
        println!("-------trydelete_many start-------");
        match try_delete_many(
            &api_location,
            DeleteManyParameters {
                payload: DeleteManyPayload {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: Some(
                        primary_keys.clone(),
                    ),
                    sqlx_types_time_time_as_postgresql_time: None,
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------trydelete_many end-------");
        println!("-------tryread_many start-------");
        match
        try_read_many(& api_location, ReadManyParameters
        {
            payload : ReadManyPayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : Some(primary_keys.clone()),
                sqlx_types_time_time_as_postgresql_time : None, select :
                DogColumnSelect ::
                StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeySqlxTypesTimeTimeAsPostgresqlTime,
                order_by : crate :: server :: postgres :: order_by :: OrderBy
                {
                    column : DogColumn :: Name, order :
                    Some(crate :: server :: postgres :: order :: Order :: Desc),
                }, limit : crate :: server :: postgres :: postgres_bigint ::
                PostgresBigint(limit), offset : crate :: server :: postgres ::
                postgres_bigint :: PostgresBigint(offset),
            }
        },).await
        {
            Ok(value) => { println! ("{value:#?}") ; value }, Err(e) => panic!
            ("{}", e)
        } ;
        println!("-------tryread_many end-------");
        println!("-------trycreate_one start-------");
        let primary_key = match try_create_one(
            &api_location,
            CreateOneParameters {
                payload: CreateOnePayload {
                    sqlx_types_time_time_as_postgresql_time:
                        postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime::default(),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------trycreate_one end-------");
        println!("-------tryread_one start-------");
        match
        try_read_one(& api_location, ReadOneParameters
        {
            payload : ReadOnePayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : primary_key.clone(), select : DogColumnSelect ::
                StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeySqlxTypesTimeTimeAsPostgresqlTime
            }
        },).await
        { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) } ;
        println!("-------tryread_one end-------");
        println!("-------tryupdate_one start-------");
        let primary_key = match try_update_one(
            &api_location,
            UpdateOneParameters {
                payload: UpdateOnePayload {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: primary_key
                        .clone(),
                    sqlx_types_time_time_as_postgresql_time: Some(
                        postgresql_crud::SqlxTypesTimeTimeAsPostgresqlTime::default(),
                    ),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------tryupdate_one end-------");
        println!("-------tryread_one start-------");
        match
        try_read_one(& api_location, ReadOneParameters
        {
            payload : ReadOnePayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : primary_key.clone(), select : DogColumnSelect ::
                StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeySqlxTypesTimeTimeAsPostgresqlTime
            }
        },).await
        { Ok(value) => println! ("{value:#?}"), Err(e) => panic! ("{}", e) } ;
        println!("-------tryread_one end-------");
        println!("-------trydelete_one start-------");
        match try_delete_one(
            &api_location,
            DeleteOneParameters {
                payload: DeleteOnePayload {
                    std_primitive_i64_as_postgresql_big_serial_not_null_primary_key: primary_key
                        .clone(),
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------trydelete_one end-------");
        println!("-------tryread_one start-------");
        match
        try_read_one(& api_location, ReadOneParameters
        {
            payload : ReadOnePayload
            {
                std_primitive_i64_as_postgresql_big_serial_not_null_primary_key
                : primary_key.clone(), select : DogColumnSelect ::
                StdPrimitiveI64AsPostgresqlBigSerialNotNullPrimaryKeySqlxTypesTimeTimeAsPostgresqlTime
            }
        },).await
        { Ok(value) => panic! ("{value:#?}"), Err(e) => println! ("{}", e) } ;
        println!("-------tryread_one end-------");
    //
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