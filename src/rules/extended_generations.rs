use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;
use super::tools::u32_print_str;
use super::BoxRule;

#[derive(PartialEq, Eq, Clone)]
pub enum Cell {
	Dead,
	Live(usize)
}
use self::Cell::*;
impl Debug for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Dead => write!(f, "."),
			Live(n) =>
				write!(f, "{}", u32_print_str(*n as u32))
		}
	}
}
impl Display for Cell {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Dead => write!(f, " -"),
			Live(n) =>
				write!(f, "\u{1b}[48;5;8m{:>2}\u{1b}[0m", u32_print_str(*n as u32)),
		}
	}
}
impl FromStream for Cell {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('.') => Some((1, Dead)),
			_ => match u32::from_stream(s) {
				Some((len, n)) => Some((len, Live(n as usize))),
				None => None,
			}
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Action {
	Active,
	Inactive
}
use self::Action::*;
impl std::ops::Not for Action {
	type Output = Action;
	#[inline(always)]
	fn not(self) -> Self::Output {
		match self {
			Active => Inactive,
			Inactive => Active,
		}
	}
}

#[allow(dead_code)]
pub fn rule(action: &[usize], birth: &'static[usize], save: &'static[usize]) -> BoxRule<Cell> {
	let action = {
		let mut now = Active;
		let mut temp = Vec::<Action>::new();
		for n in action.iter() {
			for _ in 0..*n {
				temp.push(now);
			}
			now = !now;
		}
		temp
	};
	if action.is_empty() {
		unreachable!();
	}
	macro_rules! trans {
		($c:expr) => {
			if let Live(n) = $c {action[*n]} else {Inactive}
		};
	}
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let nw = trans!(nw);
		let  n = trans!( n);
		let ne = trans!(ne);
		let  w = trans!( w);
		let  e = trans!( e);
		let sw = trans!(sw);
		let  s = trans!( s);
		let se = trans!(se);
		let [out_sum] = count!{Active in nw,n,ne,w,e,sw,s,se};
		match c {
			Dead => if birth.contains(&out_sum) {Live(0)} else {Dead},
			Live(n) =>
				if action[*n] == Active && save.contains(&out_sum) {Live(*n)}
				else if *n+1 < action.len() {Live(*n+1)}
				else {Dead}
		}
	})
}

#[allow(unreachable_patterns, dead_code)]
pub fn non_totalistic_rule(action: &[usize], birth: &'static str, save: &'static str) -> BoxRule<Cell> {
	let action = {
		let mut now = Active;
		let mut temp = Vec::<Action>::new();
		for n in action.iter() {
			for _ in 0..*n {
				temp.push(now);
			}
			now = !now;
		}
		temp
	};
	if action.is_empty() {
		unreachable!();
	}
	macro_rules! trans {
		($c:expr) => {
			if let Live(n) = $c {action[*n]} else {Inactive}
		};
	}
	let b_fun = non_totalistic_closure!(Action; Active, birth);
	let s_fun = non_totalistic_closure!(Action; Active, save);
	Box::new(move |nw: &Cell, n: &Cell, ne: &Cell,
	                w: &Cell, c: &Cell,  e: &Cell,
	               sw: &Cell, s: &Cell, se: &Cell| -> Cell {
		let nw = &trans!(nw);
		let  n = &trans!( n);
		let ne = &trans!(ne);
		let  w = &trans!( w);
		let  e = &trans!( e);
		let sw = &trans!(sw);
		let  s = &trans!( s);
		let se = &trans!(se);
		match c {
			Dead => if b_fun(nw,n,ne,w,e,sw,s,se) {Live(0)} else {Dead},
			Live(i) =>
				if action[*i] == Active && s_fun(nw,n,ne,w,e,sw,s,se) {Live(*i)}
				else if *i+1 < action.len() {Live(*i+1)}
				else {Dead}
		}
	})
}