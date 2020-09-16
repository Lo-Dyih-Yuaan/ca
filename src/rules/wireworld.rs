use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;

#[derive(PartialEq, Eq, Clone)]
pub enum State {
	Empty,
	Conductor,
	ElectronHead,
	ElectronTail
}
use self::State::*;
impl Debug for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Empty => write!(f, "."),
			Conductor => write!(f, "-"),
			ElectronHead => write!(f, "#"),
			ElectronTail => write!(f, "@"),
		}
	}
}
impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Empty => write!(f, " ."),
			Conductor => write!(f, "\u{1B}[7m -\u{1B}[0m"),
			ElectronHead => write!(f, "\u{1B}[44m -\u{1B}[0m"),
			ElectronTail => write!(f, "\u{1B}[46m -\u{1B}[0m"),
		}
	}
}
impl FromStream for State {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('.') => Some((1, Empty)),
			Some('-') => Some((1, Conductor)),
			Some('#') => Some((1, ElectronHead)),
			Some('@') => Some((1, ElectronTail)),
			_ => None,
		}
	}
}

#[allow(dead_code)]
pub fn rule(nw: &State, n: &State, ne: &State,
             w: &State, c: &State,  e: &State,
            sw: &State, s: &State, se: &State) -> State {
	match c {
		Empty => Empty,
		Conductor => {
			let [head_sum] = count!{ElectronHead in nw,n,ne,w,e,sw,s,se};
			if let 1|2 = head_sum {ElectronHead} else {Conductor}
		},
		ElectronHead => ElectronTail,
		ElectronTail => Conductor,
	}
}
