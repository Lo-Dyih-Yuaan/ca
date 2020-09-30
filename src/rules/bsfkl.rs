use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;
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
pub fn rule(bs: &[usize], ss: &[usize], fs: &[usize], ks: &[usize], ls: &[usize]) -> BoxRule<Cell> {
	let bs = bs.to_owned();
	let ss = ss.to_owned();
	let fs = fs.to_owned();
	let ks = ks.to_owned();
	let ls = ls.to_owned();
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let [live_count, destructive_count] =
			count!{Live, Destructive in nw,n,ne,w,e,sw,s,se};
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