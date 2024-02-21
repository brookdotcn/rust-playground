/* https://doc.rust-lang.org/std/keyword.trait.html */

/* structure of extended implementation on data types */
trait Cool {
    /* only constants, types or functions for traits */
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

/* traits can be extended into other traits */
trait Reset: Default {
    fn reset(&mut self) -> ();
}

/* traits can be implemented with generics */
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
