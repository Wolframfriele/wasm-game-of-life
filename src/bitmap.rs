use std::{
    fmt,
    ops::{BitAndAssign, BitXorAssign, BitOrAssign},
};

#[derive(Clone, Default)]
pub struct Bitset {
    store: u64,
}

impl Bitset {
    pub fn new() -> Bitset {
        Bitset { store: 0 }
    }

    pub fn set(&mut self, idx: u8) {
        if idx > 63 {
            panic!("Index can't be higher than 63")
        }
        self.store.bitor_assign(1 << (63 - idx));
    }

    pub fn reset(&mut self, idx: u8) {
        if idx > 63 {
            panic!("Index can't be higher than 63")
        }
        self.store.bitand_assign(!(1 << (63 - idx)));
    }

    pub fn flip(&mut self, idx: u8) {
        if idx > 63 {
            panic!("Index can't be higher than 63")
        }
        self.store.bitxor_assign(1 << (63 - idx));
    }

    pub fn get(&self, idx: u8) -> u8 {
        if idx > 63 {
            panic!("Index can't be higher than 63")
        }
        if self.store & 1 << (63 - idx) == 1 << (63 - idx) {
            1
        } else {
            0
        }
    }
}

impl fmt::Display for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:064b}", &self.store)
    }
}

#[derive(Clone, Default)]
pub struct Bitmap {
    data: Vec<Bitset>,
}

impl Bitmap {
    pub fn new() -> Bitmap {
        Bitmap { data: Vec::from_iter(std::iter::repeat(Bitset::new()).take(64)) }
    }

    pub fn set(&mut self, x: u8, y: u8) {
        self.data[y as usize].set(x);
    }

    pub fn get(&mut self, x: u8, y: u8) -> u8{
        self.data[y as usize].get(x)
    }

    pub fn flip(&mut self, x: u8, y: u8) {
        self.data[y as usize].flip(x);
    }

    pub fn as_slice(&self) -> &[Bitset]{
        self.data.as_slice()
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.data {
            writeln!(f, "{:064b}", &line.store)?;
        }
        Ok(())
    }
}
