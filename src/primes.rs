pub struct PrimeIter {
    current: u32,
    table: Vec<u32>,
}

impl Default for PrimeIter {
    fn default() -> PrimeIter {
        PrimeIter {
            current: 2,
            table: Vec::new(),
        }
    }
}

impl PrimeIter {
    pub fn new() -> PrimeIter {
        Self::default()
    }
}

trait Incremental
where
    Self: Clone + std::ops::AddAssign + From<u8>,
{
    fn increment(&mut self) -> Self {
        let temp = self.clone();
        *self += 1u8.into();
        temp
    }
}

impl<T: Clone + std::ops::AddAssign + From<u8>> Incremental for T {}

impl Iterator for PrimeIter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut limit = (self.current as f64).sqrt() as u32;
        while !self
            .table
            .iter()
            .take_while(|&&x| x <= limit)
            .all(|&x: &u32| -> bool { self.current % x != 0 })
        {
            self.current += 1;
            limit = (self.current as f64).sqrt() as u32;
        }
        self.table.push(self.current);
        Some(self.current.increment())
    }
}

#[cfg(test)]
#[test]
fn test() {
    use self::PrimeIter;
    assert_eq!(
        PrimeIter::new().take(10).collect::<Vec<_>>(),
        [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    );
}
