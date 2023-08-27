use std::prelude::v1::*;

use super::Error;

use lazy_static::lazy_static;
use std::io::Write;

lazy_static! {
    static ref ZERO_BYTES: [u8; 32] = [0_u8; 32];
    static ref ZERO_HASHES: [[u8; 32]; 65] = {
        let mut hashes = [[0_u8; 32]; 65];
        let mut tmp = [0_u8; 64];
        for i in 0usize..64usize {
            tmp[..32].copy_from_slice(&hashes[i][..]);
            tmp[32..].copy_from_slice(&hashes[i][..]);
            hashes[i + 1] = sha256_sum(&tmp[..]);
        }
        hashes
    };
}

pub trait HashTree {
    // GetTree() (*Node, error)
    // HashTreeRoot() ([32]byte, error)
    fn hash_tree_root_with(&self, h: &mut Hasher) -> Result<(), Error>;
}

fn sha256_sum(data: &[u8]) -> [u8; 32] {
    blst::sha256_sum(data)
}

#[derive(Default)]
pub struct Hasher {
    buf: Vec<u8>,
    // tmp: Vec<u8>,
}

pub fn hash_root<T: HashTree>(t: &T) -> Result<[u8; 32], Error> {
    let mut hasher = Hasher::new();
    t.hash_tree_root_with(&mut hasher)?;
    hasher.merkleize(0);
    hasher.hash_root()
}

impl Hasher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hash_root(&self) -> Result<[u8; 32], Error> {
        if self.buf.len() != 32 {
            return Err(Error::IncorrectSize);
        }
        let mut val = [0_u8; 32];
        val.copy_from_slice(&self.buf);
        Ok(val)
    }

    pub fn index(&self) -> usize {
        self.buf.len()
    }

    pub fn hash(&self) -> Vec<u8> {
        return self.buf[self.buf.len() - 32..].into();
    }

    fn do_hash(a: &[u8], b: &[u8]) -> [u8; 32] {
        sha256_sum(&[a, b].concat())
    }

    fn next_power_of_two(mut v: usize) -> usize {
        v -= 1;
        v |= v >> 1;
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;
        v += 1;
        v
    }

    fn get_depth(d: usize) -> u8 {
        if d <= 1 {
            return 0;
        }
        let i = Self::next_power_of_two(d);
        return 64 - i.leading_zeros() as u8 - 1;
    }

    fn merkleize(&mut self, indx: usize) {
        let input = &self.buf[indx..];
        // glog::info!("merkleize[{}..]: {:?}(len:{})", indx, input, input.len());
        let mut dst = Vec::from(&self.buf[..indx]);

        // merkleize the input
        Self::merkleize_impl(&mut dst, input, 0);
        self.buf = dst;
    }

    pub fn append_bytes32(&mut self, b: &[u8]) {
        self.buf.write(b).unwrap();
        let rest = b.len() % 32;
        if rest != 0 {
            // pad zero bytes to the left
            self.buf.write(&ZERO_BYTES[..32 - rest]).unwrap();
        }
    }

    pub fn put_u64(&mut self, val: u64) {
        self.append_bytes32(&val.to_le_bytes())
    }

    pub fn put_bytes(&mut self, b: &[u8]) {
        if b.len() <= 32 {
            self.append_bytes32(b);
            return;
        }

        // if the bytes are longer than 32 we have to
        // merkleize the content
        let indx = self.index();
        self.append_bytes32(b);
        self.merkleize(indx);
    }

    fn merkleize_impl<'a>(dst: &mut Vec<u8>, input: &[u8], mut limit: usize) {
        let count = input.len() / 32;
        if limit == 0 {
            limit = count
        } else if count > limit {
            panic!("BUG: count '{}' higher than limit '{}'", count, limit);
        }
        if limit == 0 {
            dst.write(&ZERO_BYTES[..]).unwrap();
            return;
        }
        if limit == 1 {
            if count == 1 {
                dst.write(&input[..32]).unwrap();
                return;
            }
            dst.write(&ZERO_BYTES[..]).unwrap();
            return;
        }

        let depth = Self::get_depth(limit);
        if input.len() == 0 {
            dst.write(&ZERO_HASHES[depth as usize]).unwrap();
            return;
        }

        let mut input = Vec::from(input);
        let get_pos = |i: usize| -> std::ops::Range<usize> { i * 32..i * 32 + 32 };

        for i in 0..depth {
            let mut layer_len = input.len() / 32;
            let odd_node_length = layer_len % 2 == 1;

            if odd_node_length {
                // is odd length
                input.write(&ZERO_HASHES[i as usize][..]).unwrap();
                layer_len += 1;
            }

            let output_len = (layer_len / 2) * 32;
            for i in (0..layer_len).step_by(2) {
                let result = Self::do_hash(&input[get_pos(i)], &input[get_pos(i + 1)]);
                input[get_pos(i / 2)].copy_from_slice(&result);
            }
            input.resize(output_len, 0);
        }

        dst.write(&input).unwrap();
    }
}
