extern crate rcomplex;
use rcomplex::complex::Complex;


fn main() {
    println!("Hello, world!");

    let c1 = Complex::new(1.0, 5.0);
    let c2 = Complex::new(3.0, 2.0);
	let var = c1 * c2;
    println!("real = {}, img = {}", var.real, var.img);
    let pol = var.to_polar();

    println!("r = {}, theta = {}", pol.r, pol.theta);
	
}
