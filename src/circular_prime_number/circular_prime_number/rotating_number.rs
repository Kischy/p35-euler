mod digits;

pub struct RotatingNumber {
    num: u128,
    digits: Vec<u32>,
    front_index: usize,
}

impl RotatingNumber {
    pub fn new(num: u128) -> RotatingNumber {
        RotatingNumber {
            num: num,
            digits: digits::get_digits(num),
            front_index: 0,
        }
    }
}

impl Iterator for RotatingNumber {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_index >= self.digits.len() {
            return None;
        }

        if self.front_index == 0 {
            self.front_index += 1;
            return Some(self.num);
        }

        let mut new_digits = Vec::with_capacity(self.digits.len());
        new_digits.extend_from_slice(&self.digits[self.front_index..]);
        new_digits.extend_from_slice(&self.digits[0..self.front_index]);

        self.front_index += 1;
        Some(digits::get_number(&new_digits))
    }
}

#[cfg(test)]
mod tests {
    use super::RotatingNumber;

    #[test]
    fn iterator_rotating_number_1_test() {
        let mut rot_num = RotatingNumber::new(1);
        assert_eq!(rot_num.next(), Some(1));
        assert_eq!(rot_num.next(), None);
    }

    #[test]
    fn iterator_rotating_number_12_test() {
        let mut rot_num = RotatingNumber::new(12);
        assert_eq!(rot_num.next(), Some(12));
        assert_eq!(rot_num.next(), Some(21));
        assert_eq!(rot_num.next(), None);
    }

    #[test]
    fn iterator_rotating_number_719_test() {
        let mut rot_num = RotatingNumber::new(719);
        assert_eq!(rot_num.next(), Some(719));
        assert_eq!(rot_num.next(), Some(197));
        assert_eq!(rot_num.next(), Some(971));
        assert_eq!(rot_num.next(), None);
    }

    #[test]
    fn iterator_rotating_number_11_test() {
        let mut rot_num = RotatingNumber::new(1111);
        assert_eq!(rot_num.next(), Some(1111));
        assert_eq!(rot_num.next(), Some(1111));
        assert_eq!(rot_num.next(), Some(1111));
        assert_eq!(rot_num.next(), Some(1111));
        assert_eq!(rot_num.next(), None)
    }

    #[test]
    fn iterator_rotating_number_71992_test() {
        let mut rot_num = RotatingNumber::new(71992);
        assert_eq!(rot_num.next(), Some(71992));
        assert_eq!(rot_num.next(), Some(19927));
        assert_eq!(rot_num.next(), Some(99271));
        assert_eq!(rot_num.next(), Some(92719));
        assert_eq!(rot_num.next(), Some(27199));
        assert_eq!(rot_num.next(), None);
    }
}
