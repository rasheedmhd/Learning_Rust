#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    // v1_iter is mutable because calling .next() on an iter consumes the iterator
    // changing the internal state thatthe iterator uses to keep track of where it is in the sequence
    // .next returns immutable references to the values
    // .into_iter -> takes ownership and returns owned values
    // .into_mut -> returns mutable references
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // let mut v1_iter = v1.into_iter();

    // assert_eq!(v1_iter.next(), Some(1));
    // assert_eq!(v1_iter.next(), Some(2));
    // assert_eq!(v1_iter.next(), Some(3));
    // assert_eq!(v1_iter.next(), None);
    //
    //
    //
    // Methods that consume the Iterator
    // that Iterator trait has different methods with default implementations
    // provided by std lib. Some of these methods call the next method in their definition
    // requiring implementors of the Iterator trait to implement their own next method
    // These methods are called consuming adaptors -> calling them consumes the interator
    //
    #[test]
    fn interator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}

// Using Closures that Capture Their Environment
// .filter and closures
// .filter takes a closure
// 2. gives an item to the closure
// 3. the closure body does something with it and returns a bool
// 4. if false, the item is not added to the iteration produced by the filter
// 5. if true, the item is added to the iteration

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandals"),
            },
            Shoe {
                size: 12,
                style: String::from("Fulani"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        )
    }
}
