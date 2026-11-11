use std::error::Error;

const NUMBER_OF_POINTS: usize = 100;
const GRID_SIZE: usize = 500;

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
    let array: [Centre; NUMBER_OF_POINTS] = 
        // calls create_point() for each array index
        std::array::from_fn(|_| create_point());
    array
}

// find euclidean dist between a coordinate in the grid and a given point (x_1, x_2)
fn find_distance(coordinate: &[f64; 2], data_point: &Centre) -> f64 {
    /*
    calculates square between respective x1, x2 values of coordinate and data point
    doesnt do square root as we only care about order of distances and sqrt is increasing */
    (data_point.coordinates[0] - coordinate[0]).powi(2) + 
        (data_point.coordinates[1] - coordinate[1]).powi(2) 
}

fn points_to_csv(data: &[Centre]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("output/part1/points.csv")?;

    // csv column headings
    writer.write_record(["label", "x1", "x2"])?;

    // write each point
    for point in data {
        writer.write_record([
            point.label.to_string(),
            point.coordinates[0].to_string(),
            point.coordinates[1].to_string(),
        ])?;
    }

    // data still buffered in memory is written to the CSV
    writer.flush()?;
    Ok(())
}

fn grid_to_csv(grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("output/part1/grid.csv")?;

    for row in grid {
        writer.serialize(row.as_slice())?;
    }

    // data still buffered in memory is written to the CSV
    writer.flush()?;
    Ok(())
}


fn main() {
    // initialise grid with GRID_SIZE 0s per dimensions
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] =
        [[0; GRID_SIZE]; GRID_SIZE];

    let data = create_data();

    // iterate over each coordinate in grid
    for row in 0..GRID_SIZE {
        for column in 0..GRID_SIZE {
            let coordinate: [f64; 2] = [
                /* cast row/column indices as floats
                since grid indices are 0 -> GRID_SIZE -1
                divide by GRID_SIZE -1 to convert from index to coordinate
                */
                row as f64 / (GRID_SIZE - 1) as f64,
                column as f64 / (GRID_SIZE - 1) as f64,
            ];

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
            grid[row][column] = if sum >= 2 {1} else {0};

        }
    }

    points_to_csv(&data).unwrap();
    grid_to_csv(&grid).unwrap();


}

