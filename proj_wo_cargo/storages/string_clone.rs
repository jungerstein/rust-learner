fn main()
{
  let s1 = String::from("Dolly"); 
  // Aware: copy -> clone; = (assign) -> move
  let s2 = s1.clone(); 

  println!("Hello, {}! Hello, {}!", s1, s2); 
}
