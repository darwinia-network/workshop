struct Bowie(pub u32);
struct Ziggy(pub u32);

// solution-1: enum
// enum Currency {
//     Bowie(Bowie),
//     Ziggy(Ziggy),
// }

// solution-2: trait
trait Hello {
    fn say() -> String {
        "hello, spaceboy".into()
    }
}

impl Hello for Ziggy {
    fn say() -> String {
        "ashes to ashes".into()
    }
}

impl Hello for Bowie {}

// print value from different currencies
fn func<T>(_c: T)
where
    T: Hello,
{
    println!("{:#?}", <T as Hello>::say());
}

fn main() {
    func(Bowie(0));
    func(Ziggy(0));
}
