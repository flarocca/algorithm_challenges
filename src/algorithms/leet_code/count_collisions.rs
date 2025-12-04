pub struct Solution;

impl Solution {
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
