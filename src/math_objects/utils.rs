const EPSILON: f64 = 0.00001;

pub fn equal(a: f64, b: f64) -> bool {
    // Fix float comparision precision
    if (a - b).abs() < EPSILON {
        return true;
    } else {
        return false;
    }
}