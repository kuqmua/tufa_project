pub fn column_names_factorial<'a>(
    original_input: std::vec::Vec<(usize, &syn::Field)>,
    input: std::vec::Vec<&'a syn::Field>,
    output: &mut [std::vec::Vec<&'a syn::Field>],
    proc_macro_name_upper_camel_case_ident_stringified: &std::string::String,
) -> std::vec::Vec<std::vec::Vec<&'a syn::Field>> {
    //todo optimize
    match input.len() {
        0 => {
            let mut end_out = {
                let output_len = output.len();
                output
                    .iter_mut()
                    .fold(std::vec::Vec::with_capacity(output_len), |mut acc, element| {
                        element.sort_by(|a, b| {
                            let (index_a, _) = original_input
                                .iter()
                                .find(|(_, field)| {
                                        a.ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                        })
                                        == field
                                            .ident
                                            .as_ref()
                                            .unwrap_or_else(|| {
                                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                            })
                                })
                                .unwrap_or_else(|| {
                                    panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} cannot find original input index"
                                    )
                                });
                            let (index_b, _) = original_input
                                .iter()
                                .find(|(_, field)| {
                                    b.ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                        })
                                        == field
                                            .ident
                                            .as_ref()
                                            .unwrap_or_else(|| {
                                                panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                            })
                                })
                                .unwrap_or_else(|| {
                                    panic!(
                                        "{proc_macro_name_upper_camel_case_ident_stringified} cannot find original input index"
                                    )
                                });
                            index_a.partial_cmp(index_b).unwrap_or_else(|| {
                                panic!(
                                    "{proc_macro_name_upper_camel_case_ident_stringified} index_a.partial_cmp(index_b) is None"
                                )
                            })
                        });
                        acc.push(element.to_vec());
                        acc
                    })
            };
            end_out.sort_by(|a, b| match a.len() == b.len() {
                true => {
                    let mut order = std::cmp::Ordering::Equal;
                    for a_elem in a {
                        let mut is_order_found = false;
                        for b_elem in a {
                            if let Some(or) = a_elem
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                })
                                .to_string()
                                .partial_cmp(
                                    &b_elem
                                        .ident
                                        .as_ref()
                                        .unwrap_or_else(|| {
                                            panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                        })
                                        .to_string(),
                                )
                            {
                                match or {
                                    std::cmp::Ordering::Less => {
                                        order = or;
                                        is_order_found = true;
                                        break;
                                    }
                                    std::cmp::Ordering::Equal => (),
                                    std::cmp::Ordering::Greater => {
                                        order = or;
                                        is_order_found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if is_order_found {
                            break;
                        }
                    }
                    order
                }
                false => std::cmp::Ordering::Equal,
            });
            end_out.sort_by(|a, b| {
                a.len().partial_cmp(&b.len()).unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} index_a.partial_cmp(index_b) is None"
                    )
                })
            });
            end_out
        }
        // 1 => {
        //     let mut output_handle = vec![];
        //     original_input.iter().for_each(|(_, element)| {
        //         output_handle.push(vec![element.clone()]);
        //     });
        //     let first_element = input.get(0).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} input.get(0) is None"));
        //     output.iter().for_each(
        //         |o| {
        //             if let false = o.contains(first_element) {
        //                 let mut cl = o.clone();
        //                 cl.push(format!("{}", input[0]));
        //                 cl.sort_by(|a,b|{
        //                     let (index_a, _) = original_input.iter().find(|(_, field)|{a == field}).unwrap();
        //                     let (index_b, _) = original_input.iter().find(|(_, field)|{b == field}).unwrap();
        //                     index_a.partial_cmp(index_b).unwrap()
        //                 });
        //                 output_handle.push(cl);
        //             }
        //         },
        //     );
        //     output_handle
        // }
        _ => {
            let mut output_handle = {
                let first_element = input.first().unwrap_or_else(|| {
                    panic!(
                        "{proc_macro_name_upper_camel_case_ident_stringified} input.first() is None"
                    )
                });
                let output_len = output.len();
                output.iter_mut().fold(std::vec::Vec::with_capacity(output_len * 2), |mut acc, out| {
                    if !acc.contains(out) {
                        out.sort_by(|a,b|{
                            let (index_a, _) = original_input.iter().find(|(_, field)|{a.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                }) == field
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                })
                            }).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot find original input index"));
                            let (index_b, _) = original_input.iter().find(|(_, field)|{b.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                }) == field
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                })
                            }).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot find original input index"));
                            index_a.partial_cmp(index_b).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} index_a.partial_cmp(index_b) is None"))
                        });
                        acc.push(out.clone());
                    }
                    if !out.contains(first_element) {
                        let mut cl = out.clone();
                        cl.push(first_element);
                        cl.sort_by(|a,b|{
                            let (index_a, _) = original_input.iter().find(|(_, field)|{a.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                }) == field
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                })
                            }).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot find original input index"));
                            let (index_b, _) = original_input.iter().find(|(_, field)|{b.ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                }) == field
                                .ident
                                .as_ref()
                                .unwrap_or_else(|| {
                                    panic!("{proc_macro_name_upper_camel_case_ident_stringified} field.ident is None")
                                })
                            }).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} cannot find original input index"));
                            index_a.partial_cmp(index_b).unwrap_or_else(||panic!("{proc_macro_name_upper_camel_case_ident_stringified} index_a.partial_cmp(index_b) is None"))
                        });
                        if !acc.contains(&cl) {
                            acc.push(cl);
                        }
                    }
                    acc
                })
            };
            let new_input_vec = {
                let input_len = input.len();
                input.into_iter().enumerate().fold(
                    std::vec::Vec::with_capacity(input_len),
                    |mut acc, (index, value)| {
                        if index != 0 {
                            acc.push(value);
                        }
                        acc
                    },
                )
            };
            column_names_factorial(
                original_input,
                new_input_vec,
                &mut output_handle as &mut [std::vec::Vec<&syn::Field>],
                proc_macro_name_upper_camel_case_ident_stringified,
            )
        }
    }
}
