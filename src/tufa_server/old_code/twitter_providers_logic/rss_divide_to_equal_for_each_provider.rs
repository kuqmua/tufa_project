extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

pub fn rss_divide_to_equal_for_each_provider(
    rss_available_providers_links: Vec<String>,
    links_temp_naming: Vec<String>,
    links_len: usize,
) -> Vec<Vec<String>> {
    let rss_available_providers_links_len = rss_available_providers_links.len();
    let links_for_each_provider: usize;
    let is_links_len_more_than_rss_available_providers_links_len =
        links_len > rss_available_providers_links_len;
    let vec_of_hashmap_parts_len: usize;
    if is_links_len_more_than_rss_available_providers_links_len {
        if links_len % rss_available_providers_links_len == 0 {
            links_for_each_provider = links_len / rss_available_providers_links_len;
        } else {
            //little bit more memory usage than needed but no second allocation!
            links_for_each_provider = (links_len / rss_available_providers_links_len) + 1;
        }
        vec_of_hashmap_parts_len = rss_available_providers_links_len;
    } else {
        links_for_each_provider = links_len;
        vec_of_hashmap_parts_len = links_len;
    }
    let mut vec_of_hashmap_parts: Vec<Vec<String>> =
        vec![Vec::with_capacity(links_for_each_provider); vec_of_hashmap_parts_len];
    let mut vec_of_hashmap_parts_element_index_counter = 0;
    let mut even_vec_of_hashmap_parts_element_index_counter = 0;
    let mut even_flag = false;
    if is_links_len_more_than_rss_available_providers_links_len {
        for element in links_temp_naming {
            if !even_flag {
                if vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter].len()
                    == links_for_each_provider
                {
                    if (vec_of_hashmap_parts.len() - 1)
                        != vec_of_hashmap_parts_element_index_counter
                    {
                        vec_of_hashmap_parts_element_index_counter += 1;
                        vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter]
                            .push(element);
                    } else {
                        even_flag = true;
                        vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
                            .push(element);
                        even_vec_of_hashmap_parts_element_index_counter += 1;
                    }
                } else {
                    vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter].push(element);
                }
            } else if (vec_of_hashmap_parts.len() - 1)
                != even_vec_of_hashmap_parts_element_index_counter
            {
                even_vec_of_hashmap_parts_element_index_counter += 1;
                vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter].push(element);
            } else {
                vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter].push(element);
                even_vec_of_hashmap_parts_element_index_counter = 0;
            }
        }
    } else {
        for (element_index, element) in links_temp_naming.into_iter().enumerate() {
            vec_of_hashmap_parts[element_index].push(element);
        }
    }
    vec_of_hashmap_parts
}
