pub fn lipmaa(n: u64) -> u64 {
    if n < 1 {
        panic!("n must be larger than 0")
    }

    if n == 1 {
        return 1;
    }

    let k = find_k(n);

    if n == ((3u64.pow(k as u32)) - 1) / 2 {
        n - (3u64.pow(k as u32 - 1))
    } else {
        n - (((3u64.pow(g(n) as u32)) - 1) / 2)
    }
}

fn g(n: u64) -> u64 {
    let k = find_k(n);
    if n == ((3u64.pow(k as u32)) - 1) / 2 {
        k
    } else {
        let k = find_new_k(n);
        g(n - (((3u64.pow(k as u32 - 1)) - 1) / 2))
    }
}

fn find_k(n: u64) -> u64 {
    f64::log((n as f64) * 2.0 + 1.0, 3.0) as u64
}

fn find_new_k(n: u64) -> u64 {
    let k = find_k(n) + 1;
    if n < (3u64.pow(k as u32) - 1) / 2 {
        k
    } else {
        k + 1
    }
}

#[cfg(test)]
mod tests {

    use crate::{find_k, lipmaa};

    #[test]
    fn find_k_is_ok() {
        assert_eq!(find_k(1), 1);
        assert_eq!(find_k(3), 1);
        assert_eq!(find_k(4), 2);
        assert_eq!(find_k(5), 2);
        assert_eq!(find_k(12), 2);
        assert_eq!(find_k(13), 3);
        assert_eq!(find_k(40), 4);
    }

    #[test]
    fn lipmaa_spines() {
        let actual_expecteds = [(4, 1), (13, 4), (40, 13)];

        actual_expecteds
            .iter()
            .for_each(|(n, expected)| assert_eq!(lipmaa(*n), *expected))
    }

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
            (17, 13),
            (21, 17),
            (25, 21),
            (26, 13),
            (40, 13),
        ];

        actual_expecteds
            .iter()
            .for_each(|(n, expected)| assert_eq!(lipmaa(*n), *expected))
    }
}
