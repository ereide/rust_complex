pub mod complex; 


#[cfg(test)]
mod tests {
	use super::*;
    fn create_complex_pair() -> (complex::Complex, complex::Complex)
	{
		let c1 = complex::Complex::new(1.0, 5.0);
		let c2 = complex::Complex::new(3.0, 2.0);
		(c1, c2)
	}
	
	#[test]
	fn generation() {
		let pol = complex::Polar {
			r: 1.0,
			theta: 0.0,
		};
		let actual = pol.to_complex();
		let expected = complex::Complex::new(1.0, 0.0);
		assert!(actual == expected);

		let actual = complex::Complex::i();
		let expected = complex::Complex::new(0.0, 1.0);
		assert!(actual == expected);
	}
	
	#[test]
	fn comparison() {
		let (c1, c2) = create_complex_pair();
		let c3 = complex::Complex::new(c1.real, c1.img);
		assert!(c1 != c2);
		assert!(c1 == c3);
	}
	
	#[test]
    fn arithmetic() {
		let (c1, c2) = create_complex_pair();
		
		let actual = c1 + c2;
		let expected = complex::Complex::new(4.0, 7.0);
		assert!(actual == expected);

		let actual = c1 - c2;
		let expected = complex::Complex::new(-2.0, 3.0);
		assert!(actual == expected);		

		let actual = c1 * c2;
		let expected = complex::Complex::new(-7.0, 17.0);
		assert!(actual == expected);		

		let actual = c1 / c2;
		let expected = complex::Complex::new(1.0, 1.0);
		assert!(actual == expected);			
    }
	
	#[test]
	fn functions() {
		let (c1, _) = create_complex_pair();
				
		
		let actual = c1.conjugate();
		let expected = complex::Complex::new(c1.real, -c1.img);
		assert!(actual == expected);	

		
		let c4 = complex::Complex::new(3.0, 1.0);
		
		let actual = c4.cos();
		let expected = complex::Complex::new(((c4.real).cos()*(c4.img).cosh()), (-(c4.real).sin()*(c4.img).sinh()));
		assert!(actual == expected);	
		
		let actual = c4.sin();
		let expected = complex::Complex::new((c4.real).sin()*(c4.img).cosh(), -(c4.real).cos()*(c4.img).sinh());
		assert!(actual == expected);	
	}	
}
