extern crate l1_solver;
extern crate nalgebra as na;

use l1_solver::l1_norm_approx;
use l1_solver::io::{load_matrix,load_vector};


fn main() {

let mut x = load_vector("data/x0.txt");
let y = load_vector("data/y.txt");
let G = load_matrix("data/G.txt");

l1_norm_approx(&y,&G,&mut x, 200, 1e-4);

println!("{}",x);


}
