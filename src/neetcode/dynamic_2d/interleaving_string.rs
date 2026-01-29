/*
Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.

An interleaving of two strings s and t is a configuration where s and t are divided into n and m substrings respectively, such that:

s = s1 + s2 + ... + sn
t = t1 + t2 + ... + tm
|n - m| <= 1
The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
Note: a + b is the concatenation of strings a and b.

Example 1:

Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
Output: true
Explanation: One way to obtain s3 is:
Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
Since s3 can be obtained by interleaving s1 and s2, we return true.
Example 2:

Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
Output: false
Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
Example 3:

Input: s1 = "", s2 = "", s3 = ""
Output: true
*/

fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    // try to use one string until you can't anymore - then try to use the other string
    // try match character by character
    // TODO: what to do when we can either use left OR right
    // - decision tree branches when we can pick either from s1 or s2
    //
    // abc bda abd

    // General Dynamic Programming (DP) Problem Template (Top-Down + Bottom-Up)

    // dp is about finding the sub-problem
    // i,j in s1/s2 and k in s3
    // dp(i, j, k) = ?
    //

    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let s3: Vec<char> = s3.chars().collect();

    let (mut i, mut j) = (0, 0);
    // let mut cache: HashMap<(usize, usize, usize), >
    for (k, c) in s3.iter().enumerate() {}

    todo!()
}
