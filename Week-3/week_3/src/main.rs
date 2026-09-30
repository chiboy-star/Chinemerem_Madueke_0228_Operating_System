fn shout(msg: &str){
	println!("{}!!",msg.to_uppercase());
}

fn run_twice(f: fn(&str), msg: &str){
	f(msg);
	f(msg);
}

fn main() {
	let action = shout;
	run_twice(action,"hello");
}
