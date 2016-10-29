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
  	println!("{:?}",math_with_args(x,2,3)); // proof of concept;
  	//diverges(x); PANIC

  	let f: fn(i32,i32) -> i32 = math_add; //function pointer
  	println!("We use add using our funciton pointer {}", f(x,4));
  	let mut g: fn(i32,i32) -> i32; // can we have a mutable function pointer?
  	g = math_add;
  	println!("g as math_add {:?}",g(x,6));
  	g = math_subtract;
  	println!("q as math_subtract {:?}",g(x,3));

  	primitive_types();
  	ifstatements();
  	loops();
  	vectors();
}


//lets go functioning. Functions start with fn first(arg: Type) -> RetType { stuff }

fn first(x: i32) {
	println!("x is back as argument {}", x);
}


fn math_with_args(x: i32, y: i32, z: i32) -> i32 {
	/*
	x + y; this will error on compile since Rust is an expression based language, that is ";" matter. 
	Some points about expressions vs statements: "let" beings a statement and cannot be used as an expression.
	so this would not work let x = (let y = 5); The value of an assignment is an empty tuple (); That is if
	we were to try to return x + y; we'd get an error because the value of x+y; is simply ();
	We can also return using the good old return x; statement;
	*/

	x
}	

fn diverges(x: i32) -> !{
	panic!("PANIC, this function breaks down and does not return");
}

fn math_subtract(x: i32, y: i32) -> i32 {
	x-y
}


fn math_add(x: i32, y: i32) -> i32 {
	x+y
}

fn primitive_types(){
	let x: bool = true;
	println!("boolean {:?}",x ); //nothing fancy
	let x: char = 'x';
	println!("char {:?}",x ); //nothing fancy
	let x: f32 = 1.00; 
	println!("floating point 32 bit {:?}", x); //nothing fancy
	let x: u32 = 1; 
	println!("unsigned integer {:?}", x); // nothing fancy

	//time for arrays:

	let x:[i32;20] = [0;20]; //array of i32s and size 20. Fill it up with 0s.
	println!("length of our array {:?}",x.len() );
	println!("value at index 5 {:?}",x[5] ); //0 indexed, so this will print number 0.

	let slicedx = &x[5..20]; // we have taken a piece of x. 
	println!("we have now sliced a piece of x {:?}",slicedx[5] );
}


fn ifstatements(){
	let x = 1;

	if x==1 {
		println!("We are in an if-statement yay");
	}

	let y = if x==1 {
		10
	}
	else {
		5
	};
	println!("here is y after magic {}", y);
}

fn loops(){

	let mut i = 0;
	loop{
		println!("Will loop forever. However will break now");
		i += 1;
		if i>2 {
			break;
		}
	}

	while i<4 {
		println!("We are in the while loop");
		i += 1;
	}	

	for x in 0..4 {
		println!("We loop 4 times, currently on iteration {}",x );
	}

	for (ind,val) in (5..8).enumerate() {
		println!("We enumerate over elements, iteration is {}, value is {}",ind, val);
	}

	'outer:for x in 0..4 {
		'inner:for y in 0..5 {
			if x+y>3 {
				println!("We continue to the outer loop");
				continue 'outer;
			}
			if y % 2 == 0 {
				println!("We continue to the inner loop");
				continue 'inner;
			}
			println!("Yay we managed to get to an odd y for which x+y is smaller than 4, the y is {}",y);
		}
	}

}


fn vectors(){
	let v = vec![2,3,4,5];
	println!("Third element is {}",v[2]);
	//vectors must be indexed with the usize type, that is i32s don't work, but rather usize;
	//We can iterate over vectors:
	for i in &v {
		println!("We are iterating, currently on element {:?} ",i );
	}

	/* the above iteration uses a reference to the vector. We can "take ownership" of it by 
	writing for i in v {} but then we couldn't iterate over v a second time, since we have already taken 
	ownership (will be discussed next).
	*/
}
