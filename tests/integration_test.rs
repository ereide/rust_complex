extern crate rcomplex;
use rcomplex::complex::Complex;

#[test]
fn all_operations() {
    let c1 = Complex::new(1.0, 1.0);

    assert!(c1 == c1);
}
