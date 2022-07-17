#[cfg(test)]
mod binary_search_tests {
    use super::super::{*};

    #[test]
    fn binary_search_test() {
        let l: [i32; 7] = [1, 3, 4, 5, 6, 8, 9];

        let ind: i32 = binary_search(&l, 1) as i32;
        assert_eq!(ind, 0);

        let ind: i32 = binary_search(&l, 3) as i32;
        assert_eq!(ind, 1);

        let ind: i32 = binary_search(&l, 4) as i32;
        assert_eq!(ind, 2);

        let ind: i32 = binary_search(&l, 5) as i32;
        assert_eq!(ind, 3);

        let ind: i32 = binary_search(&l, 6) as i32;
        assert_eq!(ind, 4);

        let ind: i32 = binary_search(&l, 8) as i32;
        assert_eq!(ind, 5);

        let ind: i32 = binary_search(&l, 9) as i32;
        assert_eq!(ind, 6);
    }
}