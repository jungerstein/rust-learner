struct cmplx
{
  re : f64, 
  im : f64
}

enum CmplxNum
{
  Cmplx(cmplx), 
  Reele(f64)
}

impl CmplxNum
{
  fn druck(&self)
  {
    match self
    {
      CmplxNum::Reele(x) => println!("Reel {}", x), 
      CmplxNum::Cmplx(z) => println!("Imagine {} + {} i", z.re, z.im) 
    }
  }
}

fn main()
{
  let x = CmplxNum::Reele(5.0);  // Real number. 
  let y = CmplxNum::Cmplx(cmplx{re: 3.0, im : 4.0}); 
  x.druck(); 
  y.druck(); 
}
