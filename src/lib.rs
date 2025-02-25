pub struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_count = 0; // Number of prefix sums that are odd
        let mut even_count = 1; // Number of prefix sums that are even (prefix sum of 0 is even)
        let mut prefix_sum = 0; // Current prefix sum
        let mut result = 0; // Result to store the count of sub-arrays with odd sum
        const MOD: i32 = 1_000_000_007; // Modulo value

        for num in arr {
            prefix_sum += num; // Update prefix sum
            if prefix_sum % 2 == 1 {
                // If prefix sum is odd, add the number of even prefix sums to the result
                result = (result + even_count) % MOD;
                odd_count += 1; // Increment odd prefix sum count
            } else {
                // If prefix sum is even, add the number of odd prefix sums to the result
                result = (result + odd_count) % MOD;
                even_count += 1; // Increment even prefix sum count
            }
        }

        result
    }
}