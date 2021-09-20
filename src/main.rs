fn main() {
    let stack: &str = "hello";

    let mut heap: String = String::from(stack);

    println!("{}", heap);

    heap.push_str(", world!");

    println!("{}", heap);

    heap.push('A');

    println!("{}", heap);

    let len: usize = heap.len();

    println!("{}", len);

    let str = heap.as_str();

    println!("{}", str);

    heap.clear();

    println!("{}", heap);

    let len: usize = heap.len();

    println!("{}", len);
}
