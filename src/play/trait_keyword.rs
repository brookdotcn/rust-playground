/* https://doc.rust-lang.org/std/keyword.trait.html */

trait Cool {
    const COOL: Self;
    fn is_cool(&self) -> bool;
}

impl Cool for i32 {
    const COOL: Self = 100;
    fn is_cool(&self) -> bool {
        println!("is {} cool -> {}", self, *self == Self::COOL);
        *self == Self::COOL
    }
}

trait Reset: Default {
    fn reset(&mut self) -> ();
}

impl<T: Default> Reset for T {
    fn reset(&mut self) -> () {
        println!("T::reset() was called");
        *self = Self::default()
    }
}

#[derive(Default)]
struct Person {
    _age: u8,
}

pub fn trait_keyword_main() -> () {
    50.is_cool();
    100.is_cool();

    let mut me = Person { _age: 10 };
    me.reset();
}
