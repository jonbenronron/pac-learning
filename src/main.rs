use rand::prelude::*;

#[derive(Debug, Clone)]
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
}

type TData = Vec<Data>;
type TDelta = f32;

fn main() {
    let goal_x_limits: [f32; 2] = [20., 40.];
    let goal_y_limits: [f32; 2] = [20., 40.];
    let goal_class: Class = Class::new(goal_x_limits, goal_y_limits);

    const DATA_SIZE: usize = 100;

    let data: TData = [0; DATA_SIZE].iter().map(|_| Data::rnd(goal_class.x_limits, goal_class.y_limits)).collect();
    let mut result = false;
    let mut i = 1;

    while !result && i < DATA_SIZE {
        let mut test_data: TData = data.clone();
        let training_data: TData = test_data.split_off(i);
        
        let hypothesis: Class = find_hypothesis(training_data.clone());
        println!("N: {}", i);

        println!(
            "Testing hypothesis class x_limits: {:?}, y_limits: {:?}: \n",
            hypothesis.x_limits.clone(),
            hypothesis.y_limits.clone()
        );
        result = test_hypothesis(test_data, hypothesis);
        println!("Result: {}", result);
        i += 1;
    }
    
}

fn find_hypothesis(data: TData) -> Class {
    let mut x_min = 0.;
    let mut x_max = 100.;
    let mut y_min = 0.;
    let mut y_max = 100.;

    for (i, d) in data.iter().enumerate() {
        if i == 0 {
            x_min = d.x[0];           
            x_max = d.x[0];
            y_min = d.x[1];
            y_max = d.x[1];
        } else {
            if d.r {
                if d.x[0] < x_min { x_min = d.x[0]; }
                if d.x[0] > x_max { x_max = d.x[0]; }
                if d.x[1] < y_min { y_min = d.x[1]; }
                if d.x[1] > y_max { y_max = d.x[1]; }
            }
        }
        
    }

    Class::new([x_min, x_max], [y_min, y_max])
}

fn test_hypothesis(
    data: TData,
    hypothesis: Class,
) -> bool {
    let delta: TDelta = 0.01;
    let prob: f32;

    let mut error: u32 = 0;
    let n: u32 = data.len().try_into().unwrap();

    for d in data {
        if d.r {
            if d.x[0] < hypothesis.x_limits[0] ||
                d.x[0] > hypothesis.x_limits[1] ||
                d.x[1] < hypothesis.y_limits[0] ||
                d.x[1] > hypothesis.y_limits[1] { error += 1; }
        }
    }
    prob = (error / n) as f32;
    prob >= (1. - delta)
}
