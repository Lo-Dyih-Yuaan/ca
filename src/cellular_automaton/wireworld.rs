use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::FromStream;

#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Empty,
	Conductor,
	ElectronHead,
	ElectronTail
}
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Empty => write!(f, "."),
			Cell::Conductor => write!(f, "-"),
			Cell::ElectronHead => write!(f, "#"),
			Cell::ElectronTail => write!(f, "@"),
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Cell::Empty => write!(f, " ."),
			Cell::Conductor => write!(f, "\u{1B}[7m -\u{1B}[0m"),
			Cell::ElectronHead => write!(f, "\u{1B}[44m -\u{1B}[0m"),
			Cell::ElectronTail => write!(f, "\u{1B}[46m -\u{1B}[0m"),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> (usize, Option<Self>) {
		match s.chars().next() {
			Some('.') => (1, Some(Cell::Empty)),
			Some('-') => (1, Some(Cell::Conductor)),
			Some('#') => (1, Some(Cell::ElectronHead)),
			Some('@') => (1, Some(Cell::ElectronTail)),
			_ => (0, None),
		}
	}
}

#[allow(dead_code)]
pub fn rule(nw: &Cell, n: &Cell, ne: &Cell,
             w: &Cell, c: &Cell,  e: &Cell,
            sw: &Cell, s: &Cell, se: &Cell) -> Cell {
	match c {
		Cell::Empty => Cell::Empty,
		Cell::Conductor => {
			let mut head_sum: usize = 0;
			count!{$
				Cell::ElectronHead => head_sum;
				*nw,*n,*ne,*w,*e,*sw,*s,*se
			};
			if let 1|2 = head_sum {Cell::ElectronHead} else {Cell::Conductor}
		},
		Cell::ElectronHead => Cell::ElectronTail,
		Cell::ElectronTail => Cell::Conductor,
	}
}
