use std::fmt::Debug;
use std::hash::Hash;
use std::io::Write;

fn main() {
    // fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    //     out.write_all(b"Hello World.\n")?;
    //     out.flush()
    // }

    fn say_hello<T: Write>(out: &mut T) -> std::io::Result<()> {
        out.write_all(b"Hello World.\n")?;
        out.flush()
    }
    println!("Hello, World!");

    // Multiple traits in generics
    fn top_ten<T: Debug + Hash + Eq>(vals: &Vec<T>) {}
}
