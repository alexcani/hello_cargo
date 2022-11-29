use hello_cargo::Draw;
use hello_cargo::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Select box draw. {}x{}", self.width, self.height);
    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(SelectBox {
            width: 70,
            height: 50,
            options: vec![String::from("One"), String::from("Two")],
        }),
        Box::new(Button {
            height: 10,
            width: 20,
            label: String::from("Click here"),
        })],
    };

    screen.run();
}
