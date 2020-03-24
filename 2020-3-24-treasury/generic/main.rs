struct Bowie(pub i32);
struct Ziggy(pub i32);

// default trait
trait Hello {
    fn hello() -> String {
        "hello, spaceboy!".into()
    }
}

impl<T> Hello for T {}

// the method
fn propose_spend<T>(_x: T)
where
    T: Hello,
{
    println!("{:#?}", <T as Hello>::hello());
}

fn main() {
    propose_spend(Bowie(0));
}
