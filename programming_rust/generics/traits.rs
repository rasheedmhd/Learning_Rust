use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(b"Hello World.\n")?;
        out.flush()
    }

    let mut file = File::create("hello.txt")?;
    say_hello(&mut file)?;

    let mut vec = vec![];
    say_hello(&mut vec)?;

    println!("{:#?}", vec);

    // Rust doesn't permit variables of type dyn Trait(Write in this case)
    let mut buf = vec![];
    // let writer: dyn Write = buf;
    let writer: &mut dyn Write = &mut buf;

    Ok(())
}
