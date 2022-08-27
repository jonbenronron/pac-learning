use rand::prelude::*;

#[derive(Debug)]
struct Data {
    x: [f32; 2],
    r: bool,
}

impl Data {
    fn rnd (
        class_x_limits: [f32; 2],
        class_y_limits: [f32; 2]
    ) -> Self {
        let mut rng = rand::thread_rng();
        
        let x = rng.gen::<f32>() * 100.;
        let y = rng.gen::<f32>() * 100.;

        Data {
            x: [x, y],
            r: is_between(x, class_x_limits) && is_between(y, class_y_limits),
        }
    }
}

fn is_between (d: f32, interval: [f32; 2]) -> bool {
    interval[0] < d && d < interval[1]
}

#[derive(Debug, Clone)]
struct Class {
    x_limits: [f32; 2],
    y_limits: [f32; 2],
}

impl Class {
    fn new (
        x_limits: [f32; 2],
        y_limits: [f32; 2],
    ) -> Self {
        Class {
            x_limits: x_limits,
            y_limits: y_limits,
        }
    }

    fn rnd () -> Self {
        let mut rng = rand::thread_rng();
        
        let mut x: [f32; 2] = [rng.gen::<f32>() * 100., rng.gen::<f32>() * 100.];
        let mut y: [f32; 2] = [rng.gen::<f32>() * 100., rng.gen::<f32>() * 100.];

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
    let goal_x_limits: [f32; 2] = [20., 40.];
    let goal_y_limits: [f32; 2] = [20., 40.];
    let goal_class: Class = Class::new(goal_x_limits, goal_y_limits);

    let training_data_proportion: f32 = 0.7;

    // Generate 10 random data points
    let mut data: TData = [0; 100].iter().map(|_| Data::rnd(goal_class.x_limits, goal_class.y_limits)).collect();
    let training_data_size: usize = f32::floor((data.len() as i32) as f32 * training_data_proportion) as usize;
    let training_data: TData = data.split_off(training_data_size);

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
