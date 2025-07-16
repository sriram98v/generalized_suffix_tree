use core::fmt::{Debug, Display};
use std::hash::Hash;

pub trait Alphabet: Display + Debug + Hash + Eq + Default
{
    /// Checks if self is a terminal character
    fn is_terminal(&self)->bool;

    /// Returns a terminal character
    fn terminal()->Self;
}

pub trait SeqSet: Default
where
    Self::Character: Alphabet,
    Self::SeqID: Display + Debug + Eq + Hash + Clone,
{
    type Character;
    type SeqID;

    /// Creates a SeqSet from an iterator
    fn from_iter(string_set_iter: impl ExactSizeIterator<Item = (Self::SeqID, Vec<Self::Character>)>) -> Self;

    /// Creates a SeqSet from an object that can be turned into an iterator
    fn from_set<T:ExactSizeIterator<Item = (Self::SeqID, Vec<Self::Character>)>>(string_set: impl Into<T>) -> Self
    {
        Self::from_iter(string_set.into())
    }

    /// Returns sequence slice by ID
    fn get_seq(&self, id: &Self::SeqID)->&[Self::Character];

    /// Get concatenated sequence. The concatenated sequence is all the sequences present in SeqSet as a contiguous sequence of elements, separated by terminal characters.
    fn get_contiguous_seq(&self)->&[Self::Character];

    /// Returns start index of sequence by ID in contiguous
    fn get_seq_start_idx(&self, id: &Self::SeqID)->usize;

    /// Returns the SeqID of string from index on contiguous segment
    fn get_seq_id(&self, idx: usize)->&Self::SeqID;

    /// Adds new sequence and seqID to set
    fn add_seq(&mut self, id: Self::SeqID, seq: Vec<Self::Character>);
}