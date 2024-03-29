use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;
use super::BoxRule;

#[derive(PartialEq, Eq, Clone)]
pub enum State {
	Empty,
	Wire,
	Head(bool),
	Tail
}
use self::State::*;

impl Debug for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Empty => write!(f, "."),
			Wire => write!(f, "-"),
			Head(true) => write!(f, "1"),
			Head(false) => write!(f, "0"),
			Tail => write!(f, "@"),
		}
	}
}
impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			Empty => write!(f, " ."),
			Wire => write!(f, "\u{1b}[48;2;51;51;102m -\u{1B}[0m"),
			Head(true) => write!(f, "\u{1b}[48;2;51;204;51m -\u{1B}[0m"),
			Head(false) => write!(f, "\u{1b}[48;2;201;51;51m -\u{1B}[0m"),
			Tail => write!(f, "\u{1b}[48;2;153;153;153m -\u{1B}[0m"),
		}
	}
}
impl FromStream for State {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('.') => Some((1, Empty)),
			Some('-') => Some((1, Wire)),
			Some('1') => Some((1, Head(true))),
			Some('0') => Some((1, Head(false))),
			Some('@') => Some((1, Tail)),
			_ => None,
		}
	}
}

#[allow(dead_code, unreachable_patterns)]
pub fn rule(binary: &[usize], ternary: &[usize]) -> BoxRule<State> {
	let binary = binary.to_owned();
	let ternary = ternary.to_owned();
	Box::new(move |_nw: &State, n: &State, _ne: &State,
	                 w: &State, c: &State,   e: &State,
	               _sw: &State, s: &State, _se: &State| -> State {
		match c {
			Empty => Empty,
			Wire =>
				if is_exist!(Tail in n,e,s,w) {Wire}
				else {
					let [wire_sum, head0_sum, head1_sum] =
						count!{Wire, Head(false), Head(true) in n,e,s,w};
					let head_sum = head0_sum + head1_sum;
					//无信号不变化
					if head_sum == 0 {Wire}
					//丁字路口倍增传输
					else if let d4_symmetry!(Head(b),Wire,Wire,Empty) = (n,w,e,s) {Head(*b)}
					//非丁字路口多出口阻塞
					else if wire_sum >= 2 {Wire}
					//二元门
					else if head_sum == 2 {Head(binary.contains(&head1_sum))}
					//三元门
					else if head_sum == 3 {Head(ternary.contains(&head1_sum))}
					//单输入不变传输，四输入不会产生影响，因此不专门写出
					else {Head(head1_sum == 1)}
				},
			Head(b) => if is_exist!(Wire in n,e,s,w) {Head(*b)} else {Tail},
			Tail => if is_exist!(Head(_) in n,e,s,w) {Tail} else {Wire}
		}
	})
}