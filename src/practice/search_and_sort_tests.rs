#[cfg(test)]
mod search_and_sort_tests {
    use super::super::*;

    #[test]
    fn binary_search_test() {
        let l: [i32; 7] = [1, 3, 4, 5, 6, 8, 9];

        let i = 1;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(0));

        let i = 2;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, None);

        let i = 3;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(1));

        let i = 4;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(2));

        let i = 5;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(3));

        let i = 6;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(4));

        let i = 7;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, None);

        let i = 8;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(5));

        let i = 9;
        let ind = binary_search(&l, 7, &i);
        assert_eq!(ind, Some(6));
    }
}
