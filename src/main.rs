fn main() {
  	let x = 5; // immutable, this value cannot be changed;
  	let mut y: i32 = 12; // mutable, this value CAN be changed;
  	let z:i32; //We initialize a variable;

  	println!("value of x:  {}", x); // ! <- macro, {} <- interpolate into string.

  	{
  		let x = 7;
  		println!("{}",x); //we use the block to shadow the value of x;

  	}

  	println!("{:?}",x ); // x is back to 5;

  	{

  		let mut x = 7;
  		println!("{:?}",x+10 ); //we can even shadow as mutable;

  	}

  	println!("{:?}",x+10 ); //The mutability does permiate BEWARE;

  	{
	
		let x = "hello rust"; //We change type; 		
  		println!("{:?}",x );
  	}

  	println!("{:?}",x ); //What happens now? x is back to a i32;
  	println!("{:?}",x+10 ); //Still mutable? YES.

  	first(x); //we call our function.


}


  	//lets go functioning. Functions start with fn first(arg: Type) -> RetType { stuff }

  	fn first(x: i32) {
  		println!("x is back as argument {}", x);
  	}


  	fn math_with_args(x: i32, y: i32, z: i32) -> i32 {
  		//x + y; this will error on compile since Rust is an expression based language, that is ";" matter. 
  	}