fn main() {
    let v: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    for i in &v {
        print!("{}\t", i);
    }

    println!("The vector is: {:?}", v);
    println!("The first element is: {}", v[0]);
    println!("The last element is: {}", v[v.len() - 1]);

    println!("\nHello, world!");
}
