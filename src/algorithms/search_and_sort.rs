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

pub fn insertion_sort(vector_to_sort: &Vec<i32>, out: &mut Vec<i32>) {
    if vector_to_sort.is_empty() {
        return;
    }
    out.push(vector_to_sort[0]);
    for i in vector_to_sort.iter().skip(1) {
        let mut j = 0;
        while j < out.len() && *i >= out[j] {
            j += 1;
        }
        out.insert(j, *i);
    }
}
