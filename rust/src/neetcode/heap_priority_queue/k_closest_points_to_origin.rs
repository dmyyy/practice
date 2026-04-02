/*

You are given an 2-D array points where points[i] = [xi, yi] represents the coordinates of a point on an X-Y axis plane. You are also given an integer k.

Return the k closest points to the origin (0, 0).

The distance between two points is defined as the Euclidean distance (sqrt((x1 - x2)^2 + (y1 - y2)^2)).

You may return the answer in any order.

Example 1:

Input: points = [[0,2],[2,2]], k = 1

Output: [[0,2]]
Explanation : The distance between (0, 2) and the origin (0, 0) is 2. The distance between (2, 2) and the origin is sqrt(2^2 + 2^2) = 2.82842. So the closest point to the origin is (0, 2).

Example 2:

Input: points = [[0,2],[2,0],[2,2]], k = 2

Output: [[0,2],[2,0]]
Explanation: The output [2,0],[0,2] would also be accepted.

*/

// Storing and ordering anything using floats is a PITA. Need either ordered-float crate or kind of hack around the
// restrictiotion using a manual Ord impl that will panic on NaN. Here I avoided issues by calculating dist in the ord
// derive

use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(PartialEq, Eq)]
struct PointMeta {
    point: Vec<i32>,
}

impl PartialOrd for PointMeta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let dist_to_origin = ((self.point[0].pow(2) + self.point[1].pow(2)) as f64).sqrt();
        let other_dist_to_origin = ((other.point[0].pow(2) + other.point[1].pow(2)) as f64).sqrt();
        dist_to_origin.partial_cmp(&other_dist_to_origin)
    }
}
impl Ord for PointMeta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::new();

    for point in points {
        heap.push(Reverse(PointMeta { point }));
    }

    let mut res = Vec::with_capacity(k as usize);
    for _ in 0..k {
        let point_meta = heap.pop().unwrap();
        res.push(point_meta.0.point);
    }
    res
}
