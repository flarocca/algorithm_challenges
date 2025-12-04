pub struct Solution;

impl Solution {
    /// Counts the number of collisions between cars based on their directions.
    ///
    /// # Parameters
    /// - `directions`: A string consisting of characters 'L', 'R', and 'S', where:
    ///     - 'L' represents a car moving to the left,
    ///     - 'R' represents a car moving to the right,
    ///     - 'S' represents a stationary car.
    ///
    /// # Collision Rules
    /// - A collision occurs when a moving car ('L' or 'R') encounters a stationary car ('S') or a car moving in the opposite direction.
    /// - Cars moving left ('L') at the start and cars moving right ('R') at the end never collide.
    /// - The function counts the number of cars that will collide (i.e., all non-stationary cars between the first non-left and last non-right car).
    ///
    /// # Returns
    /// The total number of collisions that will occur.
    pub fn count_collisions(directions: String) -> i32 {
        let bytes = directions.as_bytes();
        let n = bytes.len();

        let mut left = 0;
        while left < n && bytes[left] == b'L' {
            left += 1;
        }

        let mut right = n;
        while right > left && bytes[right - 1] == b'R' {
            right -= 1;
        }

        let mut collisions = 0;
        (left..right).for_each(|i| {
            if bytes[i] != b'S' {
                collisions += 1;
            }
        });

        collisions
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use rstest::rstest;

    #[rstest]
    #[case("RLRSLL", 5)]
    #[case("LLRR", 0)]
    #[case("RRRR", 0)]
    #[case("LLLL", 0)]
    #[case("SSSS", 0)]
    #[case("SSRSSRLLRSLLRSRSSRLRRRRLLRRLSSRR", 20)]
    #[case("RS", 1)]
    #[case("SR", 0)]
    #[case("SL", 1)]
    #[case("LS", 0)]
    fn test_count_collisions(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(Solution::count_collisions(input.to_string()), expected);
    }
}
