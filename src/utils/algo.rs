use std::{
    borrow::Borrow,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::Add,
};

pub use bfs::*;
pub use dfs::*;
pub use dijkstra::*;
mod bfs;
mod dfs;
mod dijkstra;

type DynIter<'a, T: 'a> = Box<dyn Iterator<Item = T> + 'a>;
type GetNeighboursFn<'a, T: 'a, M> = Box<dyn Fn(T) -> DynIter<'a, (T, M)> + 'a>;
type GoalCheckFn<'a, T> = Box<dyn Fn(&T) -> bool + 'a>;
type EquivKeysFn<'a, T> = Box<dyn Fn(&T) -> DynIter<'a, T> + 'a>;

pub trait Tracking<M: Clone> {
    type Output: Default + Clone;
    fn push(o1: &Self::Output, m: M) -> Self::Output;
    fn len(o: &Self::Output) -> usize;
}

pub struct WithHistory;

pub struct WithDistance;

impl<M: Clone> Tracking<M> for WithHistory {
    type Output = Vec<M>;
    fn push(o1: &Self::Output, m: M) -> Self::Output {
        let mut o1 = o1.clone();
        o1.push(m);
        o1
    }
    fn len(o: &Self::Output) -> usize {
        o.len()
    }
}
impl<M: Clone> Tracking<M> for WithDistance {
    type Output = usize;
    fn push(o1: &Self::Output, m: M) -> Self::Output {
        o1 + 1
    }
    fn len(o: &Self::Output) -> usize {
        *o
    }
}
pub struct StateWithRefdata<'a, T, R: ?Sized> {
    pub state: T,
    pub refdata: &'a R,
}
impl<'a, T, R: ?Sized> StateWithRefdata<'a, T, R> {
    pub fn new(state: T, refdata: &'a R) -> Self {
        Self { state, refdata }
    }
}
impl<'a, T, R> Eq for StateWithRefdata<'a, T, R> where T: Eq {}
impl<'a, T, R> PartialEq for StateWithRefdata<'a, T, R>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}
impl<'a, T, R> Hash for StateWithRefdata<'a, T, R>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}
impl<'a, T, R> Clone for StateWithRefdata<'a, T, R>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            refdata: self.refdata,
        }
    }
}
impl<'a, T, R> Debug for StateWithRefdata<'a, T, R>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state.fmt(f)
    }
}
