
#[inline] fn ti_to_si(ti: usize) -> usize { (ti - 1) / 2 }
#[inline] fn si_to_ti(si: usize) -> usize { 2 * si + 1 }

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ManacherValue<T> { Sep, Char(T) }

struct ManacherString<'a, T>(&'a [T]);

impl<'a, T: Eq + Copy> ManacherString<'a, T> {
  fn new(s: &'a [T]) -> ManacherString<'a, T> {
    ManacherString(s)
  }
  fn len(&self) -> usize {
    self.0.len() * 2 + 1
  }
  fn get(&self, ti: usize) -> ManacherValue<T> {
    assert!(ti < self.len());
    if ti % 2 == 0 {
      ManacherValue::Sep
    } else {
      ManacherValue::Char(self.0[ti_to_si(ti)])
    }
  }
  fn is_equal(&self, ti: usize, tj: usize) -> bool {
    self.get(ti) == self.get(tj)
  }
}

pub struct ManacherIter<'a> {
  data: &'a Manacher,
  i: usize,
  len: usize,
}

use std::ops::Range;

impl<'a> Iterator for ManacherIter<'a> {
  type Item = Range<usize>;
  fn next(&mut self) -> Option<Self::Item> {
    while self.i < self.data.0.len() {
      let (ti, rad) = (self.i, self.data.0[self.i]);
      self.i += 1;
      if rad >= self.len && rad % 2 == self.len % 2 {
        let start_si = ti_to_si(ti + 1 - self.len);
        return Some(start_si..start_si + self.len);
      }
    }
    None
  }
}

#[derive(Debug)]
pub struct Manacher(Vec<usize>);

impl Manacher {
  pub fn new<T: Eq + Copy>(s: &[T]) -> Manacher {
    let t = ManacherString::new(s); // adding separators, e.g. "abc" => "#a#b#c#"
    let t_len = t.len();
    let mut ans = vec![0]; // ans[i] is max radius centered at i
    // e.g. xxxxixxxx => 4, xix => 1, i => 0
    let mut center = 0;
    let mut right = 0;
    // left = center * 2 - right;
    // [left <- center -> right] the largest mirror we can use
    // the first character is '#', so we have valid mirror [0, 0]
    for i in 1..t_len {
      let mut delta = if i >= right {
        1 // we can not use previous calculations
      } else {
        let i_mirror = 2 * center - i;
        ans[i_mirror].min(right - i) // the valid part cut out by right boundary
      };
      while i + delta < t_len && delta <= i && t.is_equal(i + delta, i - delta) {
        delta += 1;
      }
      let radius = delta - 1; // e.g. axxxixxxb, 'xxx~' -> delta, 'xxx' -> radius
      ans.push(radius);
      let new_right = i + radius;
      if new_right > right {
        right = new_right;
        center = i;
      }
    }
    Manacher(ans)
  }

  pub fn max_palindrome_len(&self) -> usize {
    *self.0.iter().max().unwrap()
  }

  pub fn iter_of_len(&self, len: usize) -> ManacherIter<'_> {
    ManacherIter {
      data: self,
      i: 0,
      len,
    }
  }

  pub fn iter_of_max(&self) -> ManacherIter<'_> {
    ManacherIter {
      data: self,
      i: 0,
      len: self.max_palindrome_len(),
    }
  }

  pub fn odd_longest_at(&self, si: usize) -> Range<usize> {
    // ? ? si ? ?
    // si is the original string index
    let longest_len = self.0[si_to_ti(si)];
    // #a#b# i #b#a# => total count is (2*rad + 1) has (rad+1) '#'s, rad chars
    // longest_len must be odd length
    let rad = longest_len / 2;
    si - rad..si + rad + 1
  }

  pub fn even_longest_at(&self, si: usize, next_si: usize) -> Range<usize> {
    assert_eq!(si + 1, next_si);
    let ti = si_to_ti(si) + 1;
    assert!(ti < self.0.len()); // otherwise (i+1) is not in s
    // ? ? i (i+1) ? ?
    // # ? # ? # i # (i+1) # ? # ? #
    // #a#b # b#a# also has (rad+1) '#'s, rad chars
    let longest_len = self.0[ti]; // longest_len must be even
    let rad = longest_len / 2;
    next_si - rad..next_si + rad
  }

  pub fn is_palindrome(&self, sl: usize, sr: usize) -> bool {
    // [l, r)
    if sl >= sr { return true; }
    let slen = sr - sl;
    let range = if slen % 2 == 0 { // even palindrome
      let center = sl + slen / 2 - 1;
      self.even_longest_at(center, center + 1)
    } else { // odd palindrome
      let center = sl + slen / 2;
      self.odd_longest_at(center)
    };
    range.end - range.start >= slen
  }
}

fn naive_palindrome<T: Eq>(s: &[T], l: usize, r: usize) -> bool {
  // [l, r)
  for d in 0.. {
    if 2 * d + 1 + l >= r {
      break;
    }
    let i = l + d;
    let j = r - 1 - d;
    // i < j => l + d < r - 1 - d => 2 * d + 1 + l < r
    if s[i] != s[j] { return false; }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  fn match_naive<T: Eq>(s: &[T], m: &Manacher) -> bool {
    for l in 0..=s.len() {
      for r in 0..=s.len() {
        if naive_palindrome(s, l, r) != m.is_palindrome(l, r) {
          return false;
        }
      }
    }
    true
  }

  #[test]
  fn test_ascii() {
    let s = "bananas";
    let m = Manacher::new(s.as_bytes());

    assert_eq!(5, m.max_palindrome_len());

    let mut iter_max = m.iter_of_max();
    assert_eq!(Some(1..6), iter_max.next());
    assert_eq!(None, iter_max.next());

    assert!(!m.is_palindrome(0, 2));
    assert!(m.is_palindrome(1, 4));

    assert!(match_naive(s.as_bytes(), &m));
  }

  #[test]
  fn test_ascii_2() {
    let s = "abracadabra";
    let m = Manacher::new(s.as_bytes());

    assert_eq!(3, m.max_palindrome_len());

    let mut iter_max = m.iter_of_max();
    assert_eq!(Some(3..6), iter_max.next());
    assert_eq!(Some(5..8), iter_max.next());

    assert!(match_naive(s.as_bytes(), &m));
  }

  #[test]
  fn test_utf8() {
    let s: Vec<char> = "上海自来水来自海上A中山诸罗茶罗诸山中B山东落花生花落东山C花莲喷水池水喷莲花".chars().collect();
    let m = Manacher::new(&s);
    assert_eq!(9, m.max_palindrome_len());

    let mut iter_max = m.iter_of_max();
    assert_eq!(Some(0..9), iter_max.next());
    assert_eq!(Some(10..19), iter_max.next());
    assert_eq!(Some(20..29), iter_max.next());
    assert_eq!(Some(30..39), iter_max.next());
    assert_eq!(None, iter_max.next());

    assert!(match_naive(&s, &m));
  }
}