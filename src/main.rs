mod practice;

fn main() {
    let l: [i32; 7] = [1, 3, 4, 5, 6, 8, 9];
    let i = 4;

    let ind = practice::search_and_sort::binary_search(&l, 7, &i);

    println!("{} is at position {:?} in the list {:?}", i, ind, l);
}
