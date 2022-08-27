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

    let class: Class = Class::rnd();
    println!(
        "Testing hypothesis class x_limits: {:?}, y_limits: {:?}: \n",
        class.x_limits.clone(),
        class.y_limits.clone()
    );
    for d in data {
        if d.r {
            println!(
                "for data point x: {}, y: {}, r: {}",
                d.x[0].clone(),
                d.x[1].clone(),
                d.r.clone(),
            );

            let h = hypothesis(
                d,
                class.x_limits.clone(),
                class.y_limits.clone()
            );
        
            println!("result: {} \n", h);
        }
        
    }
}

fn hypothesis(
    x: Data,
    hypothesis_x_limits: [f32; 2],
    hypothesis_y_limits: [f32; 2],
) -> bool {
    let inside_x_limits = hypothesis_x_limits[0] < x.x[0] && x.x[0] < hypothesis_x_limits[1];
    let inside_y_limits = hypothesis_y_limits[0] < x.x[1] && x.x[1] < hypothesis_y_limits[1];
    x.r && inside_x_limits && inside_y_limits
}
