use std::io;




fn plus(f: i32, s: i32) {
	println!("{}", f + s);
}


fn minus(f: i32, s: i32) {
	println!("{}", f - s);
}

fn idk(f: i32, s: i32) {
	println!("{}", f / s);
}

fn multiple(f: i32, s: i32) {
	println!("{}", f * s);
}

fn main() {
println!("common calculator pls write operator");
let mut rawinput = String::new();
io::stdin().read_line(&mut rawinput).expect("ehm");
let mut f = String::new();
let mut s = String::new();
println!("write first number");
io::stdin().read_line(&mut f).expect("ehm");
println!("write second number");
io::stdin().read_line(&mut s).expect("ehm");
let f: i32 = f.trim().parse().expect("mhm");
let s: i32 = s.trim().parse().expect("mhm");
let rawinput: char = rawinput.trim().parse().expect("allo");
match rawinput {
	'+' => plus(f, s),
	'-' => minus(f,s),
	'/' => idk(f,s),
	'*' => multiple(f,s),
	_ => println!("idk bro this operator"),
};
}
