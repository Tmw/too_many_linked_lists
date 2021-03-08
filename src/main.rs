mod first;
mod second;
mod third;
mod fourth;
mod fifth;
mod silly;

fn main() {
    let mut list = first::List::new();
    list.push(12);
    list.pop().map( |val| println!("popped {}", val) );
}

