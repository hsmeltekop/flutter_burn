fn main() {
    let bytes = include_bytes!("../../four.raw");
    let b: Vec<_> = bytes
        .into_iter()
        .map(|e| if *e > 0 { 1u8 } else { *e })
        .collect();
    println!("{:?}", b);

    for i in 1..28 {
        let r = &b[(i * 28)..(i * 28 + 28)];
        println!("{:?}", r);
    }
}
