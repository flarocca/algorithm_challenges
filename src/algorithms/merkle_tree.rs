use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Display;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Hash32([u8; 32]);

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProofStep {
    pub side: Side,
    pub hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Proof {
    pub leaf_hash: String,
    pub steps: Vec<ProofStep>,
}

#[derive(Clone)]
pub struct MerkleTree {
    levels: Vec<Vec<Hash32>>,
    root: Hash32,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Hash32>) -> Self {
        assert!(
            !leaves.is_empty(),
            "Cannot build a Merkle tree with no leaves"
        );

        let mut current_level = leaves;
        let mut levels = vec![current_level.clone()];

        while current_level.len() > 1 {
            let even_level = Self::add_last_if_odd(&current_level);
            let next_level = Self::build_next_level(&even_level);

            levels.push(next_level.clone());
            current_level = next_level;
        }

        // The last (and only one) element in the last level is the root
        // for convenience and simplicity we keep it separately
        let root = levels[levels.len() - 1][0];

        Self { levels, root }
    }

    pub fn root(&self) -> Hash32 {
        self.root
    }

    pub fn proof(&self, leaf_index: usize) -> Proof {
        assert!(
            leaf_index < self.levels[0].len(),
            "Leaf index out of bounds"
        );

        let mut idx = leaf_index;
        let mut steps = Vec::new();

        for level_index in 0..self.levels.len() - 1 {
            let current_level = &self.levels[level_index];
            let (sibling_hash, side) = Self::get_sibling_hash_and_side(current_level, idx);

            steps.push(ProofStep {
                side,
                hash: sibling_hash.to_hex(),
            });

            // `usize` rounds in a floor-like way, therefore odd / 2 is handled
            // correctly due to the 0-indexing. For example, if current index is 5, then its parent
            // index is 2 in the next level (5 / 2 = 2). If current index is 4, then its parent index is also 2 (4 /
            // 2 = 2). Positions 0 and 1 map to 0, 2 and 3 map to 1, and so on.
            idx /= 2;
        }

        Proof {
            leaf_hash: self.levels[0][leaf_index].to_hex(),
            steps,
        }
    }

    pub fn verify(leaf: &Hash32, proof: &Proof, root: &Hash32) -> bool {
        let mut actual = *leaf;
        for step in &proof.steps {
            let Ok(sibling) = Hash32::from_hex(&step.hash) else {
                return false;
            };

            actual = match step.side {
                Side::Left => Hash32::from((&sibling, &actual)),
                Side::Right => Hash32::from((&actual, &sibling)),
            };
        }

        &actual == root
    }
}

impl MerkleTree {
    fn get_sibling_hash_and_side(level: &[Hash32], index: usize) -> (Hash32, Side) {
        let is_last_odd = !level.len().is_multiple_of(2) && index == level.len() - 1;
        let is_multiple_of_2 = index.is_multiple_of(2);

        let sibling_index = if is_multiple_of_2 {
            index + 1
        } else {
            index - 1
        };

        if is_last_odd {
            (level[index], Side::Right)
        } else if is_multiple_of_2 {
            (level[sibling_index], Side::Right)
        } else {
            (level[sibling_index], Side::Left)
        }
    }

    /// If the number of elements in this level is odd, it is turned into even length by
    /// duplicating the last element.
    /// Now the last element is paired with itself and hashed to form the parent node.
    ///
    /// # Pre-conditions
    /// - `level` is not empty.
    fn add_last_if_odd(level: &[Hash32]) -> Vec<Hash32> {
        let mut level = level.to_vec();

        if level.len() % 2 == 1 {
            let last = *level.last().expect("Pre-condition: level is not empty");
            level.push(last);
        }

        level
    }

    /// Builds the next level of the Merkle tree from the given level by hashing pairs.
    ///
    /// # Pre-conditions
    /// - `level` has even length.
    fn build_next_level(level: &[Hash32]) -> Vec<Hash32> {
        let mut next = Vec::with_capacity(level.len() / 2);

        for i in (0..level.len()).step_by(2) {
            next.push(Hash32::from((&level[i], &level[i + 1])));
        }

        next
    }
}

impl Hash32 {
    pub fn empty() -> Self {
        Hash32([0u8; 32])
    }

    pub fn hash(data: &[u8]) -> Hash32 {
        let digest = Sha256::digest(data);
        Hash32(digest.into())
    }

    pub fn to_hex(self) -> String {
        hex::encode(self)
    }

    pub fn from_hex(hash: &str) -> Result<Self, &'static str> {
        let Ok(bytes) = hex::decode(hash) else {
            return Err("Invalid hex string");
        };

        let Ok(hash) = Hash32::try_from(bytes.as_slice()) else {
            return Err("Invalid hash length");
        };

        Ok(hash)
    }
}

impl From<(&Hash32, &Hash32)> for Hash32 {
    fn from((left, right): (&Hash32, &Hash32)) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(left.0);
        hasher.update(right.0);

        Self(hasher.finalize().into())
    }
}

impl From<[u8; 32]> for Hash32 {
    fn from(x: [u8; 32]) -> Self {
        Hash32(x)
    }
}

impl From<Hash32> for [u8; 32] {
    fn from(x: Hash32) -> Self {
        x.0
    }
}

impl TryFrom<&[u8]> for Hash32 {
    type Error = &'static str;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() != 32 {
            return Err("Invalid length: expected 32 bytes");
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(slice);

        Ok(Hash32(array))
    }
}

impl TryFrom<Vec<u8>> for Hash32 {
    type Error = &'static str;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        Hash32::try_from(vec.as_slice())
    }
}

impl AsRef<[u8]> for Hash32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Left => write!(f, "Left"),
            Side::Right => write!(f, "Right"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl From<&str> for Hash32 {
        fn from(s: &str) -> Self {
            Hash32::hash(s.as_bytes())
        }
    }

    #[test]
    #[should_panic(expected = "Cannot build a Merkle tree with no leaves")]
    fn build_panics_on_empty() {
        // Act & Assert
        let _ = MerkleTree::new(vec![]);
    }

    #[test]
    fn single_leaf_root_is_leaf() {
        // Arrange
        let a: Hash32 = ("A").into();

        // Act
        let tree = MerkleTree::new(vec![a]);

        // Assert
        assert_eq!(tree.root(), a);
    }

    #[test]
    fn single_leaf_root_valid_prrof() {
        // Arrange
        let a: Hash32 = ("A").into();
        let tree = MerkleTree::new(vec![a]);

        // Act
        let p = tree.proof(0);

        // Assert
        assert!(MerkleTree::verify(&a, &p, &tree.root()));
    }

    #[test]
    fn single_leaf_root_empty_proof_steps() {
        // Arrange
        let a: Hash32 = ("A").into();
        let tree = MerkleTree::new(vec![a]);

        // Act
        let p = tree.proof(0);

        // Assert
        assert!(p.steps.is_empty());
    }

    #[test]
    fn two_leaves_valid_root() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let tree = MerkleTree::new(vec![a, b]);
        let expected_root = Hash32::from((&a, &b));

        // Act & Assert
        assert_eq!(tree.root(), expected_root);
    }

    #[test]
    fn two_leaves_valid_proofs() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let tree = MerkleTree::new(vec![a, b]);
        let expected_root = Hash32::from((&a, &b));

        // Act & Assert
        let pa = tree.proof(0);
        assert!(MerkleTree::verify(&a, &pa, &expected_root));

        let pb = tree.proof(1);
        assert!(MerkleTree::verify(&b, &pb, &expected_root));
    }

    #[test]
    fn even_leaves_valid_root() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let c: Hash32 = ("C").into();
        let d: Hash32 = ("D").into();

        let ab = Hash32::from((&a, &b));
        let cd = Hash32::from((&c, &d));
        let expected_root = Hash32::from((&ab, &cd));

        // Act
        let tree = MerkleTree::new(vec![a, b, c, d]);

        // Assert
        assert_eq!(tree.root(), expected_root);
    }

    #[test]
    fn even_leaves_valid_proofs() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let c: Hash32 = ("C").into();
        let d: Hash32 = ("D").into();

        let ab = Hash32::from((&a, &b));
        let cd = Hash32::from((&c, &d));
        let expected_root = Hash32::from((&ab, &cd));

        let tree = MerkleTree::new(vec![a, b, c, d]);

        // Act & Assert
        for (i, leaf) in [a, b, c, d].iter().enumerate() {
            let p = tree.proof(i);
            assert!(
                MerkleTree::verify(leaf, &p, &expected_root),
                "leaf {i} failed"
            );
        }
    }

    #[test]
    fn odd_leaf_valid_root() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let c: Hash32 = ("C").into();

        let ab = Hash32::from((&a, &b));
        let cc = Hash32::from((&c, &c));
        let expected_root = Hash32::from((&ab, &cc));

        // Act
        let tree = MerkleTree::new(vec![a, b, c]);

        // Assert
        assert_eq!(tree.root(), expected_root);
    }

    #[test]
    fn odd_leaf_valid_proofs() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let c: Hash32 = ("C").into();

        let ab = Hash32::from((&a, &b));
        let cc = Hash32::from((&c, &c));
        let expected_root = Hash32::from((&ab, &cc));

        let tree = MerkleTree::new(vec![a, b, c]);

        // Act & Assert
        for (i, leaf) in [a, b, c].iter().enumerate() {
            let p = tree.proof(i);
            assert!(
                MerkleTree::verify(leaf, &p, &expected_root),
                "leaf {i} failed"
            );
        }
    }

    #[test]
    fn verification_fails_on_tampered_leaf() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let tree = MerkleTree::new(vec![a, b]);
        let proof_a = tree.proof(0);

        // Act
        let tampered: Hash32 = ("A'").into();

        // Assert
        assert!(!MerkleTree::verify(&tampered, &proof_a, &tree.root()));
    }

    #[test]
    fn verification_fails_on_tampered_proof_step() {
        // Arrange
        let a: Hash32 = ("A").into();
        let b: Hash32 = ("B").into();
        let tree = MerkleTree::new(vec![a, b]);

        let mut p = tree.proof(0);

        // Act
        let tampered: Hash32 = ("invalid hash").into();
        p.steps[0].hash = tampered.to_hex();

        // Assert
        assert!(!MerkleTree::verify(&a, &p, &tree.root()));
    }

    #[test]
    #[should_panic(expected = "Leaf index out of bounds")]
    fn proof_panics_on_out_of_bounds_index() {
        // Arrange
        let a: Hash32 = ("A").into();
        let tree = MerkleTree::new(vec![a]);
        let out_of_bounds_index = 1;

        // Act & Assert
        let _ = tree.proof(out_of_bounds_index);
    }
}
