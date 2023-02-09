use std::{
    collections::hash_map::DefaultHasher,
    hash::{BuildHasher, BuildHasherDefault, Hash, Hasher},
    marker::PhantomData,
};

const M: usize = 256;
const B: u8 = 8;

/// An approximative distinct element counter with constant memory requirements
///
/// Uses a 64-bit hash function, just like HyperLogLog++, otherwise follows the original
/// implementation.
///
/// # Possible improvements
/// * Allow precision to be configured. (Currently uses constants M and B)
/// * Implement further bias corrections from the HLL++ paper
/// * Implement sparse representation from HLL++ paper
///
/// # Examples
///
/// ```
/// use algorithms::hyper_log_log::HyperLogLog;
///
/// let mut hll = HyperLogLog::new();
/// for item in (0..100_000) {
///     hll.add(&item);
/// }
/// for item in (50_000..150_000) {
///     hll.add(&item);
/// }
/// // hll.count() should be approximately 150,000
/// assert!(hll.count() > 140_000 && hll.count() < 160_000);
/// ```
pub struct HyperLogLog<T: Hash> {
    registers: [u8; M],
    hash_builder: BuildHasherDefault<DefaultHasher>,
    _marker: PhantomData<T>,
}

impl<T: Hash> HyperLogLog<T> {
    /// Creates a new HyperLogLog instance
    ///
    /// Configured to always use 256 registers.
    pub fn new() -> Self {
        Self {
            registers: [0; M],
            hash_builder: BuildHasherDefault::<DefaultHasher>::default(),
            _marker: PhantomData,
        }
    }

    /// Estimates the error rate of this HyoerLogLog implementation
    pub fn error_rate() -> f64 {
        1.04 / (M as f64).sqrt()
    }

    /// Calculates the bias correction constant, assumes M > 128
    fn am() -> f64 {
        0.7213 / (1f64 + 1.079 / M as f64)
    }

    /// Adds an item
    pub fn add(&mut self, item: &T) {
        let mut hasher = self.hash_builder.build_hasher();
        item.hash(&mut hasher);
        let hash = hasher.finish();
        // leading 64 - b bits
        let w = hash >> B;
        // last b bits
        let register = (hash - (w << B)) as usize;

        let leading_zeros = w.leading_zeros() as u8 + 1 - B;
        self.registers[register] = leading_zeros.max(self.registers[register]);
    }

    /// Counts the number of registers that are equal to zero
    fn empty_registers(&self) -> usize {
        self.registers
            .iter()
            .filter(|register| **register == 0)
            .count()
    }

    /// Estimates count based on a linear count
    fn linear_count(&self, empty_registers: usize) -> u64 {
        (M as f64 * (M as f64 / empty_registers as f64).log2()) as u64
    }

    /// Counts the number of distinct elements that have been seen
    pub fn count(&self) -> u64 {
        let z = 1f64
            / self
                .registers
                .iter()
                .map(|&i| 2f64.powi(-(i32::from(i))))
                .sum::<f64>();

        let estimate = Self::am() * M as f64 * M as f64 * z;
        if estimate < M as f64 * 5. / 2. {
            let empty_registers = self.empty_registers();
            if empty_registers == 0 {
                estimate as u64
            } else {
                self.linear_count(empty_registers)
            }
        } else {
            estimate as u64
        }
    }

    /// Creates a new HyperLogLog by merging this instance with another
    pub fn merge(&mut self, other: &Self) -> Self {
        let mut registers = [0; M];
        let it = self.registers.iter().zip(other.registers.iter());
        for (i, (v1, v2)) in it.enumerate() {
            registers[i] = *v1.max(v2);
        }
        Self {
            registers,
            hash_builder: BuildHasherDefault::<DefaultHasher>::default(),
            _marker: PhantomData,
        }
    }
}


impl<T: Hash> Default for  HyperLogLog<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {

    use std::cmp::{max, min};

    use super::*;

    fn error_rate(estimated_count: u64, true_count: u64) -> f64 {
        (max(estimated_count, true_count) - min(estimated_count, true_count)) as f64
            / max(estimated_count, true_count) as f64
    }

    fn assert_acceptable_error_rate(
        estimated_count: u64,
        true_count: u64,
        acceptable_error_rate: f64,
    ) {
        let error_rate = error_rate(estimated_count, true_count);
        assert!(
            error_rate < acceptable_error_rate,
            "Expected and error rate less than {:.6}, got {:.6}",
            acceptable_error_rate, error_rate
        );
    }

    /// Test that the error rate is sufficiently small for a set which is much larger
    /// than M.
    ///
    /// Note:
    ///     This test can fail due to bad luck. To reduce the risk off failing due to bad luck
    ///     it only tests that the error is smaller than 2x of the estimated error rate.
    #[test]
    fn test_large_set() {
        let n = 100_000;
        let multiples_of_two = (0..n).step_by(2);
        let multiples_of_three = (0..n).step_by(3);
        let items = multiples_of_two.chain(multiples_of_three);
        let num_distinct = n / 2 + n / 3 - n / 6;

        let mut hll = HyperLogLog::new();
        for item in items {
            hll.add(&item);
        }

        assert_acceptable_error_rate(
            hll.count(),
            num_distinct,
            2.0 * HyperLogLog::<u8>::error_rate(),
        );
    }
    /// Test that the error rate is sufficiently small for a small set.
    ///
    /// Note:
    ///     This test can fail due to bad luck. To reduce the risk off failing due to bad luck
    ///     it only tests that the error is smaller than 2x of the estimated error rate.
    #[test]
    fn test_small_set() {
        let items = vec!["a", "a", "b", "c", "d"];
        let true_distinct = 4;

        let mut hll = HyperLogLog::new();
        for item in items {
            hll.add(&item);
        }

        let distinct = hll.count();
        assert!(
            distinct <= 5 && distinct >= 3,
            "Distinct elements: {}, found {}",
            true_distinct, distinct
        );
    }
    #[test]
    fn test_merge() {
        let mut hll = HyperLogLog::new();
        let mut other_hll = HyperLogLog::new();
        let num_distinct = 150_000;
        for item in 0..100_000 {
            hll.add(&item);
        }
        for item in 50_000..150_000 {
            other_hll.add(&item);
        }
        let merged_hll = hll.merge(&other_hll);

        assert_acceptable_error_rate(
            merged_hll.count(),
            num_distinct,
            2.0 * HyperLogLog::<u8>::error_rate(),
        );
    }
}
