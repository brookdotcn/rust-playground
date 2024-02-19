/* https://doc.rust-lang.org/std/keyword.impl.html */

struct Car {
    power: u8,
}

impl Car {
    fn new(power: u8) -> Self {
        Self { power }
    }

    fn drive() -> () {
        println!("Car::drive() was called")
    }

    fn get_power(&self) -> u8 {
        println!("Car::get_power() was called");
        self.power
    }
}

trait CarMod {
    fn remap(&mut self) -> u8;
}

impl CarMod for u8 {
    fn remap(&mut self) -> u8 {
        *self * 2
    }
}

fn upgrade() -> impl Fn(u8) -> () {
    println!("upgrade() was called");
    |n: u8| println!("closure was called -> {}", n * 2)
}

pub fn implement_main() -> () {
    let instance = Car::new(10);
    Car::drive();

    let mut power = Car::get_power(&instance);
    println!("u8::remap() was called -> {}", power.remap());

    let inner = upgrade();
    inner(5);
}
