fn main ()
{
  let shuzu = [1, 2, 3, 4, 5]; 
  let mut summe = 0; 
  for x in shuzu.iter() // Do not forget iter() method
  {
    summe += x; 
    println!("Accum. sum = {}", summe); 
  }
}
