
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {

    fn new() -> Self {
        AveragedCollection {
            list: Vec::new(),
            average: 1.1_f64,
        }
    }

    fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    } 

    fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None
        }
    }

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Let's do some Average Collection !");

    // let test = AveragedCollection {
    //     list: vec![3,7,11,34],
    //     average: 1.1,
    // };

    let mut test = AveragedCollection::new();
    test.add(12);
    test.add(11);
    test.add(25);

    println!("Average Collection {:#?}", test);
}
