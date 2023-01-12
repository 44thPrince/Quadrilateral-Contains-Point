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
    point: Option<Vec<i32>>,
    verticies: Vec<Vec<f32>>,

}
impl State {
    fn check_bounds (&self) -> bool {
        // Derive y = mx + b for points AB, DC and x = my + b for points AD, BC
        let slope_x1 = State::slope(self.verticies[0].clone(), self.verticies[1].clone());
        let slope_x2 = State::slope(self.verticies[3].clone(), self.verticies[2].clone());
        let slope_y1 = State::inverse_slope(self.verticies[0].clone(), self.verticies[3].clone());
        let slope_y2 = State::inverse_slope(self.verticies[1].clone(), self.verticies[2].clone());
        // Get upper and lower y bounds
        let mut y_bounds: Vec<f32> = Vec::new();
        let mut x_bounds: Vec<f32> = Vec::new();
        match &self.point {
            Some(components) => {
                y_bounds.push(slope_y1*(components[1] as f32) + self.verticies[0][1]);
                y_bounds.push(slope_y2*(components[1] as f32) + self.verticies[1][1]);
                x_bounds.push(slope_x1*(components[0] as f32) + self.verticies[0][0]);
                x_bounds.push(slope_x2*(components[0] as f32) + self.verticies[3][0]);
                if x_bounds[0] >= components[0] as f32 || x_bounds[1] <= components[0] as f32 {
                    if y_bounds[0] >= components[1] as f32 || y_bounds[1] <= components[1] as f32 {
                        return true;
                    }
                }
                return false;
            }
            None => return false,
        }
    }
    fn slope (p1: Vec<f32>, p2: Vec<f32>) -> f32 {
        return (p2[0].clone() - p1[0].clone()) / (p2[1].clone() - p1[1].clone()) // to calculate slope for x: (x2 - x1) / (y2 - y1)
    }
    fn inverse_slope (p1: Vec<f32>, p2: Vec<f32>) -> f32 {
         return (p2[1].clone() - p1[1].clone()) / (p2[0].clone() - p1[0].clone()) // to calculate slope for y: (y2 - y1) / (x2 - x1)
    }
}
fn main() {
    let state = State {
        point: Some(vec![2, 1]),
        verticies: vec![vec![1.0,1.0], vec![3.0,1.0], vec![4.0,2.0], vec![1.0, 2.0]]

    };
    println!("{}", state.check_bounds());
}
