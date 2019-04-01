use rand::Rng; 
use std::cmp::Ordering; 
use std::io; 

fn main() 
{
  println!("Cai shuzi. "); 
  println!("Qing shuru shuzi. ");

  let suijishu = rand::thread_rng().gen_range(1, 101); 


  // println!("Nin shuru de shi {}. ", caide); 
  loop 
  {
    let mut caide = String::new(); 
    io::stdin().read_line(&mut caide).expect("Shuru you-wu, qing chong shu. "); 
    let mut caideshu: u32 = match caide.trim().parse()
    {
      Ok(n) => n, 
      Err(_) => continue 
    }; 
    // println!("Yao cai de shu shi {}. ", suijishu); 
    let say_small = "Xiao le! "; 
    let say_big = "Da le"; 
    let mut deciser = rand::thread_rng().gen_range(1, 101); 
    let threshold = 95; 
    let mut word = String::new(); 

    println!("Deciser = {}", deciser); 

    match caideshu.cmp(&suijishu)
    {
      Ordering::Less => 
        {
          match deciser.cmp(&threshold)
          {
            Ordering::Less => word = say_small.to_string(), 
            Ordering::Equal => word = say_small.to_string(), 
            Ordering::Greater => word = say_big.to_string()
          }
        }, 
      Ordering::Greater => 
        {
          match deciser.cmp(&threshold)
          {
            Ordering::Less => word = say_big.to_string(), 
            Ordering::Equal => word = say_big.to_string(), 
            Ordering::Greater => word = say_small.to_string()
          }
        }, 
      Ordering::Equal => 
        {
          println!("Dui le! "); 
          break; 
        }
    }
    println!("{}", word); 
  }
}
