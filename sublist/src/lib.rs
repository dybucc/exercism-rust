#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn loop_compare(first_list: &[i32], second_list: &[i32]) -> bool {
    let mut check = false;

    for (init, _) in first_list
        .iter()
        // unwrap is safe; the function is called knowing both lists are non-empty
        .skip_while(|k| *k == second_list.first().unwrap())
        .enumerate()
    {
        let slice = first_list.get(init..init + second_list.len());

        match slice {
            Some(slice) => {
                if slice == second_list {
                    check = true;
                    break;
                }
            }
            None => break,
        }
    }

    check
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match first_list {
        // check first if the lists are equal, both empty, or some empty
        _ if first_list == second_list => Comparison::Equal,
        _ if first_list.is_empty() && !second_list.is_empty() => Comparison::Sublist,
        _ if !first_list.is_empty() && second_list.is_empty() => Comparison::Superlist,
        _ => {
            // at this point, none of the lists are empty
            if loop_compare(second_list, first_list) {
                Comparison::Sublist
            } else if loop_compare(first_list, second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
