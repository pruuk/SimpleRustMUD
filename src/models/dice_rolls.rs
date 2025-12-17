use rand::Rng;
use rand::thread_rng;
use rand_distr::{Normal, Distribution};

/// Generates a random float from a normal distribution with the given mean and standard deviation.
pub async fn random_distribution_roll_result(mean: f64, std_dev: f64) -> i64 {
    // Create a thread-local random number generator
    let mut rng = thread_rng();

    // Define the normal distribution
    // Normal::new returns a Result, so we unwrap it (it panics if std_dev is negative or not finite)
    let normal = Normal::new(mean, std_dev).unwrap();

    // Sample a value from the distribution
    let v = normal.sample(&mut rng);
    v as i64
}