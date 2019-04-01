fn main()
{
  let five = add_one(4); 

  println!("I tried to modify 4. It is {} now. ", five); 
}

fn add_one(x: i32) -> i32 // Types are mandatory; unlike Python hints. 
{
  x + 1 // Nota bene! Sans ';' qui! 
}
