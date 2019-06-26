#[no_mangle]
pub extern "C" fn lipmaa(n: u32) -> u32 {
    let mut m: u64 = 1;
    let mut po3: u64 = 3;
    let mut u: u64 = n as u64;

    // find k such that (3^k - 1)/2 >= n
    while m < n as u64 {
        po3 *= 3;
        m = (po3 - 1) / 2;
    }

    // find longest possible backjump
    po3 /= 3;
    if m != n as u64 {
        while u != 0 {
            m = (po3 - 1) / 2;
            po3 /= 3;
            u %= m;
        }

        if m != po3 {
            po3 = m;
        }
    }

    return n - po3 as u32;
}
#[cfg(test)]
mod tests {

    use crate::lipmaa;

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
    fn largest_n() {
        assert_eq!(lipmaa(core::u32::MAX), 4294967294);
    }
}
