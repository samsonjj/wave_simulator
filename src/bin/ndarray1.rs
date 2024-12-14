use ndarray::prelude::*;

fn main() {
    let a0: Array0<f32> = Array0::default(());
    println!("{a0:?}");

    let a1_1: Array1<f32> = Array1::from_vec(vec![1., 2., 3., 4., 5.]);
    let a1_2: Array1<f32> = arr1(&[6., 7., -3., -4., -5.]);

    dbg!(&a1_1 + &a1_2);
    dbg!(a1_1.slice(s![1..3]));
    dbg!(a1_1.slice(s![1..-1]));
}