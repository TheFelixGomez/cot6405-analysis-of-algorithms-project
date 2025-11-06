use super::*;

#[test]
fn test_brute_force() {
    // Define a list of points (x, y)
    let points = vec![
        (1.0, 2.0),
        (3.0, 3.0),
        (2.0, 8.0), // index 2
        (8.0, 1.0),
        (4.0, 6.0), // index 4
        (7.0, 5.0),
    ];

    // Find the closest pair
    let result = brute_force::brute_force_closest_points(&points);

    // The closest pair is (2.0, 8.0) [index 2] and (4.0, 6.0) [index 4]
    // The brute force function should find index 2 first, then 4.
    assert_eq!(result, Some((0, 1)));

    let points_2 = vec![
        (0.0, 0.0), // index 0
        (10.0, 10.0),
        (0.0, 1.0), // index 2
    ];

    // Find the closest pair
    let result_2 = brute_force::brute_force_closest_points(&points_2);

    // The closest pair is (0.0, 0.0) [index 0] and (0.0, 1.0) [index 2]
    assert_eq!(result_2, Some((0, 2)));
}

#[test]
fn test_divide_and_conquer() {
    // Define a list of points (x, y)
    let points = vec![
        (1.0, 2.0),
        (3.0, 3.0),
        (2.0, 8.0),
        (8.0, 1.0),
        (4.0, 6.0),
        (7.0, 5.0),
    ];

    // Find the closest pair
    let (idx1, idx2) = divide_and_conquer::divide_and_conquer_closest_points(&points);

    // Sort the resulting indices because (2, 4) and (4, 2) are both
    // correct answers for the divide and conquer algorithm.
    let mut result_indices = vec![idx1, idx2];
    result_indices.sort();

    // The closest pair of indices is 0 and 1
    assert_eq!(result_indices, vec![0, 1]);

    let points_2 = vec![
        (0.0, 0.0), // index 0
        (10.0, 10.0),
        (0.0, 1.0), // index 2
    ];

    // Find the closest pair
    let (idx1_2, idx2_2) = divide_and_conquer::divide_and_conquer_closest_points(&points_2);

    // Sort indices
    let mut result_indices_2 = vec![idx1_2, idx2_2];
    result_indices_2.sort();

    // The closest pair of indices is 0 and 2.
    assert_eq!(result_indices_2, vec![0, 2]);
}
