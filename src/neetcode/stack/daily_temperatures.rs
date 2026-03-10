/*
Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

Example 1:

Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]
Example 2:

Input: temperatures = [30,40,50,60]
Output: [1,1,1,0]
Example 3:

Input: temperatures = [30,60,90]
Output: [1,1,0]
*/

fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // monotonically decreasing temps
    let mut s: Vec<(i32, usize)> = Vec::with_capacity(temperatures.len());
    let mut ans = vec![0; temperatures.len()];
    for (i, &t) in temperatures.iter().enumerate() {
        if !s.is_empty() && t > s.last().unwrap().0 {
            // is greater! keep popping items off stack and updating them
            while let Some(t2) = s.last().cloned()
                && t > t2.0
            {
                s.pop();
                ans[t2.1] = (i - t2.1) as i32;
            }
        }
        s.push((t, i));
    }
    ans
}
