#![no_std]
//! A function to calculate [lipmaa](https://github.com/AljoschaMeyer/bamboo/blob/master/README.md#links-and-entry-verification) sequence numbers.
//!
//! From the bamboo spec: "The lipmaalinks are chosen such that for any pair of entries there is a path from the newer to the older one of a length logarithmic in their distance."
//!
//! ```
//! use lipmaa_link::lipmaa;
//!
//! let result = lipmaa(13);
//! assert_eq!(result, 4);
//! ```
//!

/// Calculates the lipmaa link number given the current sequence number.
pub const fn lipmaa(n: u64) -> u64 {
    let mut m: u128 = 1;
    let mut po3: u128 = 3;
    let mut u: u128 = n as u128;

    // find k such that (3^k - 1)/2 >= n
    while m < n as u128 {
        po3 *= 3;
        m = (po3 - 1) / 2;
    }

    // find longest possible backjump
    po3 /= 3;
    if m != n as u128 {
        while u != 0 {
            m = (po3 - 1) / 2;
            po3 /= 3;
            u %= m;
        }

        if m != po3 {
            po3 = m;
        }
    }

    return n - po3 as u64;
}
const fn get_path_length(mut n: u64) -> u64 {
    let mut len = 0;

    while n > 0 {
        n = lipmaa(n);
        len += 1;
    }

    len
}

/// The number of lipmaa links needed to traverse from u64::MAX back to 0;
pub const MAX_NUM_LIPMAA_LINKS_FOR_U64: u64 = get_path_length(u64::MAX);

/// Will `n` create a "skip" link rather than just `n - 1`
pub fn is_skip_link(sequence_num: u64) -> bool {
    lipmaa(sequence_num) != sequence_num - 1
}

#[cfg(test)]
mod tests {

    use crate::{get_path_length, lipmaa};

    #[test]
    fn lipmaa_is_ok() {
        let actual_expecteds = [
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 4),
            (6, 5),
            (7, 6),
            (8, 4),
            (9, 8),
            (10, 9),
            (11, 10),
            (12, 8),
            (13, 4),
            (14, 13),
            (15, 14),
            (16, 15),
            (17, 13),
            (18, 17),
            (19, 18),
            (20, 19),
            (21, 17),
            (22, 21),
            (23, 22),
            (24, 23),
            (25, 21),
            (26, 13),
            (27, 26),
            (28, 27),
            (29, 28),
            (30, 26),
            (31, 30),
            (32, 31),
            (33, 32),
            (34, 30),
            (35, 34),
            (36, 35),
            (37, 36),
            (38, 34),
            (39, 26),
            (40, 13),
            (121, 40),
        ];

        actual_expecteds
            .iter()
            .for_each(|(n, expected)| assert_eq!(lipmaa(*n), *expected))
    }

    #[test]
    fn get_path_length_works() {
        let actual_expecteds = [(1, 1), (2, 2), (3, 3), (4, 2)];

        actual_expecteds
            .iter()
            .for_each(|(n, expected)| assert_eq!(get_path_length(*n), *expected));
    }

    #[test]
    fn largest_n_u32() {
        assert_eq!(lipmaa(core::u32::MAX as u64), 4294967294);
    }
    #[test]
    fn largest_n_u64_no_explode() {
        lipmaa(core::u64::MAX);
    }
}
