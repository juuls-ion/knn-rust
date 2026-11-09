const NUMBER_OF_CENTRES: usize = 100;
const NUMBER_OF_TRAINING_POINTS: usize = 4000;
const NUMBER_OF_TEST_POINTS: usize = 1000;

// create datatype for (y_i, x_i)
struct Centre {
    label: u8,
    coordinates: [f64; 2]
}

// create one (y_i, x_i)
fn create_centre() -> Centre {
    Centre {
        // randomly assigns integer over [0,1]
        label: rand::random_range(0..=1),
        coordinates: [
            // randomly assigns real over [0,1]
            rand::random_range(0.0..=1.0), 
            rand::random_range(0.0..=1.0)],
    }
}

// create vector S of {(y_1, x_1), (y_2, x_2)...}
fn create_data(number_of_points: usize) -> Vec<Centre> {
    let mut vector: Vec<Centre> = Vec::new();
    for _ in 0..number_of_points {
        vector.push(create_centre());
    }
    vector
}

// find euclidean dist between a coordinate in the grid and a given point (x_1, x_2)
fn find_distance(coordinate: &[f64; 2], data_point: &Centre) -> f64 {
    /*
    calculates square between respective x1, x2 values of coordinate and data point
    doesnt do square root as we only care about order of distances and sqrt is increasing */
    (data_point.coordinates[0] - coordinate[0]).powi(2) + 
        (data_point.coordinates[1] - coordinate[1]).powi(2) 
}

fn noisy_knn(centres: Vec<Centre>, data: Vec<Centre>) -> Vec<Centre> {


    /* 
    // array of distances from a coordinate to all data points
    let mut distances: [(f64, usize); NUMBER_OF_POINTS] =
        // each tuple contains float for distance, int for datapoint index
        [(0.0, 0); NUMBER_OF_POINTS];
    
    // populate distances array with distance to each datapoint; datapoint index
    for point in 0..NUMBER_OF_POINTS {
        distances[point].0 = find_distance(&coordinate, &data[point]);
        distances[point].1 = point;
    }

    // upack (distance, index) tuple and sort by distances
    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    // extract index from tuple of (distance, index) for each point
    let indexes = [distances[0].1, distances[1].1, distances[2].1];
    /*
    since we consider 3 neighbours, if the sum of labels is 2 or 3 then resulting label is 1
    as this implies that 2/3 or 3/3 labels = 1; otherwise resulting label is 0
        */
    let sum = data[indexes[0]].label + data[indexes[1]].label + data[indexes[2]].label;
    // each grid coordinate is stored as the correct label
    grid[row][column] = if sum >= 2 {1} else {0}
*/
}

fn main() {
    let centres: Vec<Centre> = create_data(NUMBER_OF_CENTRES);
    // test, train data initialised with random labels
    let train_data: Vec<Centre> = create_data(NUMBER_OF_TRAINING_POINTS);
    let test_data: Vec<Centre> = create_data(NUMBER_OF_TEST_POINTS);
}

