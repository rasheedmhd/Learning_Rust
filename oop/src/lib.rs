pub trait Draw {
    fn draw(&self);
}

// Using dyn Trait as a standin for any type that implements trait Trait
//#[derive(Debug)]
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

#[derive(Debug)]
pub struct Button {
    pub height: u32,
    pub width: u32,
    pub label: String,
}

#[derive(Debug)]
pub struct TextBox {
    pub max_length: usize,
    pub height: u32,
    pub width: u32,
    pub label: String,
}

#[derive(Debug)]
pub struct SelectBox {
    pub label: String,
    pub options: Vec<String>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            println!("drawing screen components.....");
            component.draw();
            println!("drawing screen components completed successfully");
        }
    }
    pub fn print(&self) {
        for component in self.components.iter() {
            format!("Components: {:#?}", component);
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        //
    }
}

impl Draw for TextBox {
    fn draw(&self) {
        //
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        //
    }
}
