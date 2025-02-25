# Number of Sub-arrays with Odd Sum

[![CI](https://github.com/aliezzahn/number-of-subarrays-with-odd-sum/actions/workflows/ci.yml/badge.svg)](https://github.com/aliezzahn/number-of-subarrays-with-odd-sum/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Problem Description

Given an array of integers `arr`, return the number of sub-arrays with an odd sum. A sub-array is a contiguous part of the array.

### Example

```rust
assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
```

## Solution

The solution uses a **prefix sum approach** to efficiently count the number of sub-arrays with an odd sum. It leverages the properties of odd and even numbers to determine the result in linear time.

### Key Insights

1. The sum of a sub-array is odd if the count of odd numbers in the sub-array is odd.
2. We use prefix sums to track the cumulative sum up to each index.
3. By maintaining counts of odd and even prefix sums, we can determine the number of valid sub-arrays.

### Complexity

- **Time Complexity**: O(n), where `n` is the length of the array.
- **Space Complexity**: O(1), as we only use a few variables to track counts.

## Project Structure

```
number-of-subarrays-with-odd-sum/
├── src/
│   └── lib.rs            # Main implementation
├── tests/
│   └── integration_test.rs # Integration tests
├── .github/
│   └── workflows/
│       ├── ci.yml        # CI pipeline
│       └── cd.yml        # CD pipeline
├── Cargo.toml            # Project metadata and dependencies
├── README.md             # Project documentation
├── LICENSE               # License file
└── .gitignore            # Files to ignore in Git
```

## Usage

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).

### Steps

1. Clone the repository:

   ```bash
   git clone https://github.com/aliezzahn/number-of-subarrays-with-odd-sum.git
   cd number-of-subarrays-with-odd-sum
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the tests:

   ```bash
   cargo test
   ```

4. Use the solution in your project:
   Add the following to your `Cargo.toml`:

   ```toml
   [dependencies]
   number-of-subarrays-with-odd-sum = { git = "https://github.com/aliezzahn/number-of-subarrays-with-odd-sum.git" }
   ```

   Then, use the solution in your Rust code:

   ```rust
   use number_of_subarrays_with_odd_sum::Solution;

   fn main() {
       let arr = vec![1, 2, 3, 4, 5];
       let result = Solution::num_of_subarrays(arr);
       println!("Number of sub-arrays with odd sum: {}", result);
   }
   ```

## CI/CD Pipelines

This project uses GitHub Actions for Continuous Integration (CI) and Continuous Deployment (CD).

### CI Pipeline

- Triggered on every push to the `main` branch or pull request.
- Runs tests to ensure the solution is correct.

### CD Pipeline

- Triggered when a new tag is pushed (e.g., `v0.1.0`).
- Builds the project and publishes a release to GitHub Releases.

## Contributing

Contributions are welcome! If you'd like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add your feature"
   ```
4. Push to the branch:
   ```bash
   git push origin feature/your-feature-name
   ```
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Rust community for providing excellent tools and resources.

## Contact

For questions or feedback, please open an issue on GitHub or contact:

- **Your Name**
- Email: aliezzahn@gmail.com
- GitHub: [aliezzahn](https://github.com/aliezzahn)
