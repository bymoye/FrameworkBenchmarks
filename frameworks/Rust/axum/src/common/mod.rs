use std::{env, str::FromStr};

use core::fmt::Debug;
use rand::{distr::Uniform, rngs::SmallRng, Rng, RngCore};
pub mod models;
pub mod utils;

#[cfg(feature = "simd-json")]
pub mod simd_json;

#[allow(dead_code)]
pub const SELECT_ALL_FORTUNES: &str = "SELECT * FROM fortune";
#[allow(dead_code)]
pub const SELECT_WORLD_BY_ID: &str =
    "SELECT id, randomnumber FROM world WHERE id = $1 LIMIT 1";
#[allow(dead_code)]
pub const SELECT_ALL_CACHED_WORLDS: &str =
    "SELECT id, randomnumber FROM world ORDER BY id";
#[allow(dead_code)]
pub const UPDATE_WORLDS: &str = r#"UPDATE world SET randomnumber = new.rnum FROM
    (SELECT * FROM UNNEST($1::int[], $2::int[]) AS v(id, rnum) ORDER BY 1) AS new
WHERE world.id = new.id"#;

/// Return the value of an environment variable.
#[allow(dead_code)]
pub fn get_env<T: FromStr>(key: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    env::var(key)
        .unwrap_or_else(|_| panic!("{key} environment variable was not set"))
        .parse::<T>()
        .unwrap_or_else(|_| panic!("could not parse {key}"))
}

/// Generate a single integer in the range 1 to 10,000 (inclusive)
#[allow(dead_code)]
#[inline(always)]
pub fn random_id(rng: &mut impl RngCore) -> i32 {
    rng.random_range(1..=10_000)
}

/// Generate an iterator of integers in the range 1 to 10,000 (inclusive)
#[allow(dead_code)]
#[inline(always)]
pub fn random_ids(rng: &mut SmallRng, count: usize) -> impl Iterator<Item = i32> + use<'_> {
    rng.sample_iter(Uniform::new_inclusive(1, 10_000).unwrap())
        .take(count)
}