#[cfg(test)]
#[path = "binary_search_tests.rs"]
mod binary_search_tests;

pub fn binary_search(list_to_search: &[i32], value_to_find: i32) -> usize {
    let mut lower_bound: usize = 0;
    let mut upper_bound: usize = list_to_search.len();

    let mut next_guess: usize = ((upper_bound - lower_bound) / 2) + lower_bound;

    while list_to_search[next_guess] != value_to_find {
        if lower_bound == upper_bound {
            return usize::MAX;
        }

        if list_to_search[next_guess] < value_to_find {
            lower_bound = next_guess;
        } else {
            upper_bound = next_guess;
        }

        next_guess = ((upper_bound - lower_bound) / 2) + lower_bound;
    }

    next_guess
}