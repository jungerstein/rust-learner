struct Fushu
{
  shibu: f64, 
  xubu: f64
}

impl Fushu
{
  fn abs(&self) -> f64
  {
    let abs2 = self.shibu * self.shibu + self.xubu * self.xubu; 
    abs2.sqrt() 
  }

  fn plus(&self, another : &Fushu) -> Fushu
  {
    Fushu
    {
      shibu: self.shibu + another.shibu, 
      xubu: self.xubu + another.xubu
    }
  }
}

#[derive(Debug)] // This thing is not a header. 
// It is a decorator of struct. So put it here. 
struct Fushu2(f64, f64); 

fn omega3() -> Fushu
{
  let three = 3.0 as f64;  
  Fushu{shibu : -0.5, xubu : three.sqrt() / 2.0, }
}

fn print_fushu(z : & Fushu )
{
  println!("{} + {} i", z.shibu, z.xubu); 
}

fn main()
{
  let z = Fushu{shibu : 0.0, xubu : 1.0}; // Note colon for init. 
  let w = omega3(); 
  let ww = Fushu{xubu : -w.xubu, .. w}; // Copy w's xubu. 
  let phi = Fushu2(3.0, 4.0); 
  print_fushu(&z); 
  print_fushu(&w); 
  println!("|w| = {}", w.abs()); 
  let w_p_ww = w.plus(&ww); 
  print_fushu(&w_p_ww); 
  print_fushu(&ww); 
  println!("Fushu2 phi is {:?}", phi); 
}
