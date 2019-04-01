fn main ()
{
  let shuzu = [1, 2, 3, 4, 5]; 
  let mut summe = 0; 
  for x in shuzu.iter() // Do not forget iter() method
  {
    summe += x; 
    println!("Accum. sum = {}", summe); 
  }

  summe = 0; 
  println!("Now, reverse sum. "); 
  for x in (1..10).rev() // Not hs-way 1,2,3,..10, but 1 to 9!!! (Python way)
  {
    summe += x; 
    println!("Accum. sum = {}", summe); 
  }
}
