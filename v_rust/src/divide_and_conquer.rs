use ordered_float::OrderedFloat;
use std::collections::HashSet;

fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    // return math.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()

    // A more idiomatic (and numerically stable) way in Rust (but done like this for comparison with Python):
    // let dx = p1.0 - p2.0;
    // let dy = p1.1 - p2.1;
    // dx.hypot(dy)
}

fn _divide_and_conquer_closest_points_rec(
    Px: &[(f64, f64)],
    Py: &[(f64, f64)],
) -> ((f64, f64), (f64, f64)) {
    let n = Px.len();

    // --- 1. Base Case ---
    // if |P| <= 3 find the closest pair by measuring all pairwise distances
    if n <= 3 {
        // basically the brute-force approach
        let mut min_dist = f64::INFINITY;
        // p1, p2 = None, None
        // use Option to handle the initialization, but unwrap at the
        // end, as this function guarantees a pair if n >= 2.
        let mut p1_opt: Option<(f64, f64)> = None;
        let mut p2_opt: Option<(f64, f64)> = None;

        for i in 0..n {
            for j in (i + 1)..n {
                let d = distance(Px[i], Px[j]);
                if d < min_dist {
                    min_dist = d;
                    p1_opt = Some(Px[i]);
                    p2_opt = Some(Px[j]);
                }
            }
        }
        return (p1_opt.unwrap(), p2_opt.unwrap());
    }

    // --- 2. Divide ---
    // Construct Qx, Qy, Rx, Ry in O(n) time
    let mid = n / 2;
    let (Qx, Rx) = Px.split_at(mid);

    // Qx_set = set(Qx)  // for O(1) lookups
    // wrap the f64 tuples in OrderedFloat to make them hashable.
    let Qx_set: HashSet<_> = Qx
        .iter()
        .map(|p| (OrderedFloat(p.0), OrderedFloat(p.1)))
        .collect();

    let mut Qy: Vec<(f64, f64)> = Vec::with_capacity(mid);
    let mut Ry: Vec<(f64, f64)> = Vec::with_capacity(n - mid);

    for &p in Py {
        let p_hashable = (OrderedFloat(p.0), OrderedFloat(p.1));
        if Qx_set.contains(&p_hashable) {
            Qy.push(p);
        } else {
            Ry.push(p);
        }
    }

    // --- 3. Conquer ---
    let (q0_star, q1_star) = _divide_and_conquer_closest_points_rec(Qx, &Qy);
    let (r0_star, r1_star) = _divide_and_conquer_closest_points_rec(Rx, &Ry);

    // --- 4. Combine ---
    let d_q = distance(q0_star, q1_star);
    let d_r = distance(r0_star, r1_star);
    let delta = d_q.min(d_r);
    let x_star = Qx[Qx.len() - 1].0;

    // S = points in P within distance delta of L
    // Construct Sy in O(n) time
    let Sy: Vec<(f64, f64)> = Py
        .iter()
        .cloned()
        .filter(|p| (p.0 - x_star).abs() < delta)
        .collect();

    let mut min_d_pair = f64::INFINITY;
    // use an Option, as the loop may not run if Sy is small.
    let mut s_s_prime: Option<((f64, f64), (f64, f64))> = None;

    let len_Sy = Sy.len();
    for i in 0..len_Sy {
        let j_end = (i + 16).min(len_Sy);
        for j in (i + 1)..j_end {
            let s = Sy[i];
            let s_prime = Sy[j];

            let d_s_to_s_prime = distance(s, s_prime);

            if d_s_to_s_prime < min_d_pair {
                min_d_pair = d_s_to_s_prime;
                s_s_prime = Some((s, s_prime));
            }
        }
    }

    // let s_s_prime (s, s') be the pair with the minimum distance (min_d_pair)
    if min_d_pair < delta {
        // If min_d_pair < inf, s_s_prime must be Some.
        return s_s_prime.unwrap();
    } else if d_q < d_r {
        (q0_star, q1_star)
    } else {
        (r0_star, r1_star)
    }
}

pub fn divide_and_conquer_closest_points(points: &[(f64, f64)]) -> (usize, usize) {
    // construct Px in O(nlogn) time
    let mut Px = points.to_vec();
    Px.sort_by_key(|p| OrderedFloat(p.0));

    // construct Py in O(nlogn) time
    let mut Py = points.to_vec();
    Py.sort_by_key(|p| OrderedFloat(p.1));

    let (p0_star, p1_star) = _divide_and_conquer_closest_points_rec(&Px, &Py);

    // return the indices instead of the actual point
    // look for p0_star and p1_star in the original points list
    // index_1 = points.index(p0_star)
    // .position() is the O(n) equivalent of Python's .index()
    let index_1 = points.iter().position(|&p| p == p0_star).unwrap();

    // This is a more robust way to find the second index, which correctly
    // handles duplicate points (a bug in the original Python logic, but ignored as it doesn't affect the tests).
    // It finds a point that matches p1_star *at a different index* than index_1.
    let index_2 = points
        .iter()
        .enumerate()
        .find(|&(i, p)| *p == p1_star && i != index_1)
        .map(|(i, _)| i)
        .expect("Could not find a second point; this should not happen.");

    (index_1, index_2)
}
