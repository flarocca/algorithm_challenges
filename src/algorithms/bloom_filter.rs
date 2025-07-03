use std::hash::{Hash, Hasher};

use bitvec::vec::BitVec;
use twox_hash::XxHash64;

pub struct BloomFilter {
    k_hash_functions: u8,
    m_bits: usize,
    storage: BitVec,
}

impl BloomFilter {
    /// Creates a new Bloom filter with the specified capacity and error rate.
    /// It uses a single XxHash64 function K times for simplicity. As some
    /// production implementations, different hashhing functions can be used.
    /// XxHash64 is the fastest and it is stable across platforms.
    ///
    /// # Arguments
    /// * `capacity`: The expected number of items to be stored in the filter.
    /// * `error_rate`: The expected error rate expresed as a percentage factor. For
    ///   example, `0.01` for 1%.
    ///
    /// # Example
    /// A Bloom Filter that stores up to 1000 items with a 1% error rate
    ///
    /// ```rust
    /// use bloom_filter::BloomFilter;
    /// let mut filter = BloomFilter::new(1000, 0.01);
    ///
    /// filter.add("example");
    /// assert!(filter.contains("example"));
    /// ```
    pub fn new(capacity: usize, error_rate: f64) -> Self {
        let m = Self::calculate_m(capacity as f64, error_rate);
        let k = Self::calculate_k(m as f64, capacity as f64);

        BloomFilter {
            k_hash_functions: k,
            m_bits: m,
            storage: BitVec::repeat(false, m),
        }
    }

    pub fn add(&mut self, element: &str) {
        for index in 0..self.k_hash_functions {
            let bit_index = self.compute_bitvec_index(element, index);

            self.storage.set(bit_index, true);
        }
    }

    pub fn contains(&self, element: &str) -> bool {
        for index in 0..self.k_hash_functions {
            let bit_index = self.compute_bitvec_index(element, index);

            if !self.storage[bit_index] {
                return false;
            }
        }
        true
    }

    fn compute_bitvec_index(&self, element: &str, index: u8) -> usize {
        let hash = Self::hash_item(element, index);
        (hash % self.m_bits as u64) as usize
    }

    fn hash_item(element: &str, seed: u8) -> u64 {
        let mut hasher = XxHash64::with_seed(seed as u64);
        element.hash(&mut hasher);
        hasher.finish()
    }

    fn calculate_m(capacity: f64, error_rate: f64) -> usize {
        // M is calculated using the formula:
        // m = - (n * ln(p)) / (ln(2)^2)
        let n = capacity;
        let ln_p = (error_rate).ln();
        let ln_2 = 2.0_f64.ln();

        (-(n * ln_p) / (ln_2).powi(2)) as usize
    }

    fn calculate_k(m: f64, capacity: f64) -> u8 {
        // K is calculated using the formula:
        // k = (m / n) * ln(2)
        let n = capacity;
        let ln_2 = 2.0_f64.ln();

        ((m / n) * ln_2) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_m() {
        let m = BloomFilter::calculate_m(1000.0, 0.05);
        assert_eq!(m, 6235);
    }

    #[test]
    fn test_calculate_k() {
        let capacity = 1000.0;
        let m = BloomFilter::calculate_m(capacity, 0.05);
        let k = BloomFilter::calculate_k(m as f64, capacity);
        assert_eq!(k, 4);
    }

    #[test]
    fn test_bloom_filter_add_and_contains() {
        let mut bloom = BloomFilter::new(1000, 0.01);

        bloom.add("test");
        assert!(bloom.contains("test"));
        assert!(!bloom.contains("not-test"));

        bloom.add("rust");
        assert!(bloom.contains("rust"));
        assert!(!bloom.contains("not-rust"));
    }
}
