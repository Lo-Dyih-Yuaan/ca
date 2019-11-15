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
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Live => write!(f, "#"),
			Cell::Dead => write!(f, "."),
			Cell::Destructive => write!(f, "@"),
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Live => write!(f, "\u{1b}[7m -\u{1b}[0m"),
			Cell::Dead => write!(f, " -"),
			Cell::Destructive => write!(f, "\u{1b}[42m @\u{1b}[0m"),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> (usize, Option<Self>) {
		match s.chars().next() {
			Some('#') => (1, Some(Cell::Live)),
			Some('.') => (1, Some(Cell::Dead)),
			Some('@') => (1, Some(Cell::Destructive)),
			_ => (0, None),
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
			Cell::Live => live_count,
			Cell::Destructive => destructive_count;
			*nw,*n,*ne,*w,*e,*sw,*s,*se
		};
		match c {
			Cell::Live =>
				if ks.contains(&destructive_count) {
					Cell::Dead
				} else if ss.contains(&live_count) {
					Cell::Live
				} else {Cell::Destructive},
			Cell::Dead =>
				if bs.contains(&live_count) && fs.contains(&destructive_count) {
					Cell::Live
				} else {Cell::Dead},
			Cell::Destructive =>
				if ls.contains(&live_count) {Cell::Dead} else {Cell::Destructive}
		}
	})
}