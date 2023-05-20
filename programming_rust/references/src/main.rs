use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

//fn show(table: Table) {} // takes ownership of table
// references allow us to pass table as borrow, hence show doesn't take ownership
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    //println!("Hello, world!");
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    sort_works(&mut table);
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    ////! Referencing and dereferencing is done explicitly in Rust using the
    ////!  & and * operators respectively
    ﻿let mut y = 32;
    let m = &mut y;     // &mut y is a mutable reference to y
    *m += 32;           // explicitly dereference m to set y's value
    assert!(*m == 64)

    ////! IMPLICIT DEREFERENCE AND BORROWS
    ////! . notation operator deferencing
    ////! . dereferences implicitly its left operand
    struct Movie { name: &'static str, on_netdlix: bool } // duh! Typo
    let jw4 = Movie { name: "John Wick - Chapter 4", on_netdlix: true };
    let jw4_ref = &jw4;
    assert_eq!(jw4_ref.name, "John Wick - Chapter 4");
    // Explicit deferencing:
    assert_eq!((*jw4_ref).name, "John Wick - Chapter 4");

    ////! . notation operator for borrowing
    ////! . implicitly borrows its left operand
    let mut v = vec![1973, 1968];
    v.sort();           // implicitly borrows a mutable reference to v
    (&mut v).sort();    // equivalent, but more verbose

    ////! ASSIGNING REFERENCES
    ﻿let x = 10;
    let y = 20;
    let mut r = &x;
    if b { r = &y; }
    assert!(*r == 10 || *r == 20); // this code compiles

    ////! ============================
    ////! MEMORY REPRESENTATION

    ////!                   x            r          y
    ////!             ______!____________|__________!_______
    ////! stack frame |   10   |    |    |   |     20      |
    ////!             |____!___|____|____|___|______!______|
    ////!                   \            !          /
    ////!                    \__________/\_________/

    ////! initially r points to x(10) but if b is true, r points to y(20)
    ////! C++ behaves very differently, once the reference has been initialized,
    ////! there is no magic to let it points to another locations
    ////! because C++ references stores the value of its referent

    ﻿////! References to References
    ////! Rust permits references to references:
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr; // Rust can of course infer the types here

    ﻿////! Comparing References
    ////! Like the . operator, Rust’s comparison operators “see through” any number of references:
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);

    ////! ﻿assert!(!std::ptr::eq(rx, ry));

}
