/*
Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

The overall run time complexity should be O(log (m+n)).

Example 1:

Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.
Example 2:

Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
*/

// TODO: left + right partitions needed
// left and

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // binary search
    // - even - add two middle values and divide
    // - odd - return the middle value

    // brute force
    // - iterate through and merge both arrays in O(n+m) time
    // - find the (n+m)/2 element (or two elements if even) and calculate median
    //
    //

    // binary search
    // let (mut l, mut r) = (0, std::cmp::max())

    todo!()
}
