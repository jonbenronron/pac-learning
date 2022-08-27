use rand::prelude::*;

#[derive(Debug)]
struct Data {
    x: Vec<f32>,
    r: bool,
}

type TData = Vec<Data>;

fn main() {
    // Generate random data
    let mut rng = rand::thread_rng();

    let buf: &[u8] = &[0; 10];
    let data: TData = buf.iter().map(|d| Data {
        x: Vec::from([rng.gen::<f32>(), rng.gen::<f32>()]),
        r: rng.gen::<bool>()
    }).collect();

    println!("{:#?}", data);
}
