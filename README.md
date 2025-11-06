# Closest Pair of Points - Algorithm Analysis

This project implements and compares two algorithms for solving the **Closest Pair of Points** problem: a brute-force approach and a divide-and-conquer approach. The analysis demonstrates the performance differences between O(n²) and O(n log n) time complexities.

## Project Demo
https://youtu.be/f6ZliUxIsMM

## Problem Description

Given a set of n points in a 2D plane, find the pair of points with the smallest Euclidean distance between them.

The primary objectives of this project are, first, to empirically observe the computational complexities of different algorithms that solve the same problem, and second, to analyze how well theoretical complexity estimates align with actual execution times.

To achieve these analytical goals, this project will implement and contrast two solutions for the Closest Pair of Points Problem. This problem is formally defined as: given a set of n distinct points in a two-dimensional plane (e.g., P1, P2, …, Pn), the objective is to identify and return the indices i and j of the two points (Pi and Pj) that are separated by the smallest Euclidean distance.

The input size, denoted by n, represents the total number of points provided to the algorithms. For the experimental analysis in this project, the performance of the algorithms will be empirically measured across a range of significant input sizes. The experiment will use 10 distinct values for n, starting at n=10,000 and increasing in increments of 5,000 up to a maximum of n=55,000.

The core of the analysis will involve contrasting two algorithms with vastly different theoretical efficiencies. The first is the Brute-Force (ALG1) method, which has a time complexity of (n2). The second is the Divide-and-Conquer (ALG2) strategy, which is expected to be more efficient with a time complexity of (nlog2n). This direct comparison will provide the empirical data needed to observe the real-world performance differences predicted by these theoretical bounds. Through this process, the project aims to demonstrate how theoretical complexity analysis predicts practical performance outcomes.

The Closest Pair of Points Problem is not merely a theoretical exercise; it has numerous practical applications in various fields. For example, in air traffic control, it can be used to identify aircraft that are flying too close to each other, helping to prevent potential collisions. In molecular modeling, this algorithm can help find the two closest atoms in a complex protein structure, which is crucial for understanding molecular interactions. It also serves as a fundamental problem in the broader field of computational geometry. While these applications are significant, this project's primary focus will be on the implementation and empirical performance analysis of the algorithms themselves.


## Algorithms

### ALG1: Brute-Force Algorithm
- **Time Complexity**: O(n²)
- **Approach**: Compares every pair of points to find the minimum distance
- **Best for**: Small datasets (n < 10,000)

### ALG2: Divide-and-Conquer Algorithm
- **Time Complexity**: O(n log n)
- **Approach**: 
  1. Sorts points by x-coordinate
  2. Recursively divides the point set into two halves
  3. Finds closest pairs in each half
  4. Checks for closer pairs across the dividing line within a strip
- **Best for**: Large datasets (n ≥ 10,000)

## Features

- Implementation of both algorithms with correctness verification
- Runtime performance comparison across different input sizes
- Visualization of closest pair results using matplotlib
- Empirical runtime analysis with nanosecond precision

## Requirements

- Python 3.x
- matplotlib
- math (standard library)
- time (standard library)
- random (standard library)

## Usage

Open and run the Jupyter notebook:

```bash
jupyter notebook closest_pair_algorithms_runtime_comparison.ipynb
```

The notebook includes:
1. Helper functions for visualization
2. Complete implementation of both algorithms
3. Performance testing framework
4. Runtime comparison across multiple dataset sizes

## Performance Results

The notebook demonstrates that:
- For n = 10,000 points: ALG1 takes ~5,900 ms while ALG2 takes ~58 ms (100x faster)
- For n = 15,000 points: ALG1 takes ~13,800 ms while ALG2 takes ~96 ms (140x faster)
- The performance gap increases significantly as n grows

## Project Structure

```
project-analysis-of-algorithms/
├── closest_pair_algorithms_runtime_comparison.ipynb  # Main analysis notebook
├── README.md                                          # This file
├── pyproject.toml                                     # Project configuration
└── v_rust/                                            # Rust implementation (for testing purposes)
```

## Implementation Details

### Brute-Force Algorithm
```python
def brute_force_closest_points(points):
    # Compares all pairs: O(n²)
    # Returns indices of closest pair
```

### Divide-and-Conquer Algorithm
```python
def divide_and_conquer_closest_points(points):
    # Recursive approach: O(n log n)
    # Uses helper functions:
    # - closest_split_pair: handles strip checking
    # - closest_pair_recursive: recursive implementation
```

## Testing

The notebook includes empirical runtime testing that:
- Generates random point sets of various sizes
- Runs multiple iterations (m) for each size
- Computes average runtimes
- Verifies both algorithms produce identical results

## License

See LICENSE file for details.

## Academic Context

This project was developed as part of an Analysis of Algorithms course at FAU (Florida Atlantic University), demonstrating practical applications of algorithm design and complexity analysis.
