fn main()
{
  let gougu = (3, 4, 5); // Packing. 
  let (gou, gu, xian) = gougu; // Unpacking. 
  println!("Gou {}, gu {}, xian {}", gou, gu, xian); 
  // Also referred as 
  println!("{}^2 + {}^2 == {}^2", gougu.0, gougu.1, gougu.2); 
}
