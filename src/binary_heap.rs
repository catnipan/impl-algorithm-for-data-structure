
use std::cmp::Ordering;

struct BinaryHeap<T, I> {
  data: Vec<T>,
  comparator: I
}

#[inline] fn parent(i: usize) -> usize {  (i - 1) / 2 }
#[inline] fn left_child(i: usize) -> usize { i * 2 + 1 }

type DefaultCmp<T> = fn(&T, &T) -> Ordering;
impl<T: Ord> BinaryHeap<T, DefaultCmp<T>> {
  fn comparator(a: &T, b: &T) -> Ordering {
    a.cmp(b)
  }
  fn from(data: Vec<T>) -> BinaryHeap<T, DefaultCmp<T>> {
    let mut ans = BinaryHeap {
      data,
      comparator: Self::comparator as DefaultCmp<T>,
    };
    ans.build_heap();
    ans
  }
  fn new() -> BinaryHeap<T, DefaultCmp<T>> {
    fn comparator<T: Ord>(a: &T, b: &T) -> Ordering {
      a.cmp(b)
    }
    BinaryHeap {
      data: vec![],
      comparator: Self::comparator as DefaultCmp<T>,
    }
  }
}

impl<T, I> BinaryHeap<T, I> where I: FnMut(&T, &T) -> Ordering {
    fn with_comparator(comparator: I) -> BinaryHeap<T, I> {
      BinaryHeap {
        data: vec![],
        comparator,
      }
    }

    fn from_with_comparator(data: Vec<T>, comparator: I) -> BinaryHeap<T, I> {
      let mut ans = BinaryHeap {
        data,
        comparator,
      };
      ans.build_heap();
      ans
    }

    fn is_empty(&self) -> bool {
      self.data.is_empty()
    }

    fn len(&self) -> usize {
      self.data.len()
    }

    fn build_heap(&mut self) {
      if self.len() < 2 { return; }
      // if x is last internal index, then 2x+1 == self.len() - 1
      // x = (self.len() - 2) / 2;
      let last_internal = (self.len() - 2) / 2;
      for i in (0..=last_internal).rev() {
        self.sift_down(i);
      }
    }

    #[inline]
    fn is_less(&mut self, i: usize, j: usize) -> bool {
      (self.comparator)(&self.data[i], &self.data[j]) == Ordering::Less
    }

    fn sift_up(&mut self, mut i: usize) {
      // if a value > its parent
      while i != 0 {
        let parent_i = parent(i);
        if self.is_less(parent_i, i) {
          self.data.swap(parent_i, i);
          i = parent_i;
        } else {
          break;
        }
      }
    }

    fn sift_down(&mut self, mut i: usize) {
      // if a value < max of its child
      loop {
        let lc_i = left_child(i);
        if !self.contains_idx(lc_i) { break; } // has no left child
        let mut max_idx = if self.is_less(i, lc_i) { lc_i } else { i };
        let rc_i = lc_i + 1;
        if self.contains_idx(rc_i) && self.is_less(max_idx, rc_i) {
          max_idx = rc_i;
        }
        if max_idx == i { break; } // value >= max of its child
        self.data.swap(i, max_idx);
        i = max_idx;
      }
    }

    #[inline]
    fn contains_idx(&self, i: usize) -> bool {
      i < self.data.len()
    }

    fn push(&mut self, v: T) {
      self.data.push(v);
      self.sift_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
      if self.data.is_empty() {
        return None;
      }
      let ans = self.data.swap_remove(0);
      self.sift_down(0);
      Some(ans)
    }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_basic_pq() {
    let mut pq: BinaryHeap<i32, _> = BinaryHeap::from_with_comparator(
      vec![2,1,6,3,9,7,4,8,5],
      |a, b| a.cmp(b)
    );
    for i in (1..=9).rev() {
      assert_eq!(Some(i), pq.pop());
    }
    assert_eq!(None, pq.pop());
  }

  #[test]
  fn test_default_cmp() {
    let mut pq: BinaryHeap<i32, _> = BinaryHeap::from(vec![2,1,6,3,9,7,4,8,5]);
    for i in (1..=9).rev() {
      assert_eq!(Some(i), pq.pop());
    }
    assert_eq!(None, pq.pop());
  }
}