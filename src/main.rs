extern crate complex;


fn main() {
    println!("Hello, world!");

    let c1 = complex::complex::Complex::new(1.0, 5.0);
    let c2 = complex::complex::Complex::new(3.0, 2.0);
    let var = (c1 - c2).conjugate();
    println!("real = {}, img = {}", var.real, var.img);
    let pol = var.to_polar();

    println!("r = {}, theta = {}", pol.r, pol.theta);


}
