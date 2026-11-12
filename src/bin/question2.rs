use std::error::Error;

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

/*
noisy data generation:
 - classify given 'data' against 'neighbours' -> original 100 centres
 - with k = 3, then apply 80/20 noise rule
*/
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
            data[datapoint].label = if sum >= 2 {1} else {0};
        } else {
            // datapoint's label randomly assigned with 20% probability
            data[datapoint].label = rand::random_range(0..=1);
        } 
    }
    data
}

// returns a vector of prediceted labels for each test point
fn eval_knn(test_data: &Vec<Centre>, train_data: &Vec<Centre>, k: usize) -> Vec<usize> {
    let mut output_labels: Vec<usize> = Vec::new();
    
    // finding nn for all datapoints in test data
    for test_point in 0..test_data.len() {
        // vector of distances from one data point to all neighbours
        // each tuple contains float for distance, int for train_point index
        let mut distances: Vec<(f64, usize)> = Vec::new();

        // populate vector of distances with distance between current test point
        // and all training points
        for neighbour in 0..train_data.len() {
            let v = (find_distance(&test_data[test_point], &train_data[neighbour]), neighbour);
            distances.push(v);
        }

        // upack (distance, index) tuple and sort by distances
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));
        
        // extract index from tuple of (distance, index) for each point
        let mut sum: usize = 0;
        // collect only k closest neighbours
        for neighbour in 0..k {
            // access label of training datapoint
            // distances.neighbour is a tuple with (distance, train_datapoint index)
            sum += train_data[distances[neighbour].1].label as usize;
        }

        // if the majority of nn labels are one then the point's label is 1
        let label = if 2 * sum > k {
                1
            } else if 2 * sum < k {
                0
            } else {rand::random_range(0..=1)}; // if draw, randomly assign label

        output_labels.push(label);
    }
    output_labels
}

fn classification_error_rate(test_data: &Vec<Centre>, predicted_test_labels: &Vec<usize>) -> f64 {
    let mut total_error = 0;
    for test_point in 0..test_data.len() {
        // if labels dont match
        if test_data[test_point].label != predicted_test_labels[test_point] as u8 {total_error +=1}
    };
    total_error as f64 / NUMBER_OF_TEST_POINTS as f64
}

fn data_to_csv(data: &[(usize, f64); 49]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("output/part2/error.csv")?;

    // csv column headings
    writer.write_record(["k", "error"])?;

    // write each point
    for point in data {
        writer.write_record([
            point.0.to_string(),
            point.1.to_string(),
        ])?;
    }

    // data still buffered in memory is written to the CSV
    writer.flush()?;
    Ok(())
}

fn main() {
    // initialise array to hold final outputs
    let mut error_per_k = [(0, 0.0); 49];
    for k in 1..=49 {
        let mut generalisation_error = 0.0;
        for _ in 0..100 {
            // initialise random hypothesis
            let centres: Vec<Centre> = create_data(NUMBER_OF_CENTRES);
            // test, train data initialised with random labels
            let train_data: Vec<Centre> = create_data(NUMBER_OF_TRAINING_POINTS);
            let test_data: Vec<Centre> = create_data(NUMBER_OF_TEST_POINTS);

            // assign labels to training, test data with 80/20 split
            let train_data = noisy_knn(&centres, train_data);
            let test_data = noisy_knn(&centres, test_data);

            let predicted_test_labels = eval_knn(&test_data, &train_data, k);
            // increment error sum with the error rate from this run
            generalisation_error += classification_error_rate(&test_data, &predicted_test_labels);
        }
        // arrays are 0 indexed but k starts at 1
        error_per_k[k-1] = (k, generalisation_error / 100.0);
    }
    data_to_csv(&error_per_k).unwrap();
}

