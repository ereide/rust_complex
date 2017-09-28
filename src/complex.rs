
use std::ops;


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
}


impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex { 
            real: self.real + other.real, 
            img: self.img + other.img 
        }
    }
}

impl ops::Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        self + (-other)
    }
}

impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex { real: self.real*other.real - self.img*self.img, img: self.img * other.real + other.img * self.img}
    }
}

impl ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex { real: - self.real, img: -self.img}
    }
}

impl ops::Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let den = other.real * other.real + other.img * other.img;
        Complex { 
            real: (self.real*other.real + self.img*other.img)/den, 
            img: (self.img * other.real - self.real * other.img)/den,
        }
    }
}

