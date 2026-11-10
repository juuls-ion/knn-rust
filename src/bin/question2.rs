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

// find euclidean dist between two given points
fn find_distance(p1: &Centre, p2: &Centre) -> f64 {
    /*
    calculates square between respective x1, x2 values of p1 and p2
    doesnt do square root as we only care about order of distances and sqrt is increasing */
    (p1.coordinates[0] - p2.coordinates[0]).powi(2) + 
        (p1.coordinates[1] - p2.coordinates[1]).powi(2) 
}

// 
fn noisy_knn(neighbours: &Vec<Centre>, mut data: Vec<Centre>) -> Vec<Centre> {
    // want to classify each point in 'data' according to knn in 'neighbours'
    // start with distances from point in data to all neighbours
    
    // finding nn for all datapoints in data
    for datapoint in 0..data.len() {
        // vector of distances from one data point to all neighbours
        // each tuple contains float for distance, int for datapoint index
        let mut distances: Vec<(f64, usize)> = Vec::new();

        // populate vector of distances with distance between current point in 'data'
        // and all its neighbours
        for neighbour in 0..neighbours.len() {
            let v = (find_distance(&neighbours[neighbour], &data[datapoint]), neighbour);
            distances.push(v);
        }

        // upack (distance, index) tuple and sort by distances
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));
        
        let noise_gate = rand::random_range(0.0..=1.0);
        if noise_gate <= 0.8 {
            // extract index from tuple of (distance, index) for each point
            let indexes = [distances[0].1, distances[1].1, distances[2].1];
            /*
            since we consider 3 neighbours, if the sum of labels is 2 or 3 then resulting label is 1
            as this implies that 2/3 or 3/3 labels = 1; otherwise resulting label is 0
                */
            let sum = neighbours[indexes[0]].label + neighbours[indexes[1]].label + neighbours[indexes[2]].label;
            // each datapoint's label is correctly updated with 80% probability
            data[datapoint].label = if sum >= 2 {1} else {0}
        } else {
            // datapoint's label randomly assigned with 20% probability
            data[datapoint].label = rand::random_range(0..=1);
        } 
    }
    data
}

fn main() {
    let centres: Vec<Centre> = create_data(NUMBER_OF_CENTRES);
    // test, train data initialised with random labels
    let train_data: Vec<Centre> = create_data(NUMBER_OF_TRAINING_POINTS);
    let test_data: Vec<Centre> = create_data(NUMBER_OF_TEST_POINTS);

    // assign labels to training, test data according to protocol A
    let train_data = noisy_knn(&centres, train_data);
    let test_data = noisy_knn(&centres, test_data);

    
}

