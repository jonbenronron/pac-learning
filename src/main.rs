use rand::prelude::*;

#[derive(Debug)]
struct Data {
    x: [f32; 2],
    r: bool,
}

impl Data {
    fn rnd () -> Self {
        let mut rng = rand::thread_rng();
        Data {
            x: [rng.gen::<f32>(), rng.gen::<f32>()],
            r: rng.gen::<bool>(),
        }
    }
}

#[derive(Debug, Clone)]
struct Class {
    x_limits: [f32; 2],
    y_limits: [f32; 2],
}

impl Class {
    fn rnd () -> Self {
        let mut rng = rand::thread_rng();
        
        let mut x: [f32; 2] = [rng.gen::<f32>(), rng.gen::<f32>()];
        let mut y: [f32; 2] = [rng.gen::<f32>(), rng.gen::<f32>()];

        x.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Class {
            x_limits: x,
            y_limits: y,
        }
    }
}

type TData = Vec<Data>;

fn main() {
    // Generate 10 random data points
    let data: TData = [0; 10].iter().map(|_| Data::rnd()).collect();

    assert_eq!(data.len(), 10);

    println!("{:#?}", data);

    let class: Class = Class::rnd();
    for d in data {
        let h = hypothesis(d, Class::rnd());
        println!(
            "Testing hypothesis: {:#?}",
            h,
        );
    }
}

fn hypothesis(x: Data, c: Class) -> bool {
    let inside_x_limits = c.x_limits[0] < x.x[0] && x.x[0] < c.x_limits[1];
    let inside_y_limits = c.y_limits[0] < x.x[1] && x.x[1] < c.x_limits[1];
    x.r && inside_x_limits && inside_y_limits
}
