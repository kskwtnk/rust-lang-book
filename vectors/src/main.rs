fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {} (indexing syntax)", third);

    match v.get(2) {
        Some(third) => println!("The third element is {} (get method)", third),
        None => println!("There is no third element."),
    }
}
