use rand::prelude::*;

#[derive(Debug)]
struct Data {
    x: f32,
    y: f32,
}

type TData = Vec<Data>;

fn main() {
    // Generate random data
    let mut rng = rand::thread_rng();

    let buf: &[u8] = &[0; 10];
    let data: Vec<Data> = buf.iter().map(|d| Data { x: rng.gen::<f32>(), y: rng.gen::<f32>() }).collect();

    println!("{:#?}", data);
}
