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
  	let mut v1 = vec![1,1,1];
  	let mut v2 = vec![1,1,1];
  	println!("v1[2] = {} , v2[2] = {}",v1[2],v2[2]);
  	testBorrow(&mut v1, &mut v2);
  	println!("v1[2] = {} , v2[2] = {}",v1[2],v2[2]);

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


/*One of the most useful features of Rust is the concept of ownership. This is how Rust achieves memory safety.
The basic gist is as follows. If you have an object of a type that does not support the copy trait then the 
following line of code would leave v unusable:

let v = vec![1,2,3,4,5];
let v2 = v;

here v2 would have ownership over our funny little vector and v would have been left dead. This is to stop dataraces
from happening. This is crucial behaviour in Rust. Types such as bool and i32 have the copy trait which allows
the above assignment to work, but what happens over the hood is that v2 receives ownership of a COPY of v while
v retain the ownership of the original copy. Naturally we can move ownership back and forth like:

let v = vec![1,2,3,4,5];
let v2 = v;
let v = v2;

However this gets cumbersome and hence has another very important feature called borrowing.

Borrowing is handled using references (much like pointers). Consider the following code:

fn doStuff(v1: Vec<i32>, v2: Vec<i32>){
	do stuff here.
}

If the above function is given two vectors it gains ownership over them as we've seen earlier. However, we can
pass references to the vectors as follows without taking away ownership:

fn doStuff(v1: &Vec<i32>, v2: &Vec<i32>){
	do stuff here.
}

This is called borrowing. However these references are unmutable meaning that we can't actually change the two
vectors in any way, just look at them. Naturally there are exceptions to this, namely the &mut refrence. Consider
the following code:

fn doStuff(v1: &mut vec<i32>, v2: &mut vec<i32>){
	do stuff here.
}

Here we can actually change the values in the two vectors. The bellow code does just this:

*/


fn testBorrow(v1: &mut Vec<i32>, v2: &mut Vec<i32>){
	v1[2] = 2;
	v2[2] = 2;
}


/*

There are rules to follow when making use of references and mutable references: you can either have 

1) one more more references (using the normal "&" marking; ex: &Vec<i32>) to a resource.
OR
2) you can have exactly one mutable reference (using "&mut" marking; ex: &mut Vec<i32>) to a resource.

Essentially this means that you can have an arbitrary number of references that are read only but ever only one
reference if that reference can both read and write. 

Another important rule is that the scope of the borrow must not be greater than the scope of the owner.
Borrowing is tied to the scope and as such when we borrow the borrow last until the end of the scope it
is borrowed in.

*/