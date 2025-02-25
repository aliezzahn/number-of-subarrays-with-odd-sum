use number_of_subarrays_with_odd_sum::Solution;

#[test]
fn test_num_of_subarrays() {
    // Test case 1: All odd numbers
    assert_eq!(Solution::num_of_subarrays(vec![1, 1, 1, 1, 1]), 15);

    // Test case 2: All even numbers
    assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);

    // Test case 3: Mixed odd and even numbers
    assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5]), 9);

    // Test case 4: Single odd number
    assert_eq!(Solution::num_of_subarrays(vec![1]), 1);

    // Test case 5: Single even number
    assert_eq!(Solution::num_of_subarrays(vec![2]), 0);

    // Test case 6: Empty array
    assert_eq!(Solution::num_of_subarrays(vec![]), 0);

    // Test case 7: Large array with mixed numbers
    assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 25);
}