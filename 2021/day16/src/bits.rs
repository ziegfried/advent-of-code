#[derive(Debug)]
pub struct Bits {
    idx: usize,
    data: Vec<bool>,
}

impl Bits {
    pub fn new(data: Vec<bool>) -> Self {
        Bits { idx: 0, data }
    }
    pub fn from_hex_str(hex_str: &str) -> Self {
        Self::new(
            hex_str
                .chars()
                .map(|c| c.to_digit(16).unwrap())
                .map(|c| format!("{:04b}", c))
                .collect::<String>()
                .chars()
                .map(|c| match c.to_digit(2).unwrap() {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                })
                .collect(),
        )
    }
    pub fn take(&mut self, n: usize) -> &[bool] {
        if n > self.len() {
            panic!(
                "take({}) out of bounds with len={} remaining",
                n,
                self.len()
            );
        }
        let start = self.idx;
        self.idx += n;
        &self.data[start..(start + n)]
    }
    pub fn len(&self) -> usize {
        self.data.len() - self.idx
    }
    pub fn empty(&self) -> bool {
        let len = self.len();
        len <= 8 && (0..len).all(|i| self.data[self.idx + i] == false)
    }
}

#[test]
fn test_bits_empty() {
    assert_eq!(Bits::new(vec![]).empty(), true);
    assert_eq!(Bits::new(vec![false, false, false]).empty(), true);
    assert_eq!(
        Bits::new(vec![false, false, false, false, false, false, false]).empty(),
        true
    );
    let mut bits = Bits::from_hex_str("C0FF88");
    bits.take(21);
    assert_eq!(bits.empty(), true);
}

#[test]
fn test_bits_take() {
    let mut bits = Bits::new(vec![true, true, true, false, false, false]);
    assert_eq!(bits.take(3), vec![true, true, true]);
    assert_eq!(bits.take(3), vec![false, false, false]);
}
