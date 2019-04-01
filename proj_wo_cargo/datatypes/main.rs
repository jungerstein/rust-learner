fn main()
{
  let gougu = (3, 4, 5); // Packing. 
  let (gou, gu, xian) = gougu; // Unpacking. 
  println!("Gou {}, gu {}, xian {}", gou, gu, xian); 
  // Also referred as 
  println!("{}^2 + {}^2 == {}^2", gougu.0, gougu.1, gougu.2); 

  let shuzu :[i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
  println!("Cong 0 kaishi shu, biru shuzu[7] = {}", shuzu[7]); 
}
