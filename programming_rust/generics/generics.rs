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

    Ok(())
}
