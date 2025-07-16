pub mod tree_item;

use std::collections::LinkedList;
use std::fmt;
use core::fmt::{Debug, Display};
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use crate::data::tree_item::{Alphabet, SeqSet};
use vers_vecs::{BitVec, RsVec};
use std::{cmp::Ordering, hash::Hash};


#[cfg(feature = "non_crypto_hash")]
use fxhash::FxHashMap as HashMap;
#[cfg(not(feature = "non_crypto_hash"))]
use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Character<T: PartialEq + Display + Debug + PartialOrd>{
    Char(T),
    Terminal
}

impl<T> Display for Character<T>
where
    T: PartialEq + Display + Debug + PartialOrd
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Character::Char(t) => write!(f, "{}", t),
            Character::Terminal => write!(f, ""),
        }
    }
}

impl<T> Character<T>
where
    T: PartialEq + Display + Debug + PartialOrd
{
    pub fn into_inner(&self)->Option<&T>{
        match self {
            Character::Char(x) => Some(x),
            _ => None,
        }
    }
}

impl<T> PartialOrd for Character<T>
where
    T: PartialEq + Display + Debug + PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self{
            Character::Terminal => {
                match other{
                    Character::Terminal => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
            },
            Character::Char(t) => {
                match other{
                    Character::Terminal => Some(Ordering::Greater),
                    Character::Char(o) => t.partial_cmp(o)
                }
            }
        }
        // return None;
    }
}

impl<T> Alphabet for Character<T>
where
    T: Display + Debug + PartialOrd + Hash + Eq
{
    fn is_terminal(&self)->bool{
        matches!(&self, Character::Terminal)
    }

    fn terminal()->Self {
        Character::Terminal
    }

}

impl<T> Default for Character<T>
where
    T: Display + Debug + PartialOrd + Hash + Eq
{
    fn default() -> Self {
        Self::Terminal
    }

}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum DNA{
    A = b'A',
    C = b'C',
    G = b'G',
    T = b'T',
    Terminal = b'$',
}

impl Default for DNA{
    fn default() -> Self {
        DNA::Terminal
    }
}

impl Display for DNA{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            DNA::A => write!(f, "{}", b'A'),
            DNA::C => write!(f, "{}", b'C'),
            DNA::G => write!(f, "{}", b'G'),
            DNA::T => write!(f, "{}", b'T'),
            DNA::Terminal => write!(f, ""),
        }
    }
}

impl Alphabet for DNA{
    fn is_terminal(&self)->bool {
        self==&DNA::Terminal
    }

    fn terminal()->Self {
        DNA::Terminal
    }
}


pub struct StringSet<T, U>
where
    T: Alphabet,
    U: Display + Debug + PartialEq + Hash + Eq
{
    strings: Vec<T>,
    pos_bv: RsVec,
    ids: Vec<Option<U>>,
    id_map: HashMap<U, usize>
}

impl<T, U> Default for StringSet<T,U>
where
    T: Alphabet,
    U: Display + Debug + PartialEq + Hash + Eq
{
    fn default() -> Self {
        Self{
            strings: vec![],
            pos_bv: RsVec::from_bit_vec(BitVec::new()),
            ids: vec![],
            id_map: vec![].into_iter().collect(),
        }
    }
}

impl<T, U> SeqSet for StringSet<T, U>
where
    T: Alphabet,
    U: Display + Debug + Eq + Hash + Clone + Eq
{
    type Character = T;
    type SeqID = U;

    fn from_iter(string_set_iter: impl ExactSizeIterator<Item = (Self::SeqID, Vec<Self::Character>)>)-> Self {
        let mut num_strings = 0_usize;
        let mut string_set_len = 0_usize;
        let mut strings: LinkedList<T> = LinkedList::new();
        let mut bv_positions: LinkedList<usize> = LinkedList::new();
        let mut id_map: HashMap<U, usize> = vec![].into_iter().collect();
        let mut new_string_idx = 0;
        for (n, (id, old_string)) in string_set_iter.enumerate(){
            let mut new_string: LinkedList<T> = old_string.into_iter().collect();
            new_string.push_back(T::terminal());
            bv_positions.push_back(new_string_idx);
            id_map.insert(id, n);
            num_strings += 1;
            string_set_len += new_string.len();
            new_string_idx += new_string.len();
            strings.append(&mut new_string);
        }
        let mut ids: Vec<Option<U>> = vec![None;num_strings];
        let mut bv = BitVec::from_zeros(num_strings+string_set_len);

        for pos in bv_positions{
            bv.set_unchecked(pos, 1);
        }
        for (id, idx) in id_map.iter(){
            ids[*idx] = Some(id.clone());
        }
        StringSet { strings: strings.into_iter().collect_vec(), pos_bv: RsVec::from_bit_vec(bv), ids: ids, id_map: id_map }
    }

    fn add_seq(&mut self, _id: Self::SeqID, _seq: Vec<Self::Character>) {
        todo!()
    }

    fn get_contiguous_seq(&self)->&[Self::Character] {
        self.strings.as_slice()
    }

    fn get_seq(&self, id: &Self::SeqID)->&[Self::Character] {
        let idx = self.id_map.get(id).cloned().unwrap();
        let string_start = self.pos_bv.select1(idx);
        let string_end = self.pos_bv.select1(idx+1)-1;
        &self.strings[string_start..string_end]
    }

    fn get_seq_id(&self, idx: usize)->&Self::SeqID {
        self.ids[idx].as_ref().unwrap()
    }

    fn get_seq_start_idx(&self, id: &Self::SeqID)->usize {
        *self.id_map.get(id).unwrap()
    }
}