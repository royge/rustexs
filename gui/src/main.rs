use gui::{Button, Draw, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 0,
                height: 0,
                label: String::from("Test"),
            }),
            Box::new(SelectBox {
                width: 0,
                height: 0,
                options: vec![String::from("Hello")],
            }),
        ],
    };
    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "w: {:?}, h: {:?}, opt: {:?}",
            self.width, self.height, self.options,
        )
    }
}
