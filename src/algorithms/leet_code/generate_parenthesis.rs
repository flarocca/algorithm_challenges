#[derive(Debug)]
pub struct Node {
    pub value: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: String, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self { value, left, right }
    }
}

pub struct Solution;

impl Solution {
    /// Generate Parentheses
    ///
    /// Given n pairs of parentheses, write a function to generate all combinations of well-formed
    /// parentheses.
    ///
    /// Example 1:
    ///     Input: n = 3
    ///     Output: ["((()))","(()())","(())()","()(())","()()()"]
    ///
    /// Example 2:
    ///     Input: n = 1
    ///     Output: ["()"]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let result = Self::generate_tree(n, n + 1, "(".to_owned());
        let paths = Self::extract_paths(Vec::new(), result);
        let valid_paths = Self::filter_paths(paths, n);

        valid_paths
            .into_iter()
            .map(|inner| inner.join(""))
            .collect()
    }

    fn generate_tree(open: i32, close: i32, value: String) -> Option<Box<Node>> {
        if (open == 0 && close == 0) || (open > close) {
            return None;
        }

        let left = if open > 0 {
            Self::generate_tree(open - 1, close, "(".to_owned())
        } else {
            None
        };
        let right = if close > 0 {
            Self::generate_tree(open, close - 1, ")".to_owned())
        } else {
            None
        };

        Some(Box::new(Node::new(value, left, right)))
    }

    fn extract_paths(mut path: Vec<String>, node: Option<Box<Node>>) -> Vec<Vec<String>> {
        match node {
            None => vec![path],
            Some(parent) => {
                path.push(parent.value);
                let mut left = Self::extract_paths(path.clone(), parent.left);
                left.extend(Self::extract_paths(path, parent.right));

                left
            }
        }
    }

    fn filter_paths(paths: Vec<Vec<String>>, n: i32) -> Vec<Vec<String>> {
        let mut valid_paths = Vec::new();

        for path in paths {
            let mut stack = Vec::new();

            for c in &path {
                if c.as_str() == "(" {
                    stack.push(c);
                } else {
                    let _ = stack.pop();
                }
            }

            if stack.is_empty() {
                valid_paths.push(path.clone());
            }
        }

        valid_paths
            .into_iter()
            .filter(|inner| inner.len() == n as usize * 2)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use rstest::rstest;

    #[rstest]
    #[case(1, &["()"])]
    #[case(2, &["(())","()()"])]
    #[case(3, &["((()))","(()())","(())()","()(())","()()()"])]
    #[case(4, &["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"])]
    fn test_generate_parenthesis(#[case] input: i32, #[case] expected: &[&str]) {
        assert_eq!(Solution::generate_parenthesis(input), expected);
    }
}
