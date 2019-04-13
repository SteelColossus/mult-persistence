fn mult_one_digit(digits: &mut Vec<u8>, mult: u8) {
    match mult {
        0 => {
            digits.truncate(1);
            digits[0] = 0;
        }
        1 => (),
        2...9 => {
            let mut carry_over = 0;

            for d in digits.iter_mut().rev() {
                let d_mult = *d * mult + carry_over;
                carry_over = d_mult / 10;
                *d = d_mult % 10;
            }

            if carry_over > 0 {
                digits.insert(0, carry_over);
            }
        }
        _ => panic!("Invalid digit: {}", mult),
    }
}

fn mult_digits(digits: &[u8]) -> Vec<u8> {
    if digits.len() == 1 {
        digits.to_vec()
    } else {
        let mut new_digits = vec![1];

        for d in digits.iter() {
            mult_one_digit(&mut new_digits, *d);

            if new_digits.len() == 1 && new_digits[0] == 0 {
                break;
            }
        }

        new_digits
    }
}

fn mult_persistence(digits: &[u8]) -> (Vec<u8>, u8) {
    let mut new_digits = digits.to_vec();
    let mut n_times = 0;

    while new_digits.len() > 1 {
        new_digits = mult_digits(&new_digits);
        n_times += 1;
    }

    (new_digits, n_times)
}

fn initial_candidate(len: usize) -> Vec<u8> {
    let mut candidate = Vec::with_capacity(len);

    match len {
        0 => panic!("Can't have a candidate with zero digits!"),
        1 => candidate.push(0),
        _ => candidate.push(2),
    }

    for _i in 1..len {
        candidate.push(6);
    }

    candidate
}

fn is_candidate(digits: &[u8]) -> bool {
    let mut highest = 0;
    let mut has_two = false;
    let mut has_three = false;
    let mut has_five = false;
    let mut has_even = false;

    for d in digits.iter() {
        if *d == 5 {
            has_five = true;
        } else if *d % 2 == 0 {
            has_even = true;
        }

        if *d == 0
            || *d == 1
            || *d < highest
            || (has_two && (*d == 2 || *d == 3 || *d == 4))
            || (has_three && *d == 3)
            || (has_five && has_even)
            || digits.len() < 2
        {
            return false;
        }

        highest = *d;

        if *d == 2 {
            has_two = true;
        } else if *d == 3 {
            has_three = true;
        }
    }

    true
}

fn fill_highest(candidate: &mut [u8]) {
    let mut max = 0;
    let mut max_i = 0;

    for (i, d) in candidate.iter().enumerate() {
        if max < *d {
            max = *d;
            max_i = i;
        }

        if max == 9 {
            break;
        }
    }

    for d in candidate.iter_mut().skip(max_i + 1) {
        *d = max;
    }
}

fn next_candidate(prev_candidate: &[u8]) -> Vec<u8> {
    let mut curr_candidate = prev_candidate.to_vec();
    let mut len = curr_candidate.len();
    let mut i = len - 1;
    let mut i_changed = false;
    let mut new_digit_needed = false;

    loop {
        while curr_candidate[i] == 9 {
            i_changed = true;

            if i == 0 {
                new_digit_needed = true;
                break;
            }

            i -= 1;
        }

        if i_changed {
            if new_digit_needed {
                len += 1;
                curr_candidate.insert(0, 2);
                new_digit_needed = false;
            } else {
                curr_candidate[i] += 1;
            }

            for d in curr_candidate.iter_mut().skip(i + 1) {
                *d = 4;
            }

            i = len - 1;
            i_changed = false;

            if !new_digit_needed {
                fill_highest(&mut curr_candidate);
            }
        } else {
            curr_candidate[i] += 1;
        }

        if is_candidate(&curr_candidate) {
            break;
        }
    }

    curr_candidate
}

pub fn calc_slice(max: usize) -> (Vec<u8>, u8, u8) {
    let mut digits = initial_candidate(max);
    let mut max_times = 0;
    let mut max_digits = digits.clone();
    let mut res_digit = 0;

    loop {
        if digits.len() > max {
            break;
        }

        let (res_digits, n_times) = mult_persistence(&digits);

        if max_times < n_times {
            max_times = n_times;
            max_digits = digits.clone();
            res_digit = res_digits[0];
        }

        digits = next_candidate(&digits);
    }

    (max_digits, res_digit, max_times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_zero_test() {
        let mut digits = vec![1, 8, 1];
        mult_one_digit(&mut digits, 0);
        assert_eq!(digits, vec![0]);
    }

    #[test]
    fn multiplying_by_one_test() {
        let mut digits = vec![1, 8, 1];
        mult_one_digit(&mut digits, 1);
        assert_eq!(digits, vec![1, 8, 1]);
    }

    #[test]
    fn multiplying_by_two_test() {
        let mut digits = vec![1, 8, 1];
        mult_one_digit(&mut digits, 2);
        assert_eq!(digits, vec![3, 6, 2]);
    }

    #[test]
    fn multiplying_by_nine_test() {
        let mut digits = vec![1, 8, 1];
        mult_one_digit(&mut digits, 9);
        assert_eq!(digits, vec![1, 6, 2, 9]);
    }

    #[test]
    fn multiplying_large_numbers_test() {
        let mut digits = vec![1, 3, 5, 0, 8, 5, 1, 7, 1, 7, 6, 7, 2, 9, 9, 2, 0, 8, 9];
        mult_one_digit(&mut digits, 7);
        assert_eq!(
            digits,
            vec![9, 4, 5, 5, 9, 6, 2, 0, 2, 3, 7, 1, 0, 9, 4, 4, 6, 2, 3]
        );
    }

    #[test]
    #[should_panic]
    fn multiplying_by_ten_test() {
        let mut digits = vec![1, 8, 1];
        mult_one_digit(&mut digits, 10);
    }

    #[test]
    fn mult_persistence_test() {
        let (new_digits, n_times) = mult_persistence(&[6, 7, 9]);
        assert_eq!(new_digits, vec![6]);
        assert_eq!(n_times, 5);
    }

    #[test]
    fn is_candidate_test() {
        assert_eq!(is_candidate(&[1, 2, 3]), false);
        assert_eq!(is_candidate(&[6, 7, 6]), false);
        assert_eq!(is_candidate(&[2, 2, 6]), false);
        assert_eq!(is_candidate(&[2, 3, 4]), false);
        assert_eq!(is_candidate(&[2, 4, 6]), false);
        assert_eq!(is_candidate(&[3, 3, 6]), false);
        assert_eq!(is_candidate(&[4, 5, 7]), false);
        assert_eq!(is_candidate(&[2, 6, 6]), true);
        assert_eq!(is_candidate(&[3, 5, 7]), true);
    }

    #[test]
    fn initial_candidate_test() {
        let candidate = initial_candidate(6);
        assert_eq!(candidate, vec![2, 6, 6, 6, 6, 6]);
    }

    #[test]
    fn next_candidate_test() {
        assert_eq!(next_candidate(&[9, 9, 9]), vec![2, 6, 6, 6]);
        assert_eq!(next_candidate(&[1, 0, 0, 0]), vec![2, 6, 6, 6]);
        assert_eq!(next_candidate(&[2, 4, 6, 8]), vec![2, 6, 6, 6]);
        assert_eq!(next_candidate(&[3, 5, 7, 9]), vec![3, 5, 9, 9]);
        assert_eq!(next_candidate(&[6, 7, 6]), vec![6, 7, 7]);
        assert_eq!(next_candidate(&[0]), vec![2, 6]);
        assert_eq!(next_candidate(&[3, 3, 3]), vec![3, 4, 4]);
    }

    #[test]
    fn fill_highest_test() {
        let mut digits = [6, 7, 6, 5];
        fill_highest(&mut digits);
        assert_eq!(digits, [6, 7, 7, 7]);
    }

    #[test]
    fn calc_slice_test() {
        assert_eq!(calc_slice(7), (vec![2, 6, 7, 7, 8, 8, 9], 0, 8));
    }
}
