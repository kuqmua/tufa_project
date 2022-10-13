use crate::fetch::info_structures::common_rss_structures::GithubInfoFromHtml;
use crate::lazy_static::git_info::GIT_INFO;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use html_parser::Dom;
use html_parser::Node;

//todo: think about renaming
pub type GithubPostInfoVec = Vec<(
    Option<String>, //avatar_link_handle
    Option<String>, //relative_commit_link_handle
    Option<String>, //commit_text_handle
    Option<String>, //from_text_handle
    Option<String>, //commits_number_handle
    Option<String>, //isssue_label_handle//somewhere wrong like "brotzeit/nyxt" or "taiki-e/criterion.rs"
    Option<String>, //data_hovercard_type,
    Option<String>, //data_hovercard_url,
    Option<String>, //data_id,
    Option<String>, //href,
    Option<String>, //data_url,
)>;

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn parse_github_html(option_content: Option<String>) -> GithubInfoFromHtml {
    let mut avatar_link: Option<String> = None;
    let mut author: Option<String> = None;
    let mut action: Option<String> = None;
    let mut repository: Option<String> = None;
    let mut from_what_repository_forked: Option<String> = None;
    let mut from: Option<String> = None;
    let mut datejs: Option<String> = None;
    let mut date: Option<String> = None;
    let mut actionto: Option<String> = None;
    let mut branch: Option<String> = None;
    let mut release_tag: Option<String> = None;
    let mut of: Option<String> = None;
    let mut bot_tag: Option<String> = None;
    let mut who_follow: Option<String> = None;
    let mut vec_of_something: GithubPostInfoVec = Vec::new();
    match option_content {
        Some(content) => {
            let result_of_dom_parse_content = Dom::parse(&content);
            match result_of_dom_parse_content {
                Ok(dom) => {
                    match dom.children.len() {
                        1 => match dom.children[0] {
                            Node::Element(ref dom_first_child) => {
                                match dom_first_child.children.len() {
                                1 => match dom_first_child.children[0] {
                                    Node::Element(ref dom_first_child_first_child) => {
                                        match dom_first_child_first_child.children.len() {
                                            1 => match dom_first_child_first_child.children[0] {
                                                Node::Element(
                                                    ref dom_first_child_first_child_first_child,
                                                ) => {
                                                    match dom_first_child_first_child_first_child.children.len() {
                                                        2 => {
                                                            avatar_link =
                                                                parse_github_html_first_part(
                                                                    &dom_first_child_first_child_first_child.children[0],
                                                                );
                                                            let (
                                                                author_handle,
                                                                action_handle,
                                                                repository_handle,
                                                                from_what_repository_forked_handle,
                                                                from_handle,
                                                                datejs_handle,
                                                                date_handle,
                                                                actionto_handle,
                                                                branch_handle,
                                                                release_tag_handle,
                                                                of_handle,
                                                                bot_tag_handle,
                                                                who_follow_handle,
                                                                vec_of_something_handle
                                                            ) = parse_github_html_second_part(
                                                                &dom_first_child_first_child_first_child.children[1],
                                                            );
                                                            author = author_handle;
                                                            action = action_handle;
                                                            repository = repository_handle;
                                                            from_what_repository_forked = from_what_repository_forked_handle;
                                                            from =   from_handle;
                                                            datejs = datejs_handle;
                                                            date = date_handle;
                                                            actionto = actionto_handle;
                                                            branch = branch_handle;
                                                            release_tag = release_tag_handle;
                                                            of = of_handle;
                                                            who_follow = who_follow_handle;
                                                            bot_tag  = bot_tag_handle;
                                                            vec_of_something = vec_of_something_handle;
                                                        }
                                                        _ => {
                                                            let warning_message = format!(
                                                                "different children.len(): {}",
                                                                dom_first_child_first_child_first_child.children.len()
                                                            );
                                                            print_colorful_message(
                                                                None,
                                                                PrintType::WarningHigh,
                                                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                warning_message,
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    dom_first_child_first_child.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            );
                                            }
                                        }
                                    }
                                    _ => print_colorful_message(
                                                    None,
                                        PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                                },
                                _ => {
                                    let warning_message =
                                        format!("different children.len(): {}", dom_first_child.children.len());
                                    print_colorful_message(
                                                    None,
                                        PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                                }
                            }
                            }
                            _ => print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                "different node".to_string(),
                            ),
                        },
                        _ => {
                            let warning_message =
                                format!("different children.len(): {}", dom.children.len());
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                warning_message,
                            )
                        }
                    }
                }
                Err(e) => {
                    let error_message = format!("Dom::parse error {e}");
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        error_message,
                    )
                }
            }
        }
        None => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "option content is None".to_string(),
        ),
    }
    // println!("avatar_link {avatar_link:#?}");
    // println!("author {author:#?}");
    // println!("action {action:#?}");
    // println!("repository {repository:#?}", );
    // println!("from_what_repository_forked {from_what_repository_forked:#?}");
    // println!("from {from:#?}");
    // println!("datejs {datejs:#?}");
    // println!("date {date:#?}");
    // println!("actionto {actionto:#?}");
    // println!("branch {branch:#?}");
    // println!("release_tag {release_tag:#?}");
    // println!("of {of:#?}");
    // println!("bot_tag {bot_tag:#?}");
    // println!("who_follow {who_follow:#?}");
    // println!("vec_of_something {vec_of_something:#?}");
    // for i in &vec_of_something {
    //     println!("label{:#?}", i.5)
    // }
    GithubInfoFromHtml::initialize_with_params(
        avatar_link,
        author,
        action,
        repository,
        from_what_repository_forked,
        from,
        datejs,
        date,
        actionto,
        branch,
        release_tag,
        of,
        bot_tag,
        who_follow,
        vec_of_something,
    )
}

fn parse_github_html_first_part(node: &Node) -> Option<String> {
    let mut avatar_link: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Element(ref node_element_first) => match node_element_first.children.len() {
                    1 => match node_element_first.children[0] {
                        Node::Element(ref node_element_first_first) => {
                            let attribute = "src"; //todo move variable like this into some whole variable structure
                            match node_element_first_first.attributes.get(attribute) {
                                Some(avatar_link_handle_option) => {
                                    match avatar_link_handle_option {
                                        Some(avatar_link_handle) => {
                                            if !avatar_link_handle.is_empty() {
                                                avatar_link = avatar_link_handle_option.clone();
                                            }
                                        }
                                        None => {
                                            //todo?
                                        }
                                    }
                                }
                                None => {
                                    let warning_message = format!("no {attribute} attribute");
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    },
                    _ => {
                        let warning_message = format!(
                            "different children.len(): {}",
                            node_element_first.children.len()
                        );
                        print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            warning_message,
                        )
                    }
                },
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    avatar_link
}

fn parse_github_html_second_part(
    node: &Node,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Vec<(
        Option<String>, //avatar_link_handle
        Option<String>, //relative_commit_link_handle
        Option<String>, //commit_text_handle
        Option<String>, //from_text_handle
        Option<String>, //commits_number_handle
        Option<String>, //isssue_label_handle
        Option<String>, //data_hovercard_type,
        Option<String>, //data_hovercard_url,
        Option<String>, //data_id,
        Option<String>, //href,
        Option<String>, //data_url,
    )>,
) {
    let mut author: Option<String> = None;
    let mut action: Option<String> = None;
    let mut repository: Option<String> = None;
    let mut from_what_repository_forked: Option<String> = None;
    let mut from: Option<String> = None;
    let mut datejs: Option<String> = None;
    let mut date: Option<String> = None;
    let mut actionto: Option<String> = None;
    let mut branch: Option<String> = None;
    let mut release_tag: Option<String> = None;
    let mut of: Option<String> = None;
    let mut bot_tag: Option<String> = None;
    let mut who_follow: Option<String> = None; //todo duplication problem
    let mut vec_of_something: Vec<(
        Option<String>, //avatar_link_handle
        Option<String>, //relative_commit_link_handle
        Option<String>, //commit_text_handle
        Option<String>, //from_text_handle
        Option<String>, //commits_number_handle
        Option<String>, //isssue_label_handle
        //
        Option<String>, //data_hovercard_type,
        Option<String>, //data_hovercard_url,
        Option<String>, //data_id,
        Option<String>, //href,
        Option<String>, //data_url,
    )> = Vec::new();
    match node {
        Node::Element(ref node_element) => {
            match node_element.children.len() {
                1 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                5 => {
                                    match node_element_first.children[0] {
                                        Node::Element(ref node_element_first_first) => {
                                            match node_element_first_first.children.len() {
                                                1 => match node_element_first_first.children[0] {
                                                    Node::Text(ref author_handle) => {
                                                        if !author_handle.is_empty() {
                                                            author =
                                                                Some(author_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_first.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[1] {
                                        Node::Text(ref action_handle) => {
                                            if !action_handle.is_empty() {
                                                action = Some(action_handle.to_string())
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[2] {
                                        Node::Element(ref node_element_first_third) => {
                                            match node_element_first_third.children.len() {
                                                1 => match node_element_first_third.children[0] {
                                                    Node::Text(ref repository_handle) => {
                                                        if !repository_handle.is_empty() {
                                                            repository =
                                                                Some(repository_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_third.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[3] {
                                        Node::Element(ref node_element_first_fourth) => {
                                            match node_element_first_fourth.children.len() {
                                                1 => {
                                                    match node_element_first_fourth.children[0] {
                                                        Node::Element(
                                                            ref node_element_first_fourth_first,
                                                        ) => {
                                                            let attribute = "datetime";
                                                            match node_element_first_fourth_first
                                                                .attributes
                                                                .get(attribute)
                                                            {
                                                                Some(datejs_handle_option) => {
                                                                    match datejs_handle_option {
                                                                        Some(datejs_handle) => {
                                                                            if !datejs_handle
                                                                                .is_empty()
                                                                            {
                                                                                datejs = datejs_handle_option.clone();
                                                                            }
                                                                        }
                                                                        None => {
                                                                            //todo
                                                                        }
                                                                    }
                                                                }
                                                                None => {
                                                                    let warning_message = format!(
                                                                        "no {attribute} attribute",
                                                                    );
                                                                    print_colorful_message(
                                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    warning_message,
                                                                )
                                                                }
                                                            }
                                                            match node_element_first_fourth_first
                                                        .children
                                                        .len()
                                                    {
                                                        1 => {
                                                            match node_element_first_fourth_first
                                                                .children[0]
                                                            {
                                                                Node::Text(
                                                                    ref date_handle,
                                                                ) => {
                                                                    if !date_handle.is_empty() {
                                                                        date =
                                                                        Some(date_handle.to_string());
                                                                    }
                                                                }
                                                                _ => print_colorful_message(
                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                "different node".to_string(),
                                                            ),
                                                            }
                                                        }
                                                        _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_fourth_first.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                            }
                                                    }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_fourth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[4] {
                                        Node::Element(ref node_element_first_firth) => {
                                            match node_element_first_firth.children.len() {
                                                3 => {
                                                    match node_element_first_firth.children[0] {
                                                        Node::Element(
                                                            ref node_element_first_firth_first,
                                                        ) => {
                                                            match node_element_first_firth_first
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_first
                                                                    .children[0]
                                                                {
                                                                    Node::Text(
                                                                        ref actionto_handle,
                                                                    ) => {
                                                                        if !actionto_handle.is_empty() {
                                                                            actionto = Some(actionto_handle.to_string());
                                                                        }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
                                                                        PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    "different node".to_string(),
                                                                ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                    "different children.len(): {}",
                                   node_element_first_firth_first.children.len()
                                );
                                                                print_colorful_message(
                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                warning_message,
                                                            )
                                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                    match node_element_first_firth.children[1] {
                                                        Node::Element(
                                                            ref node_element_first_firth_second,
                                                        ) => {
                                                            match node_element_first_firth_second
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_second
                                                                    .children[0]
                                                                {
                                                                    Node::Text(ref branch_handle) => {
                                                                        if !branch_handle.is_empty() {
                                                                            branch = Some(branch_handle.to_string());
                                                                        }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
                                                                        PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    "different node".to_string(),
                                                                ),
                                                                }
                                                            }
                                                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first_firth_second.children.len()
                                );
                                print_colorful_message(
                                                    None,
                                    PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                warning_message,
                            )
                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                    match node_element_first_firth.children[2] {
                                                        Node::Element(
                                                            ref node_element_first_firth_third,
                                                        ) => {
                                                            match node_element_first_firth_third
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_third
                                                                    .children[0]
                                                                {
                                                                    Node::Element(
                                                                        ref
                                                                        node_element_first_firth_third_first,
                                                                    ) => {
                                                                        for i in &node_element_first_firth_third_first.children {
                                                                                vec_of_something.push(parse_github_html_second_part_inner_one_element(&i));
                                                                            }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
                                                                        PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    "different node".to_string(),
                                                                ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                    "different children.len(): {}",
                                    node_element_first_firth_third.children.len()
                                );
                                                                print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_firth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                }
                                6 => {
                                    match node_element_first.children[0] {
                                        Node::Element(ref node_element_first_first) => {
                                            match node_element_first_first.children.len() {
                                                1 => match node_element_first_first.children[0] {
                                                    Node::Text(ref author_handle) => {
                                                        if !author_handle.is_empty() {
                                                            author =
                                                                Some(author_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_first.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[1] {
                                        Node::Element(ref node_element_first_second) => {
                                            match node_element_first_second.children.len() {
                                                1 => match node_element_first_second.children[0] {
                                                    Node::Text(ref bot_tag_handle) => {
                                                        if !bot_tag_handle.is_empty() {
                                                            bot_tag =
                                                                Some(bot_tag_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_second.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[2] {
                                        Node::Text(ref action_handle) => {
                                            if !action_handle.is_empty() {
                                                action = Some(action_handle.to_string())
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[3] {
                                        Node::Element(ref node_element_first_fourth) => {
                                            match node_element_first_fourth.children.len() {
                                                1 => match node_element_first_fourth.children[0] {
                                                    Node::Text(ref repository_handle) => {
                                                        if !repository_handle.is_empty() {
                                                            repository =
                                                                Some(repository_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_fourth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[4] {
                                        Node::Element(ref node_element_first_firth) => {
                                            match node_element_first_firth.children.len() {
                                                1 => {
                                                    match node_element_first_firth.children[0] {
                                                        Node::Element(
                                                            ref node_element_first_firth_first,
                                                        ) => {
                                                            let attribute = "datetime";
                                                            match node_element_first_firth_first
                                                                .attributes
                                                                .get(attribute)
                                                            {
                                                                Some(datejs_handle_value) => {
                                                                    match datejs_handle_value {
                                                                        Some(datejs_handle) => {
                                                                            if !datejs_handle
                                                                                .is_empty()
                                                                            {
                                                                                datejs =
                                                                                datejs_handle_value
                                                                                    .clone();
                                                                            }
                                                                        }
                                                                        None => {}
                                                                    }
                                                                }
                                                                None => {
                                                                    let warning_message = format!(
                                                                        "no {attribute} attribute"
                                                                    );
                                                                    print_colorful_message(
                                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    warning_message,
                                                                )
                                                                }
                                                            }
                                                            match node_element_first_firth_first
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_first
                                                                    .children[0]
                                                                {
                                                                    Node::Text(ref date_handle) => {
                                                                        if !date_handle.is_empty() {
                                                                            date = Some(date_handle.to_string());
                                                                        }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        "different node".to_string(),
    ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message = format!(
                                                                    "different children.len(): {}",
                                                                    node_element_first_firth_first
                                                                        .children
                                                                        .len()
                                                                );
                                                                print_colorful_message(
                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                warning_message,
                                                            )
                                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_firth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[5] {
                                        Node::Element(ref node_element_first_sixth) => {
                                            match node_element_first_sixth.children.len() {
                                                3 => {
                                                    match node_element_first_sixth.children[0] {
                                                        Node::Element(
                                                            ref node_element_first_sixth_first,
                                                        ) => {
                                                            match node_element_first_sixth_first
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_sixth_first
                                                                    .children[0]
                                                                {
                                                                    Node::Text(
                                                                        ref actionto_handle,
                                                                    ) => {
                                                                        if !actionto_handle.is_empty() {
                                                                            actionto = Some(actionto_handle.to_string());
                                                                        }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        "different node".to_string(),
    ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                    "different children.len(): {}",
                                    node_element_first_sixth_first.children.len()
                                );
                                                                print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                    match node_element_first_sixth.children[1] {
                                                        Node::Element(
                                                            ref node_element_first_sixth_second,
                                                        ) => {
                                                            match node_element_first_sixth_second
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_sixth_second
                                                                    .children[0]
                                                                {
                                                                    Node::Text(ref branch_handle) => {
                                                                        if !branch_handle.is_empty() {
                                                                            branch = Some(branch_handle.to_string());
                                                                        }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        "different node".to_string(),
    ),
                                                                }
                                                            }
                                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_sixth_second.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                    match node_element_first_sixth.children[2] {
                                                        Node::Element(
                                                            ref node_element_first_sixth_third,
                                                        ) => {
                                                            match node_element_first_sixth_third
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_sixth_third
                                                                    .children[0]
                                                                {
                                                                    Node::Element(
                                                                        ref
                                                                        node_element_first_sixth_third_first,
                                                                    ) => {
                                                                        for i in &node_element_first_sixth_third_first.children {
                                                                                vec_of_something.push(parse_github_html_second_part_inner_one_element(&i));
                                                                            }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        "different node".to_string(),
    ),
                                                                }
                                                            }
                                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_sixth_third.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_sixth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                }
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    }
                }
                2 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref second_child_element3) => {
                                        match second_child_element3.children.len() {
                                            1 => {
                                                let (
                                                    author_name_another_handle,
                                                    action_another_handle,
                                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_handle,
                                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle,
                                                    datejs_another_handle,
                                                    date_another_handle,
                                                    from_handle,
                                                    isssue_label_handle,
                                                ) = two_elements_one_child(
                                                    &second_child_element3.children[0],
                                                );
                                                author = author_name_another_handle;
                                                action = action_another_handle;
                                                repository = the_accounts_repo_on_which_the_action_was_performed_relative_href_handle;
                                                from_what_repository_forked = the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle;
                                                from = from_handle;
                                                datejs = datejs_another_handle;
                                                date = date_another_handle;
                                                if isssue_label_handle.is_some() {
                                                    vec_of_something.push((
                                                        None,                //avatar_link_handle
                                                        None, //relative_commit_link_handle
                                                        None, //commit_text_handle
                                                        None, //from_text_handle
                                                        None, //commits_number_handle
                                                        isssue_label_handle, //isssue_label_handle
                                                        None, //data_hovercard_type,
                                                        None, //data_hovercard_url,
                                                        None, //data_id,
                                                        None, //href,
                                                        None, //data_url,
                                                    ))
                                                }
                                            }
                                            4 => {
                                                author = two_elements_four_children_first(
                                                    &second_child_element3.children[0],
                                                );
                                                action = two_elements_four_children_second(
                                                    &second_child_element3.children[1],
                                                );
                                                who_follow = two_elements_four_children_third(
                                                    &second_child_element3.children[2],
                                                );
                                                let (datejs_another_handle, date_another_handle) =
                                                    two_elements_four_children_fourth(
                                                        &second_child_element3.children[3],
                                                    );
                                                datejs = datejs_another_handle;
                                                date = date_another_handle;
                                            }
                                            //todo case 3
                                            //todo case 6
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    second_child_element3.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
                                                    vec![format!(
                                                        "{}{}{}",
                                                        file!(),
                                                        line!(),
                                                        column!()
                                                    )],
                                                    vec![GIT_INFO.data.get_git_source_file_link(
                                                        file!(),
                                                        line!(),
                                                    )],
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        "different node".to_string(),
                                    ),
                                },
                                6 => {
                                    match node_element_first.children[0] {
                                        Node::Element(ref node_element_first_first) => {
                                            match node_element_first_first.children.len() {
                                                1 => match node_element_first_first.children[0] {
                                                    Node::Text(ref author_handle) => {
                                                        if !author_handle.is_empty() {
                                                            author =
                                                                Some(author_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_first.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[1] {
                                        Node::Text(ref action_handle) => {
                                            if !action_handle.is_empty() {
                                                action = Some(action_handle.to_string());
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[2] {
                                        Node::Element(ref node_element_first_third) => {
                                            match node_element_first_third.children.len() {
                                                1 => match node_element_first_third.children[0] {
                                                    Node::Text(ref release_tag_handle) => {
                                                        if !release_tag_handle.is_empty() {
                                                            release_tag = Some(
                                                                release_tag_handle.to_string(),
                                                            );
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_third.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[3] {
                                        Node::Text(ref of_handle) => {
                                            if !of_handle.is_empty() {
                                                of = Some(of_handle.to_string());
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[4] {
                                        Node::Element(ref node_element_first_fourth) => {
                                            match node_element_first_fourth.children.len() {
                                                1 => match node_element_first_fourth.children[0] {
                                                    Node::Text(ref repository_handle) => {
                                                        if !repository_handle.is_empty() {
                                                            repository =
                                                                Some(repository_handle.to_string());
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_fourth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_first.children[5] {
                                        Node::Element(ref node_element_first_sixth) => {
                                            match node_element_first_sixth.children.len() {
                                                1 => {
                                                    match node_element_first_sixth.children[0] {
                                                        Node::Element(
                                                            ref node_element_first_sixth_first,
                                                        ) => {
                                                            let attribute = "datetime";
                                                            match node_element_first_sixth_first
                                                                .attributes
                                                                .get(attribute)
                                                            {
                                                                Some(datejs_handle_option) => {
                                                                    match datejs_handle_option {
                                                                        Some(datejs_handle) => {
                                                                            if !datejs_handle
                                                                                .is_empty()
                                                                            {
                                                                                datejs = datejs_handle_option.clone();
                                                                            }
                                                                        }
                                                                        None => {}
                                                                    }
                                                                }
                                                                None => {
                                                                    let warning_message = format!(
                                                                        "no {attribute} attribute"
                                                                    );
                                                                    print_colorful_message(
                                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    warning_message,
                                                                )
                                                                }
                                                            }
                                                            match node_element_first_sixth_first
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match node_element_first_sixth_first
                                                                .children[0] {
                                                                    Node::Text(ref date_handle) => {
                                                                        if !date_handle.is_empty() {
                                                                            date = Some(date_handle.to_string());
                                                                        }
                                                                    }
                                                                    _ => print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        "different node".to_string(),
    ),
                                                                }
                                                                }
                                                                _ => {
                                                                    let warning_message = format!(
                                                                        "different children.len(): {}",
                                                                        node_element_first_sixth_first.children.len()
                                                                    );
                                                                    print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_first_sixth.children.len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                }
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[0] {
                        Node::Element(ref node_element_second) => {
                            match node_element_second.children.len() {
                                1 => match node_element_second.children[0] {
                                    Node::Element(ref node_element_second_element) => {
                                        match node_element_second_element.children.len() {
                                            1 => {
                                                let (
                                                        author_handle,
                                                        action_handle,
                                                        repository_handle,
                                                        from_handle,
                                                        from_what_repository_forked_handle,
                                                        datejs_handle,
                                                        date_handle,
                                                    ) = parse_github_html_second_part_two_children_first(&node_element_second_element.children[0]);
                                                author = author_handle;
                                                action = action_handle;
                                                repository = repository_handle;
                                                from = from_handle;
                                                from_what_repository_forked =
                                                    from_what_repository_forked_handle;
                                                datejs = datejs_handle;
                                                date = date_handle;
                                            }
                                            4 => {
                                                let (
                                                    author_handle,
                                                    action_handle,
                                                    who_follow_handle,
                                                    datejs_handle,
                                                    date_handle,
                                                ) = four_cases(
                                                    &node_element_second_element.children,
                                                );
                                                author = author_handle;
                                                action = action_handle;
                                                who_follow = who_follow_handle;
                                                datejs = datejs_handle;
                                                date = date_handle;
                                            }
                                            //todo case 3
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_second_element.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
                                                    vec![format!(
                                                        "{}{}{}",
                                                        file!(),
                                                        line!(),
                                                        column!()
                                                    )],
                                                    vec![GIT_INFO.data.get_git_source_file_link(
                                                        file!(),
                                                        line!(),
                                                    )],
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        "different node".to_string(),
                                    ),
                                },
                                6 => {
                                    match node_element_second.children[0] {
                                        Node::Element(ref node_element_second_first_element) => {
                                            match node_element_second_first_element.children.len() {
                                                1 => match node_element_second_first_element
                                                    .children[0]
                                                {
                                                    Node::Text(ref author_handle) => {
                                                        if !author_handle.is_empty() {
                                                            author = Some(author_handle.to_string())
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_first_element
                                                            .children
                                                            .len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[1] {
                                        Node::Text(ref action_handle) => {
                                            if !action_handle.is_empty() {
                                                action = Some(action_handle.to_string())
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[2] {
                                        Node::Element(ref node_element_second_second_element) => {
                                            match node_element_second_second_element.children.len()
                                            {
                                                1 => match node_element_second_second_element
                                                    .children[0]
                                                {
                                                    Node::Text(ref release_tag_handle) => {
                                                        if !release_tag_handle.is_empty() {
                                                            release_tag =
                                                                Some(release_tag_handle.to_string())
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_second_element
                                                            .children
                                                            .len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[3] {
                                        Node::Text(ref of_handle) => {
                                            if !of_handle.is_empty() {
                                                of = Some(of_handle.to_string())
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[4] {
                                        Node::Element(ref node_element_second_firth_element) => {
                                            match node_element_second_firth_element.children.len() {
                                                1 => match node_element_second_firth_element
                                                    .children[0]
                                                {
                                                    Node::Text(ref repository_handle) => {
                                                        if !repository_handle.is_empty() {
                                                            repository =
                                                                Some(repository_handle.to_string())
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_firth_element
                                                            .children
                                                            .len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[5] {
                                        Node::Element(ref node_element_second_six_element) => {
                                            match node_element_second_six_element.children.len() {
                                                1 => {
                                                    match node_element_second_six_element.children
                                                        [0]
                                                    {
                                                        Node::Element(
                                                            ref node_element_second_six_element,
                                                        ) => {
                                                            let attribute = "datetime";
                                                            match node_element_second_six_element
                                                                .attributes
                                                                .get(attribute)
                                                            {
                                                                Some(datejs_handle_option) => {
                                                                    match datejs_handle_option {
                                                                        Some(datejs_handle) => {
                                                                            if !datejs_handle
                                                                                .is_empty()
                                                                            {
                                                                                datejs = datejs_handle_option.clone();
                                                                            }
                                                                        }
                                                                        None => {}
                                                                    }
                                                                }
                                                                None => {
                                                                    let warning_message = format!(
                                                                        "no {attribute} attribute"
                                                                    );
                                                                    print_colorful_message(
                                                                    None,
                                                                    PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    warning_message,
                                                                )
                                                                }
                                                            }
                                                            match node_element_second_six_element.children.len() {
                                                            1 => match node_element_second_six_element.children
                                                                [0]
                                                            {
                                                                Node::Text(
                                                                    ref date_handle,
                                                                ) => {
                                                                    if !date_handle.is_empty() {
                                                                        date = Some(
                                                                            date_handle
                                                                            .to_string(),
                                                                    );
                                                                    }
                                                                }
                                                                _ => print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        "different node".to_string(),
    ),
                                                            },
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                "different children.len(): {}",
                                node_element_second_six_element.children.len()
                            );
                                                                print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                                            }
                                                        }
                                                        }
                                                        _ => print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            "different node".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_six_element
                                                            .children
                                                            .len()
                                                    );
                                                    print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            "different node".to_string(),
                                        ),
                                    }
                                }
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_second.children.len()
                                    );
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    }
                }
                5 => {
                    println!("5f");
                }
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        warning_message,
                    )
                }
            }
        }
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("author {author}");
    // println!("action {action}");
    // println!("repository {repository}");
    // println!("datejs {datejs:#?}");
    // println!("date {date}");
    // println!("repository {repository}");
    // println!("actionto {actionto}");
    // println!("branch {branch}");
    // println!("vec_of_something.len() {}", vec_of_something.len());
    (
        author,
        action,
        repository,
        from_what_repository_forked,
        from,
        datejs,
        date,
        actionto,
        branch,
        release_tag,
        of,
        bot_tag,
        who_follow,
        vec_of_something,
    )
}

fn parse_github_html_second_part_inner_one_element(
    node: &Node,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    //out params need to be pushed into vec array or something
    let mut avatar_link: Option<String> = None;
    let mut relative_commit_link: Option<String> = None;
    let mut commit_text: Option<String> = None;
    let mut from_text: Option<String> = None;
    let mut commits_number: Option<String> = None;

    let mut data_hovercard_type: Option<String> = None; //todo there are few values for this
    let mut data_hovercard_url: Option<String> = None; //todo there are few values for this
    let mut data_id: Option<String> = None; //todo there are few values for this
    let mut href: Option<String> = None; //todo there are few values for this
    let mut data_url: Option<String> = None; //todo there are few values for this

    match node {
        Node::Element(ref node_element) => {
            match node_element.children.len() {
                1 => match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref commits_number_handle) => {
                                    if !commits_number_handle.is_empty() {
                                        commits_number = Some(commits_number_handle.to_string());
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                },
                3 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_first) => {
                                        match node_element_first_first.children.len() {
                                            0 => {
                                                let attribute = "data-canonical-src";
                                                match node_element_first_first
                                                    .attributes
                                                    .get(attribute)
                                                {
                                                    Some(_) => {
                                                        //useless default github avatar
                                                    }
                                                    None => {
                                                        let warning_message =
                                                            format!("no {attribute} attribute");
                                                        print_colorful_message(
                                                            None,
                                                            PrintType::WarningHigh,
                                                            vec![format!(
                                                                "{}{}{}",
                                                                file!(),
                                                                line!(),
                                                                column!()
                                                            )],
                                                            vec![GIT_INFO
                                                                .data
                                                                .get_git_source_file_link(
                                                                    file!(),
                                                                    line!(),
                                                                )],
                                                            warning_message,
                                                        )
                                                    }
                                                }
                                            }
                                            1 => match node_element_first_first.children[0] {
                                                Node::Element(
                                                    ref node_element_first_first_element,
                                                ) => {
                                                    let attribute = "src";
                                                    match node_element_first_first_element
                                                        .attributes
                                                        .get(attribute)
                                                    {
                                                        Some(avatar_link_handle_option) => {
                                                            match avatar_link_handle_option {
                                                                Some(avatar_link_handle) => {
                                                                    if !avatar_link_handle
                                                                        .is_empty()
                                                                    {
                                                                        avatar_link =
                                                                        avatar_link_handle_option.clone();
                                                                    }
                                                                }
                                                                None => {
                                                                    //todo
                                                                }
                                                            }
                                                        }
                                                        None => {
                                                            let warning_message =
                                                                format!("no {attribute} attribute");
                                                            print_colorful_message(
                                                                None,
                                                                PrintType::WarningHigh,
                                                                vec![format!(
                                                                    "{}{}{}",
                                                                    file!(),
                                                                    line!(),
                                                                    column!()
                                                                )],
                                                                vec![GIT_INFO
                                                                    .data
                                                                    .get_git_source_file_link(
                                                                        file!(),
                                                                        line!(),
                                                                    )],
                                                                warning_message,
                                                            )
                                                        }
                                                    }
                                                }
                                                _ => print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
                                                    vec![format!(
                                                        "{}{}{}",
                                                        file!(),
                                                        line!(),
                                                        column!()
                                                    )],
                                                    vec![GIT_INFO.data.get_git_source_file_link(
                                                        file!(),
                                                        line!(),
                                                    )],
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_first.children.len()
                                                );
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
                                                    vec![format!(
                                                        "{}{}{}",
                                                        file!(),
                                                        line!(),
                                                        column!()
                                                    )],
                                                    vec![GIT_INFO.data.get_git_source_file_link(
                                                        file!(),
                                                        line!(),
                                                    )],
                                                    warning_message,
                                                );
                                                println!("node_element_first_first{node_element_first_first:#?}");
                                            }
                                        }
                                    }
                                    _ => print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[1] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_element) => {
                                        let attribute = "href";
                                        match node_element_first_element.attributes.get(attribute) {
                                            Some(relative_commit_link_handle_option) => {
                                                match relative_commit_link_handle_option {
                                                    Some(relative_commit_link_handle) => {
                                                        if !relative_commit_link_handle.is_empty() {
                                                            relative_commit_link =
                                                                relative_commit_link_handle_option
                                                                    .clone();
                                                        }
                                                    }
                                                    None => {}
                                                }
                                            }
                                            None => {
                                                let warning_message =
                                                    format!("no {attribute} attribute");
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
                                                    vec![format!(
                                                        "{}{}{}",
                                                        file!(),
                                                        line!(),
                                                        column!()
                                                    )],
                                                    vec![GIT_INFO.data.get_git_source_file_link(
                                                        file!(),
                                                        line!(),
                                                    )],
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[2] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_first) => {
                                        match node_element_first_first.children.len() {
                                            1 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                            }
                                            2 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                                let (
                                                    data_hovercard_type_handle,
                                                    data_hovercard_url_handle,
                                                    data_id_handle,
                                                    href_handle,
                                                    data_url_handle,
                                                ) = second_element(
                                                    &node_element_first_first.children[1],
                                                );
                                                data_hovercard_type = data_hovercard_type_handle;
                                                data_hovercard_url = data_hovercard_url_handle;
                                                data_id = data_id_handle;
                                                href = href_handle;
                                                data_url = data_url_handle;
                                            }
                                            3 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                                let (
                                                    data_hovercard_type_handle,
                                                    data_hovercard_url_handle,
                                                    data_id_handle,
                                                    href_handle,
                                                    data_url_handle,
                                                ) = second_element(
                                                    &node_element_first_first.children[1],
                                                );
                                                data_hovercard_type = data_hovercard_type_handle;
                                                data_hovercard_url = data_hovercard_url_handle;
                                                data_id = data_id_handle;
                                                href = href_handle;
                                                data_url = data_url_handle;

                                                match node_element_first_first.children[2] {
                                                    Node::Text(ref text) => {
                                                        if !text.is_empty() {
                                                            from_text = Some(text.to_string());
                                                        }
                                                    }
                                                    Node::Element(ref emoji_element) => {
                                                        let attribute = "fallback-src";
                                                        match emoji_element
                                                            .attributes
                                                            .get(attribute)
                                                        {
                                                            Some(link_option) => {
                                                                match link_option {
                                                                    Some(link) => {
                                                                         if link != "https://github.githubassets.com/images/icons/emoji/unicode/1f680.png" {
                                                                    let warning_message = format!(
                                                                                "different emoji link: {:#?}",
                                                                                emoji_element.attributes.get(attribute)
                                                                            );
                                                                            print_colorful_message(
                                                    None,
                                                                                PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                            warning_message,
                                                                        )
                                                                }
                                                                    }
                                                                    None => {
                                                                        let warning_message = "unexpected, emoji link is None"
                                                                    .to_string();
                                                                         print_colorful_message(
                                                    None,
                                                                                PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                            warning_message,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                            None => {
                                                                let warning_message = "unexpected, no emoji link"
                                                                    .to_string();
                                                                    print_colorful_message(
                                                    None,
                                                                        PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                                                    warning_message,
                                                                )
                                                            },
                                                        }
                                                    }
                                                    _ => print_colorful_message(
                                                        None,
                                                        PrintType::WarningHigh,
                                                        vec![format!(
                                                            "{}{}{}",
                                                            file!(),
                                                            line!(),
                                                            column!()
                                                        )],
                                                        vec![GIT_INFO
                                                            .data
                                                            .get_git_source_file_link(
                                                                file!(),
                                                                line!(),
                                                            )],
                                                        "different node".to_string(),
                                                    ),
                                                }
                                            }
                                            4 => {
                                                println!("TODO")
                                            }
                                            5 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                                let (
                                                    data_hovercard_type_handle,
                                                    data_hovercard_url_handle,
                                                    data_id_handle,
                                                    href_handle,
                                                    data_url_handle,
                                                ) = second_element(
                                                    &node_element_first_first.children[1],
                                                );
                                                println!("data_hovercard_type_handle1 {data_hovercard_type_handle:#?}");
                                                println!("data_hovercard_url_handle1{data_hovercard_url_handle:#?}");
                                                println!("data_id_handle1 {data_id_handle:#?}");
                                                println!("href_handle1 {href_handle:#?}",);
                                                println!("data_url_handle1 {data_url_handle:#?}");
                                                println!("----------------------");
                                                // todo: value copies
                                                // data_hovercard_type = data_hovercard_type_handle;
                                                // data_hovercard_url = data_hovercard_url_handle;
                                                // data_id = data_id_handle;
                                                // href = href_handle;
                                                // data_url = data_url_handle;
                                                from_text = handle_text_element(
                                                    &node_element_first_first.children[2],
                                                );
                                                let (
                                                    data_hovercard_type_handle,
                                                    data_hovercard_url_handle,
                                                    data_id_handle,
                                                    href_handle,
                                                    data_url_handle,
                                                ) = second_element(
                                                    &node_element_first_first.children[3],
                                                );
                                                println!("data_hovercard_type_handle2 {data_hovercard_type_handle:#?}");
                                                println!("data_hovercard_url_handle2 {data_hovercard_url_handle:#?}");
                                                println!("data_id_handle2 {data_id_handle:#?}");
                                                println!("href_handle2 {href_handle:#?}");
                                                println!("data_url_handle2 {data_url_handle:#?}");
                                                data_hovercard_type = data_hovercard_type_handle;
                                                data_hovercard_url = data_hovercard_url_handle;
                                                data_id = data_id_handle;
                                                href = href_handle;
                                                data_url = data_url_handle;
                                                let some_trash = handle_text_element(
                                                    //some trash
                                                    &node_element_first_first.children[4],
                                                );
                                                println!("some_trash{some_trash:#?}");
                                            }
                                            _ => {
                                                let len = node_element_first_first.children.len();
                                                let warning_message =
                                                    format!("different children.len(): {len}");
                                                print_colorful_message(
                                                    None,
                                                    PrintType::WarningHigh,
                                                    vec![format!(
                                                        "{}{}{}",
                                                        file!(),
                                                        line!(),
                                                        column!()
                                                    )],
                                                    vec![GIT_INFO.data.get_git_source_file_link(
                                                        file!(),
                                                        line!(),
                                                    )],
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_colorful_message(
                                        None,
                                        PrintType::WarningHigh,
                                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                        vec![GIT_INFO
                                            .data
                                            .get_git_source_file_link(file!(), line!())],
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_colorful_message(
                            None,
                            PrintType::WarningHigh,
                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                            "different node".to_string(),
                        ),
                    }
                }
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        warning_message,
                    )
                }
            }
        }
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("avatar_link {avatar_link:#?}");
    // println!("relative_commit_link {relative_commit_link:#?}");
    // println!("commit_text {commit_text:#?}");
    // println!("from_text {from_text:#?}");
    // println!("commits_number {commits_number:#?}");
    (
        avatar_link,
        relative_commit_link,
        commit_text,
        from_text,
        commits_number,
        None, //issue_label
        data_hovercard_type,
        data_hovercard_url,
        data_id,
        href,
        data_url,
    )
}
fn handle_text_element(node: &Node) -> Option<String> {
    let mut text: Option<String> = None;
    match node {
        Node::Text(ref text_handle) => {
            if !text_handle.is_empty() {
                text = Some(text_handle.to_string());
            }
        }
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    text
}
fn second_element(
    node: &Node,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let mut data_hovercard_type: Option<String> = None;
    let mut data_hovercard_url: Option<String> = None;
    let mut data_id: Option<String> = None;
    let mut href: Option<String> = None;
    let mut data_url: Option<String> = None;
    match node {
        Node::Element(ref node_element) => {
            if let Some(data_hovercard_type_handle_option) =
                node_element.attributes.get("data-hovercard-type")
            {
                match data_hovercard_type_handle_option {
                    Some(data_hovercard_type_handle) => {
                        if !data_hovercard_type_handle.is_empty() {
                            data_hovercard_type = data_hovercard_type_handle_option.clone()
                        }
                    }
                    None => {}
                }
            }
            if let Some(data_hovercard_url_handle_option) =
                node_element.attributes.get("data-hovercard-url")
            {
                match data_hovercard_url_handle_option {
                    Some(data_hovercard_url_handle) => {
                        if !data_hovercard_url_handle.is_empty() {
                            data_hovercard_url = data_hovercard_url_handle_option.clone()
                        }
                    }
                    None => {}
                }
            }
            if let Some(data_id_handle_option) = node_element.attributes.get("data-id") {
                match data_id_handle_option {
                    Some(data_id_handle) => {
                        if !data_id_handle.is_empty() {
                            data_id = data_id_handle_option.clone()
                        }
                    }
                    None => {}
                }
            }
            if let Some(href_handle_option) = node_element.attributes.get("href") {
                match href_handle_option {
                    Some(href_handle) => {
                        if !href_handle.is_empty() {
                            href = href_handle_option.clone()
                        }
                    }
                    None => {}
                }
            }
            if let Some(data_url_handle_option) = node_element.attributes.get("data-url") {
                match data_url_handle_option {
                    Some(data_url_handle) => {
                        if !data_url_handle.is_empty() {
                            data_url = data_url_handle_option.clone()
                        }
                    }
                    None => {}
                }
            }
            //just to check any new formats
            match node_element.children.len() {
                1 => match node_element.children[0] {
                    Node::Text(_) => {
                        //todo what should do with this
                    }
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(_) => {
                                    //todo what should do with this
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                },
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        warning_message,
                    )
                }
            }
        }
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("data_hovercard_type {data_hovercard_type:#?}");
    // println!("data_hovercard_url {data_hovercard_url:#?}");
    // println!("data_id {data_id:#?}");
    // println!("href {href:#?}");
    // println!("data_url {data_url:#?}");
    (
        data_hovercard_type,
        data_hovercard_url,
        data_id,
        href,
        data_url,
    )
}
fn two_elements_one_child(
    node: &Node,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let mut author: Option<String> = None;
    let mut action: Option<String> = None;
    let mut the_accounts_repo_on_which_the_action_was_performed_relative_href: Option<String> =
        None;
    let mut the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from: Option<
        String,
    > = None;
    let mut datejs: Option<String> = None;
    let mut date: Option<String> = None;
    let mut from: Option<String> = None;
    let mut isssue_label: Option<String> = None; //todo or isssuelabel in struct by default in html
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            4 => {
                match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref author_handle) => {
                                    if !author_handle.is_empty() {
                                        author = Some(author_handle.to_string());
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[1] {
                    Node::Text(ref action_handle) => {
                        if !action_handle.is_empty() {
                            action = Some(action_handle.to_string());
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[2] {
                    Node::Element(ref node_element_third) => {
                        let attribute = "href";
                        match node_element_third.attributes.get(attribute) {
                            Some(
                                the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option,
                            ) => {
                                match the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option {
                                    Some(the_accounts_repo_on_which_the_action_was_performed_relative_href_handle) => {
                                        if !the_accounts_repo_on_which_the_action_was_performed_relative_href_handle.is_empty() {
                                            the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                            the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option.clone();
                                        }
                                    }
                                    None => {}
                                }
                            }
                            None => {
                                let warning_message = format!("no {attribute} attribute");
                                print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[3] {
                    Node::Element(ref node_element_fourth) => {
                        match node_element_fourth.children.len() {
                            1 => match node_element_fourth.children[0] {
                                Node::Element(ref node_element_fourth_element) => {
                                    let attribute = "datetime";
                                    match node_element_fourth_element.attributes.get(attribute) {
                                        Some(datejs_handle_option) => match datejs_handle_option {
                                            Some(datejs_handle) => {
                                                if !datejs_handle.is_empty() {
                                                    datejs = datejs_handle_option.clone();
                                                }
                                            }
                                            None => {}
                                        },
                                        None => {
                                            let warning_message =
                                                format!("no {attribute} attribute");
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                    match node_element_fourth_element.children.len() {
                                        1 => match node_element_fourth_element.children[0] {
                                            Node::Text(ref date_handle) => {
                                                if !date_handle.is_empty() {
                                                    date = Some(date_handle.to_string());
                                                }
                                            }
                                            _ => print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                "different node".to_string(),
                                            ),
                                        },
                                        _ => {
                                            let warning_message = format!(
                                                "different children.len(): {}",
                                                node_element_fourth_element.children.len()
                                            );
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_fourth.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
            }
            5 => {
                match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref author_handle) => {
                                    if !author_handle.is_empty() {
                                        author = Some(author_handle.to_string());
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[1] {
                    Node::Text(ref action_handle) => {
                        if !action_handle.is_empty() {
                            action = Some(action_handle.to_string());
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[2] {
                    Node::Element(ref node_element_third) => {
                        let attribute = "href";
                        match node_element_third.attributes.get(attribute) {
                                Some(
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option,
                                ) => {
                                    match the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option {
                                        Some(the_accounts_repo_on_which_the_action_was_performed_relative_href_handle) => {
                                            if !the_accounts_repo_on_which_the_action_was_performed_relative_href_handle.is_empty() {
                                                 the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                                 the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option.clone();
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {
                                    let warning_message = format!("no {attribute} attribute");
                                    print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                }
                            }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[3] {
                    Node::Text(ref from_handle) => {
                        if !from_handle.is_empty() {
                            from = Some(from_handle.to_string());
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[4] {
                    Node::Element(ref node_element_firth) => {
                        match node_element_firth.children.len() {
                            1 => match node_element_firth.children[0] {
                                Node::Element(ref node_element_firth_element) => {
                                    let attribute = "datetime";
                                    match node_element_firth_element.attributes.get(attribute) {
                                        Some(datejs_handle_option) => match datejs_handle_option {
                                            Some(datejs_handle) => {
                                                if !datejs_handle.is_empty() {
                                                    datejs = datejs_handle_option.clone();
                                                }
                                            }
                                            None => {}
                                        },
                                        None => print_colorful_message(
                                            None,
                                            PrintType::WarningHigh,
                                            vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                            vec![GIT_INFO
                                                .data
                                                .get_git_source_file_link(file!(), line!())],
                                            format!("no {attribute} attrubite"),
                                        ),
                                    }
                                    match node_element_firth_element.children.len() {
                                        1 => match node_element_firth_element.children[0] {
                                            Node::Text(ref date_handle) => {
                                                if !date_handle.is_empty() {
                                                    date = Some(date_handle.to_string());
                                                }
                                            }
                                            _ => print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                "different node".to_string(),
                                            ),
                                        },
                                        _ => {
                                            let warning_message = format!(
                                                "different children.len(): {}",
                                                node_element_firth_element.children.len()
                                            );
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_firth.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
            }
            6 => {
                match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref author_handle) => {
                                    if !author_handle.is_empty() {
                                        author = Some(author_handle.to_string());
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[1] {
                    Node::Text(ref action_handle) => {
                        if !action_handle.is_empty() {
                            action = Some(action_handle.to_string());
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[2] {
                    Node::Element(ref node_element_third) => {
                        let attribute = "href";
                        let attribute_check_case = "data-view-component";
                        match node_element_third.attributes.get(attribute) {
                                Some(the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option) => {
                                    match the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option {
                                        Some(the_accounts_repo_on_which_the_action_was_performed_relative_href_handle) => {
                                            if !the_accounts_repo_on_which_the_action_was_performed_relative_href_handle.is_empty() {
                                                  the_accounts_repo_on_which_the_action_was_performed_relative_href = the_accounts_repo_on_which_the_action_was_performed_relative_href_handle_option.clone();
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {
                                    match node_element_third.attributes.get(attribute_check_case) {
                                        Some(_) => {}
                                        None => {
                                            let warning_message = format!("no {attribute} attribute or {attribute_check_case} attribute");
                                            print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    );
                                        }
                                    }
                                }
                            }
                        match node_element_third.children.len() {
                            1 => match node_element_third.children[0] {
                                Node::Text(ref isssue_label_handle) => {
                                    if !isssue_label_handle.is_empty() {
                                        isssue_label = Some(isssue_label_handle.to_string());
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_third.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[3] {
                    Node::Text(ref from_handle) => {
                        if !from_handle.is_empty() {
                            from = Some(from_handle.to_string());
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[4] {
                    Node::Element(ref node_element_firth) => {
                        let attribute = "href";
                        match node_element_firth.attributes.get(attribute) {
                                Some(
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle_option,
                                ) => {
                                    match the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle_option {
                                        Some(the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle) => {
                                            if !the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle.is_empty() {
                                                the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from = the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle_option.clone();
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                None => {
                                    let warning_message = format!("no {attribute} attribute");
                                    print_colorful_message(
                                                    None,
        PrintType::WarningHigh,
        vec![format!("{}:{}:{}", file!(), line!(), column!())],
        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
        warning_message,
    )
                                }
                            }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[5] {
                    Node::Element(ref node_element_sixth) => {
                        match node_element_sixth.children.len() {
                            1 => match node_element_sixth.children[0] {
                                Node::Element(ref node_element_sixth_first) => {
                                    let attribute = "datetime";
                                    match node_element_sixth_first.attributes.get(attribute) {
                                        Some(datejs_handle_option) => match datejs_handle_option {
                                            Some(datejs_handle) => {
                                                if !datejs_handle.is_empty() {
                                                    datejs = datejs_handle_option.clone();
                                                }
                                            }
                                            None => {
                                                //todo
                                            }
                                        },
                                        None => {
                                            let warning_message =
                                                format!("no {attribute} attribute");
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                    match node_element_sixth_first.children.len() {
                                        1 => match node_element_sixth_first.children[0] {
                                            Node::Text(ref date_handle) => {
                                                if !date_handle.is_empty() {
                                                    date = Some(date_handle.to_string());
                                                }
                                            }
                                            _ => print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                "different node".to_string(),
                                            ),
                                        },
                                        _ => {
                                            let warning_message = format!(
                                                "different children.len(): {}",
                                                node_element_sixth_first.children.len()
                                            );
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_sixth.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
            }
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("-----------------");
    // println!("author_name {author_name:#?}");
    // println!("action {action:#?}");
    // println!("the_accounts_repo_on_which_the_action_was_performed_relative_href {the_accounts_repo_on_which_the_action_was_performed_relative_href:#?}");
    // println!("the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from {the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from:#?}");
    // println!("datejs {datejs:#?}");
    // println!("date {date}");
    // println!("from {from}");
    // println!("isssue_label {isssue_label}");
    (
        author,
        action,
        the_accounts_repo_on_which_the_action_was_performed_relative_href,
        the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from,
        datejs,
        date,
        from,
        isssue_label,
    )
}

fn two_elements_four_children_first(node: &Node) -> Option<String> {
    let mut author_another_another: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Text(ref author_another_another_handle) => {
                    if !author_another_another_handle.is_empty() {
                        author_another_another = Some(author_another_another_handle.to_string());
                    }
                }
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("author_another_another {author_another_another}");
    author_another_another
}

fn two_elements_four_children_second(node: &Node) -> Option<String> {
    let mut action_another_another: Option<String> = None;
    match node {
        Node::Text(ref action_another_another_handle) => {
            if !action_another_another_handle.is_empty() {
                action_another_another = Some(action_another_another_handle.to_string());
            }
        }
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("action_another_another {action_another_another}");
    action_another_another
}

fn two_elements_four_children_third(node: &Node) -> Option<String> {
    let mut who_follow: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Text(ref who_follow_handle) => {
                    if !who_follow_handle.is_empty() {
                        who_follow = Some(who_follow_handle.to_string());
                    }
                }
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("who_follow {who_follow}");
    who_follow
}

fn two_elements_four_children_fourth(node: &Node) -> (Option<String>, Option<String>) {
    let mut datejs: Option<String> = None;
    let mut date: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Element(ref node_element_first) => {
                    let attribute = "datetime";
                    match node_element_first.attributes.get(attribute) {
                        Some(datejs_handle_option) => match datejs_handle_option {
                            Some(datejs_handle) => {
                                if !datejs_handle.is_empty() {
                                    datejs = datejs_handle_option.clone();
                                }
                            }
                            None => {}
                        },
                        None => {
                            let warning_message = format!("no {attribute} attribute");
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                warning_message,
                            )
                        }
                    }
                    match node_element_first.children.len() {
                        1 => match node_element_first.children[0] {
                            Node::Text(ref date_handle) => {
                                if !date_handle.is_empty() {
                                    date = Some(date_handle.to_string());
                                }
                            }
                            _ => print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                "different node".to_string(),
                            ),
                        },
                        _ => {
                            let warning_message = format!(
                                "different children.len(): {}",
                                node_element_first.children.len()
                            );
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                warning_message,
                            )
                        }
                    }
                }
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("datejs {datejs:#?}");
    // println!("date {date:#?}");
    (datejs, date)
}

fn parse_github_html_second_part_two_children_first(
    node: &Node,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let mut author: Option<String> = None;
    let mut action: Option<String> = None;
    let mut repository: Option<String> = None;
    let mut from: Option<String> = None;
    let mut from_what_repository_forked: Option<String> = None;
    let mut datejs: Option<String> = None;
    let mut date: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            4 => {
                match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref author_handle) => {
                                    if !author_handle.is_empty() {
                                        author = Some(author_handle.to_string())
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[1] {
                    Node::Text(ref action_handle) => {
                        if !action_handle.is_empty() {
                            action = Some(action_handle.to_string())
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[2] {
                    Node::Element(ref node_element_third) => {
                        match node_element_third.children.len() {
                            1 => match node_element_third.children[0] {
                                Node::Text(ref repository_handle) => {
                                    if !repository_handle.is_empty() {
                                        repository = Some(repository_handle.to_string())
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_third.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[3] {
                    Node::Element(ref node_element_fourth) => {
                        match node_element_fourth.children.len() {
                            1 => match node_element_fourth.children[0] {
                                Node::Element(ref node_element_fourth_first) => {
                                    let attribute = "datetime";
                                    match node_element_fourth_first.attributes.get(attribute) {
                                        Some(datejs_handle_option) => match datejs_handle_option {
                                            Some(datejs_handle) => {
                                                if !datejs_handle.is_empty() {
                                                    datejs = datejs_handle_option.clone();
                                                }
                                            }
                                            None => {}
                                        },
                                        None => {
                                            let warning_message =
                                                format!("no {attribute} attribute");
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                    match node_element_fourth_first.children.len() {
                                        1 => match node_element_fourth_first.children[0] {
                                            Node::Text(ref date_handle) => {
                                                if !date_handle.is_empty() {
                                                    date = Some(date_handle.to_string());
                                                }
                                            }
                                            _ => print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                "different node".to_string(),
                                            ),
                                        },
                                        _ => {
                                            let warning_message = format!(
                                                "different children.len(): {}",
                                                node_element_fourth_first.children.len()
                                            );
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_fourth.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
            }
            5 => {
                println!("todo 5 elements github parsing")
            }
            6 => {
                match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref author_handle) => {
                                    if !author_handle.is_empty() {
                                        author = Some(author_handle.to_string())
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[1] {
                    Node::Text(ref action_handle) => {
                        if !action_handle.is_empty() {
                            action = Some(action_handle.to_string())
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[2] {
                    Node::Element(ref node_element_third) => {
                        match node_element_third.children.len() {
                            1 => match node_element_third.children[0] {
                                Node::Text(ref repository_handle) => {
                                    if !repository_handle.is_empty() {
                                        repository = Some(repository_handle.to_string())
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_third.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[3] {
                    Node::Text(ref from_handle) => {
                        if !from_handle.is_empty() {
                            from = Some(from_handle.to_string())
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[4] {
                    Node::Element(ref node_element_firth) => {
                        match node_element_firth.children.len() {
                            1 => match node_element_firth.children[0] {
                                Node::Text(ref from_what_repository_forked_handle) => {
                                    if !from_what_repository_forked_handle.is_empty() {
                                        from_what_repository_forked =
                                            Some(from_what_repository_forked_handle.to_string())
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_firth.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
                match node_element.children[5] {
                    Node::Element(ref node_element_sixth) => {
                        match node_element_sixth.children.len() {
                            1 => match node_element_sixth.children[0] {
                                Node::Element(ref node_element_sixth_first) => {
                                    let attribute = "datetime";
                                    match node_element_sixth_first.attributes.get(attribute) {
                                        Some(datejs_handle_option) => match datejs_handle_option {
                                            Some(datejs_handle) => {
                                                if !datejs_handle.is_empty() {
                                                    datejs = datejs_handle_option.clone();
                                                }
                                            }
                                            None => {}
                                        },
                                        None => {
                                            let warning_message =
                                                format!("no {attribute} attribute");
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                    match node_element_sixth_first.children.len() {
                                        1 => match node_element_sixth_first.children[0] {
                                            Node::Text(ref date_handle) => {
                                                if !date_handle.is_empty() {
                                                    date = Some(date_handle.to_string());
                                                }
                                            }
                                            _ => print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                "different node".to_string(),
                                            ),
                                        },
                                        _ => {
                                            let warning_message = format!(
                                                "different children.len(): {}",
                                                node_element_sixth_first.children.len()
                                            );
                                            print_colorful_message(
                                                None,
                                                PrintType::WarningHigh,
                                                vec![format!(
                                                    "{}{}{}",
                                                    file!(),
                                                    line!(),
                                                    column!()
                                                )],
                                                vec![GIT_INFO
                                                    .data
                                                    .get_git_source_file_link(file!(), line!())],
                                                warning_message,
                                            )
                                        }
                                    }
                                }
                                _ => print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_sixth.children.len()
                                );
                                print_colorful_message(
                                    None,
                                    PrintType::WarningHigh,
                                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_colorful_message(
                        None,
                        PrintType::WarningHigh,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                        "different node".to_string(),
                    ),
                }
            }
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("author {author:#?}");
    // println!("action {action:#?}");
    // println!("repository {repository:#?}");
    // println!("from {from:#?}");
    // println!("from_what_repository_forked {from_what_repository_forked:#?}");
    // println!("datejs {datejs:#?}");
    // println!("date {date:#?}");
    (
        author,
        action,
        repository,
        from,
        from_what_repository_forked,
        datejs,
        date,
    )
}

fn four_cases(
    vec_of_nodes: &[Node],
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let mut author: Option<String> = None;
    let mut action: Option<String> = None;
    let mut who_follow: Option<String> = None;
    let mut datejs: Option<String> = None;
    let mut date: Option<String> = None;
    match vec_of_nodes[0] {
        Node::Element(ref first_element) => match first_element.children.len() {
            1 => match first_element.children[0] {
                Node::Text(ref author_handle) => {
                    if !author_handle.is_empty() {
                        author = Some(author_handle.to_string())
                    }
                }
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", first_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    match vec_of_nodes[1] {
        Node::Text(ref action_handle) => {
            if !action_handle.is_empty() {
                action = Some(action_handle.to_string())
            }
        }
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    match vec_of_nodes[2] {
        Node::Element(ref third_element) => match third_element.children.len() {
            1 => match third_element.children[0] {
                Node::Text(ref who_follow_handle) => {
                    if !who_follow_handle.is_empty() {
                        who_follow = Some(who_follow_handle.to_string())
                    }
                }
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", third_element.children.len());
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    match vec_of_nodes[3] {
        Node::Element(ref node_element_fourth) => match node_element_fourth.children.len() {
            1 => match node_element_fourth.children[0] {
                Node::Element(ref node_element_fourth) => {
                    let attribute = "datetime";
                    match node_element_fourth.attributes.get(attribute) {
                        Some(option_datejs_handle) => match option_datejs_handle {
                            Some(datejs_handle) => {
                                if !datejs_handle.is_empty() {
                                    datejs = option_datejs_handle.clone();
                                }
                            }
                            None => {}
                        },
                        None => {
                            let warning_message = format!("no {attribute} attribute");
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                warning_message,
                            )
                        }
                    }

                    match node_element_fourth.children.len() {
                        1 => match node_element_fourth.children[0] {
                            Node::Text(ref second_child_element5fourth) => {
                                if !second_child_element5fourth.is_empty() {
                                    date = Some(second_child_element5fourth.to_string());
                                }
                            }
                            _ => print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                "different node".to_string(),
                            ),
                        },
                        _ => {
                            let warning_message = format!(
                                "different children.len(): {}",
                                node_element_fourth.children.len()
                            );
                            print_colorful_message(
                                None,
                                PrintType::WarningHigh,
                                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                                vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                                warning_message,
                            )
                        }
                    }
                }
                _ => print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message = format!(
                    "different children.len(): {}",
                    node_element_fourth.children.len()
                );
                print_colorful_message(
                    None,
                    PrintType::WarningHigh,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
                    warning_message,
                )
            }
        },
        _ => print_colorful_message(
            None,
            PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.data.get_git_source_file_link(file!(), line!())],
            "different node".to_string(),
        ),
    }
    // println!("author {author:#?}");
    // println!("action {action:#?}");
    // println!("who_follow {who_follow:#?}");
    // println!("datejs {datejs:#?}");
    // println!("date {date:#?}");
    (author, action, who_follow, datejs, date)
}
