/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::mem::swap;
use std::process::id;

pub struct Heap<T: Clone>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::clone::Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count+=1;
        self.upper(self.count);
    }
    fn upper(&mut self,idx: usize){
        if &self.parent_idx(idx)>=&1&&!(self.comparator)(&self.items[self.parent_idx(idx)],&self.items[idx]){
            let p=self.parent_idx(idx).clone();
            //swap(&mut self.items[p], &mut self.items[idx]);
            let tp:T=self.items.get(p).unwrap().clone();
            self.items[p]=self.items[idx].clone();
            self.items[idx]=tp.clone();

            println!("change: idx:{} p:{} ",idx,self.parent_idx(idx));
            //println!("after change:idx:{} ,p:{}",self.items[idx].clone(),self.items[p].clone());
            self.upper(self.parent_idx(idx));
        }
    }
    fn down(&mut self,idx:usize){
        let l=self.left_child_idx(idx);
        let r=self.right_child_idx(idx);
        let mut to=idx.clone();
        if l<=self.count&&(self.comparator)(&self.items[l],&self.items[to]){
            to=l.clone();
        }
        if r<=self.count&&(self.comparator)(&self.items[r],&self.items[to]){
            to=r.clone();
        }
        if to!=idx{
            //swap(&mut self.items[to],&mut self.items[idx]);
            let tp=self.items.get(to).unwrap().clone();
            self.items[to]=self.items.get(idx).unwrap().clone();
            self.items[idx]=tp;
            self.down(to);
        }
    }
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::clone::Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + std::clone::Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            return None;
        }
        let t=self.items[1].clone();
        self.items[1]=self.items[self.count].clone();
        self.items.remove(self.count);
        self.count-=1;
        self.down(1);
        Some(t.clone())
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}