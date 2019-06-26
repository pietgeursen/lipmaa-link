pub fn lipmaa(n: u32) -> u32 {
    if n < 1 {
        panic!("n must be larger than 0")
    }

    if n == 1 {
        return 1;
    }

    let k = find_k(n);

    if n == (((3u64.pow(k)) - 1) / 2) as u32 {
        n - (3u32.pow(k - 1))
    } else {
        n - (((3u64.pow(g(n))) - 1) / 2) as u32
    }
}

fn g(n: u32) -> u32 {
    let k = find_k(n);
    if n == (((3u64.pow(k)) - 1) / 2) as u32 {
        k
    } else {
        let k = find_new_k(n);
        g(n - (((3u64.pow(k - 1)) - 1) / 2) as u32)
    }
}

fn find_k(n: u32) -> u32 {
    f64::log((n as f64) * 2.0 + 1.0, 3.0) as u32
}

fn find_new_k(n: u32) -> u32 {
    let k = find_k(n) + 1;
    if n < ((3u64.pow(k) - 1) / 2) as u32 {
        k
    } else {
        k + 1
    }
}

#[cfg(test)]
mod tests {

    use crate::{find_k, lipmaa};

    #[test]
    fn lipmaa_is_ok() {
        let actual_expecteds = [
            (1, 1),
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
        ];

        actual_expecteds
            .iter()
            .for_each(|(n, expected)| assert_eq!(lipmaa(*n), *expected))
    }

    #[test]
    fn largest_n() {
        assert_eq!(lipmaa(core::u32::MAX), 4294967294);
    }

    #[test]
    fn find_k_is_ok() {
        assert_eq!(find_k(1), 1);
        assert_eq!(find_k(3), 1);
        assert_eq!(find_k(4), 2);
        assert_eq!(find_k(5), 2);
        assert_eq!(find_k(12), 2);
        assert_eq!(find_k(13), 3);
        assert_eq!(find_k(40), 4);
        assert_eq!(find_k(core::u32::MAX), 20);
    }

    #[test]
    fn lipmaa_spines() {
        let actual_expecteds = [(4, 1), (13, 4), (40, 13)];

        actual_expecteds
            .iter()
            .for_each(|(n, expected)| assert_eq!(lipmaa(*n), *expected))
    }
}
