/*
 *  A program that finds if a point exists within a quadrilateral.
 *  Given point P and quadrilateral from points A (bottom left) to D (top left)
 *  D--C
 *  |  |
 *  A--B
 *      Check if point is within y bounds
 *      Step 1: Derive y = mx + b function from slope of lines AB and DC (where b is the y of point A/D)
 *      Step 2: Plug x value of P into both functions to find upper and lower bounds.
 *      Step 3: If y value of P is in bounds go on, otherwise return false.
 *      Check if point is within x bounds.
 *      Step 1: Derive x = my + b function from *inverse* slope of lines AD and BC
 *      Step 2: Plug y value of P into both functions to find upper and lower bounds.
 *      Step 3: If x value of P is in bounds return true.
 */

struct State {
    // points are x, y
    point: Option<Vec<f64>>,
    verticies: Vec<Vec<f64>>,

}
impl State {
    fn check_bounds (&self) -> bool {
        // Derive y = mx + b for points AB, DC and x = my + b for points AD, BC
        for vertex in &self.verticies {
            println!("{}, {}", vertex[0], vertex[1]);
        }
        let slope_x1 = State::slope(self.verticies[0].clone(), self.verticies[1].clone());
        let slope_x2 = State::slope(self.verticies[3].clone(), self.verticies[2].clone());
        let slope_y1 = State::inverse_slope(self.verticies[0].clone(), self.verticies[3].clone());
        let slope_y2 = State::inverse_slope(self.verticies[1].clone(), self.verticies[2].clone());
        // Get upper and lower y bounds
        let mut y_bounds: Vec<f64> = Vec::new();
        let mut x_bounds: Vec<f64> = Vec::new();
        match &self.point {
            Some(components) => {
                println!("{} and {}", components[0], components[1]);
                println!("y lower{:.10}", ((slope_x1*(components[1])) + self.verticies[0][1]));
                y_bounds.push((slope_x1*(components[1])) + self.verticies[0][1]);
                println!("y upper{:.10}", ((slope_x2*(components[1])) + self.verticies[3][1]));
                y_bounds.push((slope_x2*(components[1])) + self.verticies[3][1]);
                println!("x lower{:.10}", ((slope_y1*(components[0])) + self.verticies[0][0]));
                x_bounds.push((slope_y1*(components[0])) + self.verticies[0][0]);
                println!("x upper{:.10}", ((slope_y2*(components[0])) + self.verticies[3][0]));
                x_bounds.push((slope_y2*(components[0])) + self.verticies[3][0]);
                if x_bounds[0] <= components[0] && x_bounds[1] >= components[0] {
                    if y_bounds[0] <= components[1] && y_bounds[1] >= components[1] {
                        return true;
                    }
                }
                return false;
            }
            None => return false,
        }
    }
    fn inverse_slope (p1: Vec<f64>, p2: Vec<f64>) -> f64 {
        println!("inv slope{:.10}", ((p2[0].clone() - p1[0].clone()) / (p2[1].clone() - p1[1].clone())) as f64);
        return (p2[0].clone() - p1[0].clone()) / (p2[1].clone() - p1[1].clone()) // to calculate slope for y: (x2 - x1) / (y2 - y1)
    }
    fn  slope (p1: Vec<f64>, p2: Vec<f64>) -> f64 {
        println!("slope{:.10}", ((p2[1].clone() - p1[1].clone()) / (p2[0].clone() - p1[0].clone())) as f64);
        return (p2[1].clone() - p1[1].clone()) / (p2[0].clone() - p1[0].clone()) // to calculate slope for x: (y2 - y1) / (x2 - x1)
    }
}
fn main() {
    let state = State {
        point: Some(vec![3.0, 1.5]),
        verticies: vec![vec![2.0,1.0], vec![3.0,1.0], vec![4.0,2.0], vec![1.0, 4.0]]

    };
    println!("{}", state.check_bounds());
}
