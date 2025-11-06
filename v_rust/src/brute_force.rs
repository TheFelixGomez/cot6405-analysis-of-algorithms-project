pub fn brute_force_closest_points(points: &[(f64, f64)]) -> Option<(usize, usize)> {
    let n = points.len();

    if n < 2 {
        return None;
    }

    let mut d_min = f64::INFINITY;
    let mut index_1 = 0;
    let mut index_2 = 0;

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            // d = math.sqrt((points[i][0] - points[j][0])**2 + (points[i][1] - points[j][1])**2)
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            let d = (dx.powi(2) + dy.powi(2)).sqrt();

            // A more robust alternative for distance calculation:
            // let d = dx.hypot(dy); (but kept this for Python comparison)

            if d < d_min {
                d_min = d;
                index_1 = i;
                index_2 = j;
            }
        }
    }

    // return index_1, index_2
    Some((index_1, index_2))
}
