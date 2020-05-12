fn main() {
    let _: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    println!("The third element is {}", &v[2]);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }
}
