mod practice;

fn main() {
    let inp = vec![4, 2, 6, 8, 1, 6, 8];

    let mut out = Vec::<i32>::new();
    practice::search_and_sort::insertion_sort(&inp, &mut out);

    println!("{:?} sorted is {:?}", inp, out);
}
