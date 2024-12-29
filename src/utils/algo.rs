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

type DynIter<T> = Box<dyn Iterator<Item = T>>;
type GetNeighboursFn<T, M> = Box<dyn Fn(T) -> DynIter<(T, M)>>;
type GetNeighboursRefFn<T, M, R> = Box<dyn Fn(T, &R) -> DynIter<(T, M)>>;
type GoalCheckFn<T> = Box<dyn Fn(&T) -> bool>;
type GoalCheckRefFn<T, R> = Box<dyn Fn(&T, &R) -> bool>;
type EquivKeysFn<'a, T> = Box<dyn Fn(&T) -> DynIter<T> + 'a>;
type EquivKeysRefFn<T, R> = Box<dyn Fn(&T, &R) -> DynIter<T>>;

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
