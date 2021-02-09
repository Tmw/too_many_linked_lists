mod first;
mod second;

fn main() {
    let mut list = first::List::new();
    list.push(12);
    list.pop().map( |val| println!("popped {}", val) );
}

