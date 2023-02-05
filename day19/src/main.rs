// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
// https://nalgebra.org/docs/user_guide/vectors_and_matrices


extern crate nalgebra as na;
use na::{Matrix3, Vector3};

fn main() {
    println!("Hello, world!");

    let mut m = Matrix3::<isize>::new(1, 1, 1,
				2, 2, 2,
				3, 3, 3);
    let n = Matrix3::from_column_slice(&[
	1, 2, 3,
	1, 2, 3,
	1, 2, 3
    ]);
    let o = Matrix3::from_row_slice(&[
	1.0, 2.0, 0.0,
	0.0, 2.0, 0.0,
	0.0, 0.0, 3.0
    ]);

    let v = Vector3::<isize>::new(1, 2, 3);

    println!("{:?} * {:?} = {:?}", m, v, m * v);
    println!("{:?}", n);
    println!("{:?}", o);

    m[(0,1)] = 12;
    println!("{:?}", m[(0,1)]);

    //let decomp = o.lu();
    let inv = o.try_inverse();

}
