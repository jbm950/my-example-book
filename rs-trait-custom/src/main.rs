trait Summary {
    fn summarize(&self) -> String;
}

struct Item1 {
    name: String,
}

impl Summary for Item1 {
    fn summarize(&self) -> String {
        format!("My name is {}", self.name)
    }
}

struct Item2 {
    x: u8,
    y: u8,
}

impl Summary for Item2 {
    fn summarize(&self) -> String {
        format!("x: {}, y: {}", self.x, self.y)
    }
}

fn display_item<T: Summary>(item: T) {
    println!("Printing an item: {}", item.summarize())
}

fn main() {
    let item1 = Item1{name: "Steve".into()};
    display_item(item1);

    let item2 = Item2{x: 3, y:7};
    display_item(item2);
}
