//Using Trait Objects That Allow for Values of Different Types
use oop::Button;

use oop::{Screen, SelectBox, TextBox};

fn main() {
    let page = Screen {
        components: vec![
            Box::new(TextBox {
                max_length: 10000,
                width: 200,
                height: 17,
                label: String::from("About Startup"),
            }),
            Box::new(SelectBox {
                label: String::from("Industries"),
                options: vec![
                    String::from("Aerospace"),
                    String::from("Surveillance Drones"),
                    String::from("Systems"),
                ],
            }),
            Box::new(TextBox {
                max_length: 10000,
                width: 200,
                height: 17,
                label: String::from("Mission"),
            }),
            Box::new(Button {
                height: 12,
                width: 24,
                label: String::from("Create"),
            }),
        ],
    };

    page.run();
}
