use ndarray::{Array, array, s};

fn main() {
    let a1 = Array::from_vec(vec![1.0, 2.0, 3.0]);

    println!("a1: {a1}");

    // Element-wise operations
    println!("a1 + 1.0: {}", &a1 + 1.0);
    println!("a1 * 2.0: {}", &a1 * 2.0);

    println!("sum a1: {}", a1.sum());

    // Element-wise array addition
    let a2 = array![2.0, 5.0, 3.0];
    println!("a2: {a2}");
    println!("a1 + a2: {}", &a1 + &a2);

    // 2D matrix
    let m1 = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    println!("m1: {}", m1);
    println!("m1 shape: {:?}", m1.shape());

    // Slices (.. = axis 0 full range, 1.. = axis 1 second element to the end)
    println!("m1 last 2 columns: {:}", m1.slice(s![.., 1..]));
}
