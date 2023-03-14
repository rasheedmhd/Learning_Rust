fn print_padovan() {
    let mut padovan = vec![1,1,1]; // a mutable vector allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here

fn main() {
    //let x = padovan[0];
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");

        println!("point: {:#?}", point);
        println!("label: {}", label);
    }

    //println!("label: {}", label);
}


//RUST DATA MEMORY REPRESENTATION
//             ______________________________
//             |        |          |        |
//stack frame = | buffer | capacity | length |
//             |________|__________|________|
// -> the buffer + capacity + length are stored in the  stack frame of the print_padovan fn above
//
//buffer -> a pointer to the stack/heap where the data is stored
//capacity -> the amount of data the buffer can take before it needs to be assigned a new buffer
//length -> the number of numbers/characters the data holds


