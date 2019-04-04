use std::io; 

fn main()
{
  loop 
  {
    println!("?"); 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).expect("Input a valid string. "); 
    let mut x : u32 = match s.trim().parse()
    {
      Ok(n) => n, 
      Err(_) => continue 
    }; 
    let mut y = x as f32; 
    if y < 0.0 {continue; } 
    println!("Sqrt {} = {}. ", x, y.sqrt()); // Sorry, but y.sqrt() and y.sin()????
    break;
  }
}
