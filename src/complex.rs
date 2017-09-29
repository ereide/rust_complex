
use std::ops;
use std::cmp;

pub struct Polar {
    pub r: f64,
    pub theta: f64,
}

impl Polar {
    pub fn to_complex(&self) -> Complex {
        let com = Complex {
            real: self.r * self.theta.cos(),
            img: self.r * self.theta.sin(),
        };
        com
    }
}

pub struct Complex {
    pub real: f64,
    pub img: f64,
}

impl Complex {
    pub fn new(real : f64, img: f64) -> Complex{
        let ans = Complex {
            real: real, 
            img: img
        };
        ans
    }
	
	pub fn i() -> Complex {
		Complex {
			real: 0.0,
			img: 1.0,
		}
	}
	
	

    pub fn to_polar(&self) -> Polar {
        let r2 = self.modulus();
        let polar = Polar {
            r : (r2).sqrt(),
            theta : (self.real/self.img).atan(),
        };
        polar
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real, 
            img: -self.img
        }
    }

    pub fn exp(&self) -> Complex {
        let pol = self.to_polar();
        let exp_r = pol.r.exp();
        Complex{
            real: exp_r * pol.theta.cos(),
            img:  exp_r * pol.theta.sin(),
        }
    }

    pub fn modulus(&self) -> f64 {
        self.real * self.real + self.img * self.img
    }
	
	pub fn cos(&self) -> Complex {
		(&(self.exp()) + &((-self).exp()))
	}
}

impl<'a> ops::Neg for &'a Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
		//TODO: divide by two
        Complex { real: - self.real, img: -self.img}
    }
}


impl<'a, 'b> ops::Add<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn add(self, other: &'b Complex) -> Complex {
        Complex { 
            real: self.real + other.real, 
            img: self.img + other.img 
        }
    }
}

impl cmp::PartialEq for Complex {
    fn eq(&self, other: & Complex) -> bool {
        ((self.real == other.real) && (self.img == other.img))
    }
}


/*
impl<'a, 'b> ops::Mul<&'b T> for &'a Complex {
    type Output = Complex;

    fn add(self, other: &'b T) -> Complex {
        Complex { 
            real: self.real*other, 
            img: self.img* other, 
        }
    }
}
*/
impl<'a, 'b> ops::Sub<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn sub(self, other: &'b Complex) -> Complex {
        self + &(-other)
		/*Complex { 
            real: self.real - other.real, 
            img: self.img - other.img 
        }*/
    }
}

impl<'a, 'b> ops::Mul<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn mul(self, other: &'b Complex) -> Complex {
        Complex {
			real: self.real*other.real - self.img*other.img, 
			img: self.img * other.real + self.real * other.img
		}
    }
}


impl<'a, 'b> ops::Div<&'b Complex> for &'a Complex {
    type Output = Complex;

    fn div(self, other: &'b Complex) -> Complex {
        let den = other.real * other.real + other.img * other.img;
		let x = (self.real*other.real + self.img*other.img)/den;
		let y = (self.img * other.real - self.real * other.img)/den;
        Complex { 
            real: x, 
            img: y,
        }
		
    }
}

