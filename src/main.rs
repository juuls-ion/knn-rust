const NUMBER_OF_POINTS: usize = 100;
const NUMBER_OF_NEIGHBOURS: usize = 3;

// create datatype for (y_i, x_i)
struct Centre {
    label: u8,
    coordinates: [f64; 2]
}

// create one (y_i, x_i)
fn create_point() -> Centre {
    Centre {
        // randomly assigns integer over [0,1]
        label: rand::random_range(0..=1),
        coordinates: [
            // randomly assigns real over [0,1]
            rand::random_range(0.0..=1.0), 
            rand::random_range(0.0..=1.0)],
    }
}

// create array S of {(y_1, x_1), (y_2, x_2)...}
fn create_data() -> [Centre; NUMBER_OF_POINTS] {
    let array: [Centre; 100] = 
        // calls create_point() for each array index
        std::array::from_fn(|_| create_point());
    array
}

// classify each point 


fn main() {
    println!("Hello, world!");
}

