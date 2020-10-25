use std::ops::{Add, AddAssign, Neg};

#[derive(Debug, PartialEq, Eq)]
struct Complex<T>
{
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T>
{
      fn add_assign(&mut self, rhs: Self) {
          self.re += rhs.re;
          self.im += rhs.im;
      }
}

impl<T> Add for Complex<T> where T: Add<Output = T>
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

// partial eq
// full eq: x = x
// NAN = not a number 0/0 inf/inf
impl<T> PartialEq for Complex<T>  where T: PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

fn main() {
   let mut a = Complex::new(1.,2.); 
   let mut b = Complex::new(3.,4.); 
   //println!("{:?}", a+b);
   //a += b;
   //-a
   // a==b
   println!("a = {:?}", a==a);
}
