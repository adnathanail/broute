#[cfg(test)]
#[path = "search_and_sort_tests.rs"]
mod search_and_sort_tests;

pub fn binary_search(list_to_search: &[i32], len: usize, value_to_find: &i32) -> Option<usize> {
    let mut lower_bound: i32 = 0;
    let mut upper_bound: i32 = len as i32 - 1;

    while lower_bound <= upper_bound {
        let next_guess_index = ((upper_bound - lower_bound) / 2) + lower_bound;
        let next_guess_value = &list_to_search[next_guess_index as usize];

        if next_guess_value == value_to_find {
            return Some(next_guess_index as usize);
        }

        if next_guess_value < value_to_find {
            lower_bound = next_guess_index + 1;
        }

        if next_guess_value > value_to_find {
            upper_bound = next_guess_index - 1;
        }
    }

    None
}
