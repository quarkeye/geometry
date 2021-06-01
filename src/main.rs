trait Geometry {
	fn area(&self) -> f32;
	fn perimeter(&self) -> f32;
}

struct Rectangle {
	width: f32,
	height: f32,
}

impl Geometry for Rectangle {
	fn area(&self) -> f32 {
		self.width * self.height
	}
	fn perimeter(&self) -> f32 {
		(self.width + self.height) * 2.0
	}
}

struct Circle {
	radius: f32
}

impl Geometry for Circle {
	fn area(&self) -> f32 {
		3.1415926 * self.radius * self.radius
	}
	fn perimeter(&self) -> f32 {
		2.0 * 3.1415926 * self.radius
	}
}

fn main() {
	let rect = Rectangle{ width: 6.5, height: 2.3};
    println!("rect.area: {}, rect.perimeter: {}",rect.area(),rect.perimeter());
	
	let circle = Circle{ radius: 3.4};
	println!("circle.area: {}, circle.perimeter: {}",circle.area(),circle.perimeter());
}
