fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    if let Some(x) = iter.next() { println!("First element: {}", x); }
    if let Some(x) = iter.next() { println!("Second element: {}", x); }
    if let Some(x) = iter.next() { println!("Third element: {}", x); }
    if let Some(x) = iter.next() { println!("Fourth element: {}", x); } else { println!("Fourth element: No more elements"); }
} 