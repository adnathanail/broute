mod practice;

fn main() {
    let l: [i32; 7] = [1, 3, 4, 5, 6, 8, 9];
    let i = 4;

    let ind: i32 = practice::binary_search::binary_search(&l, i) as i32;

    println!("{} is at position {} in the list {:?}", i, ind, l);
}
