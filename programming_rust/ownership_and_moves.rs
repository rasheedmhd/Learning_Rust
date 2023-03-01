fn print_padovan() {
    let mut padovan = vec![1,1,1]; // a mutable vector allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here

fn main() {
    print_padovan();
}