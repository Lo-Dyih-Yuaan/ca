use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::FromStream;
use super::BoxRule;
// BSFKL
// 
#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Live,
	Dead,
	Destructive
}
use self::Cell::*;
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "#"),
			Dead => write!(f, "."),
			Destructive => write!(f, "@"),
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Dead => write!(f, " -"),
			Destructive => write!(f, "\u{1b}[42m @\u{1b}[0m"),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('#') => Some((1, Live)),
			Some('.') => Some((1, Dead)),
			Some('@') => Some((1, Destructive)),
			_ => None
		}
	}
}

#[allow(dead_code)]
pub fn rule(bs: &'static[usize], ss: &'static[usize],
  fs: &'static[usize], ks: &'static[usize], ls: &'static[usize]) -> BoxRule<Cell> {
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let mut live_count: usize = 0;
		let mut destructive_count: usize = 0;
		count!{$
			Live => live_count,
			Destructive => destructive_count;
			*nw,*n,*ne,*w,*e,*sw,*s,*se
		};
		match c {
			Live =>
				if ks.contains(&destructive_count) {Dead}
				else if ss.contains(&live_count) {Live}
				else {Destructive},
			Dead =>
				if bs.contains(&live_count) && fs.contains(&destructive_count) {Live}
				else {Dead},
			Destructive =>
				if ls.contains(&live_count) {Dead} else {Destructive}
		}
	})
}