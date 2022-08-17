use super::*;

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

#[test]
fn test_insertion_sort() {
    let inp = vec![4, 2, 6, 8, 1, 6, 8];

    let mut out = Vec::<i32>::new();
    insertion_sort(&inp, &mut out);

    println!("{:?} sorted is {:?}", inp, out);
    assert_eq!(out, vec![1, 2, 4, 6, 6, 8, 8]);
}