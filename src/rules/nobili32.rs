use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
	East,
	North,
	West,
	South
}
use self::Direction::*;
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Excitation {
	Quiescent,
	Excited
}
use self::Excitation::*;
#[derive(PartialEq, Eq, Clone)]
pub enum State {
	Unexcitable,
	OrdinaryTransmission(Direction, Excitation),
	SpecialTransmission(Direction, Excitation),
	Confluent(Excitation, Excitation),
	HorizontalConfluent, VerticalConfluent, OrthogonalConfluent,
	/*Sensitized*/
	S, S0, S1, S00, S01, S10, S11, S000
}
use self::State::*;

impl Debug for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		macro_rules! dir_fmt {
			($e:expr) => {
				match $e {
					East => ">",
					North => "^",
					West => "<",
					South => "v",
				}
			}
		}
		macro_rules! exc_fmt {
			($e:expr) => {
				match $e {
					Quiescent => "_",
					Excited => "~",
				}
			}
		}
		match self {
			Unexcitable => write!(f, "U"),
			Confluent(e1, e2) =>
				write!(f, "C{}{}", exc_fmt!(e1), exc_fmt!(e2)),
			OrdinaryTransmission(d, e) =>
				write!(f, "To{}{}", dir_fmt!(d), exc_fmt!(e)),
			SpecialTransmission(d, e) =>
				write!(f, "Ts{}{}", dir_fmt!(d), exc_fmt!(e)),
			HorizontalConfluent => write!(f, "C-"),
			VerticalConfluent => write!(f, "C|"),
			OrthogonalConfluent => write!(f, "C+"),
			S => write!(f, "S"),
			S0 => write!(f, "S0"),
			S1 => write!(f, "S1"),
			S00 => write!(f, "S00"),
			S01 => write!(f, "S01"),
			S10 => write!(f, "S10"),
			S11 => write!(f, "S11"),
			S000 => write!(f, "S000"),
		}
	}
}
impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		macro_rules! dir_fmt {
			($e:expr) => {
				match $e {
					East => ">",
					North => "^",
					West => "<",
					South => "v",
				}
			}
		}
		macro_rules! exc_fmt {
			($e:expr) => {
				match $e {
					Quiescent => "\u{1b}[43m_",
					Excited => "\u{1b}[103m~",
				}
			}
		}
		match self {
			Unexcitable => write!(f, " -"),
			Confluent(e1, e2) =>
				write!(f, "{}{}\u{1b}[0m", exc_fmt!(e1), exc_fmt!(e2)),
			OrdinaryTransmission(d, Quiescent) =>
				write!(f, "\u{1b}[34m {}\u{1b}[0m", dir_fmt!(d)), //蓝
			OrdinaryTransmission(d, Excited) =>
				write!(f, "\u{1b}[32m {}\u{1b}[0m", dir_fmt!(d)), //绿
			SpecialTransmission(d, Quiescent) =>
				write!(f, "\u{1b}[31m {}\u{1b}[0m", dir_fmt!(d)), //红
			SpecialTransmission(d, Excited) =>
				write!(f, "\u{1b}[35m {}\u{1b}[0m", dir_fmt!(d)), //紫
			HorizontalConfluent => write!(f, "\u{1b}[103m -\u{1b}[0m"),
			VerticalConfluent => write!(f, "\u{1b}[103m |\u{1b}[0m"),
			OrthogonalConfluent => write!(f, "\u{1b}[103m +\u{1b}[0m"),
			S => write!(f, " S"),
			S0 => write!(f, "S0"),
			S1 => write!(f, "S1"),
			S00 => write!(f, "00"),
			S01 => write!(f, "01"),
			S10 => write!(f, "10"),
			S11 => write!(f, "11"),
			S000 => write!(f, "*0"),
		}
	}
}
impl FromStream for State {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		macro_rules! get_dir {
			($e:expr) => {
				match $e {
					Some('>') => East,
					Some('^') => North,
					Some('<') => West,
					Some('v') => South,
					_ => {return None;}
				}
			}
		}
		macro_rules! get_exc {
			($e:expr) => {
				match $e {
					Some('_') => Quiescent,
					Some('~') => Excited,
					_ => {return None;}
				}
			}
		}
		let mut chars = s.chars();
		match chars.next() {
			Some('U') => Some((1, Unexcitable)),
			Some('C') => {
				let c = chars.next();
				match c {
					Some('-') => Some((2, HorizontalConfluent)),
					Some('|') => Some((2, VerticalConfluent)),
					Some('+') => Some((2, OrthogonalConfluent)),
					_ => {
						let e1 = get_exc!(c);
						let e2 = get_exc!(chars.next());
						Some((3, Confluent(e1, e2)))
					}
				}
			},
			Some('T') => {
				let state_type = chars.next();
				let d = get_dir!(chars.next());
				let e = get_exc!(chars.next());
				match state_type {
					Some('o') => Some((4, OrdinaryTransmission(d, e))),
					Some('s') => Some((4, SpecialTransmission(d, e))),
					_ => None
				}
			},
			Some('S') => {
				let mut now = S;
				let mut len = 0usize;
				for c in chars {
					now = match (now.clone(), c) {
						(S, '0') => S0,
						(S0, '0') => S00,
						(S00, '0') => S000,
						(S0, '1') => S01,
						(S, '1') => S1,
						(S1, '0') => S10,
						(S1, '1') => S11,
						_ => {break;}
					};
					len += 1;
				}
				Some((len+1, now))
			},
			_ => None
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Stimulus {
	Ordinary, //普通冲激
	Empty, //未激发的普通传输态传输的信号
	Special, //特殊冲激
	Logical, //逻辑冲激，由汇合态进行合取运算后得到的冲激
	Input, //表明可接受信号
	Silent //静止，无冲激
}
#[allow(dead_code)]
pub fn rule(_nw: &State, n: &State, _ne: &State,
              w: &State, c: &State,   e: &State,
            _sw: &State, s: &State, _se: &State) -> State {
	use self::Stimulus::*;
	/*一个状态`$s`向`$d`方向传导的冲激类型*/
	macro_rules! get_stimulus {
		($s:expr, $d:expr) => {
			match $s {
				Confluent(Excited, _) => Logical,
				HorizontalConfluent => if let East|West = $d {Logical} else {Silent},
				VerticalConfluent => if let South|North = $d {Logical} else {Silent},
				OrthogonalConfluent => Logical,
				OrdinaryTransmission(d, Excited) if *d == $d => Ordinary,
				OrdinaryTransmission(d, Quiescent) if *d == $d => Empty,
				OrdinaryTransmission(_, _) => Input,
				SpecialTransmission(d, Excited) if *d == $d => Special,
				SpecialTransmission(d, Quiescent) if *d == $d => Silent,
				SpecialTransmission(_, _) => Input,
				_ => Silent
			}
		};
	}
	//e、n、w、s分别替换为从此处来的冲激
	let e = get_stimulus!(e, West);
	let n = get_stimulus!(n, South);
	let w = get_stimulus!(w, East);
	let s = get_stimulus!(s, North);
	macro_rules! is_exist {
		($($p:pat)|+ in not $d:expr) => {
			match $d {
				East => is_exist!($($p)|+ in n,w,s),
				North => is_exist!($($p)|+ in e,w,s),
				West => is_exist!($($p)|+ in e,n,s),
				South => is_exist!($($p)|+ in e,n,w),
			}
		};
		($($p:pat)|+ in $i:expr) => {matches!($i, $($p)|+)};
		($($p:pat)|+ in $i:expr, $($is:expr),*) =>
			{matches!($i, $($p)|+) || is_exist!($($p)|+ in $($is),*)};
	}
	//汇合态
	if let Confluent(_, _)|HorizontalConfluent|VerticalConfluent|OrthogonalConfluent = c {
		let is_intersection =
			matches!((e, w), (Ordinary, Input)|(Empty, Input)|(Input, Ordinary)|(Input, Empty)) &&
			matches!((n, s), (Ordinary, Input)|(Empty, Input)|(Input, Ordinary)|(Input, Empty));
		let next = if let Confluent(_, next) = c {*next} else {Quiescent};
		if is_intersection {
			let horizontal = is_exist!(Ordinary in e,w);
			let vertical = is_exist!(Ordinary in n,s);
			match (horizontal, vertical) {
				( true,  true) => OrthogonalConfluent,
				( true, false) => HorizontalConfluent,
				(false,  true) => VerticalConfluent,
				(false, false) => Confluent(Quiescent, Quiescent)
			}
		} else if is_exist!(Special in e,n,w,s) {
			Unexcitable
		} else if is_exist!(Ordinary in e,n,w,s) && !is_exist!(Empty in e,n,w,s) {
			Confluent(next, Excited)
		} else if is_exist!(Input in e,n,w,s) {
			Confluent(next, Quiescent)
		} else if let Confluent(_, _) = c {
			c.clone()
		} else {Confluent(Quiescent, Quiescent)}
	//普通传输态
	} else if let OrdinaryTransmission(dir, _) = c {
		if is_exist!(Special in e,n,w,s) {
			Unexcitable
		} else if is_exist!(Ordinary|Logical in not dir) {
			OrdinaryTransmission(*dir, Excited)
		} else {OrdinaryTransmission(*dir, Quiescent)}
	//特殊传输态
	} else if let SpecialTransmission(dir, _) = c {
		if is_exist!(Ordinary in e,n,w,s) {
			Unexcitable
		} else if is_exist!(Special|Logical in not dir) {
			SpecialTransmission(*dir, Excited)
		} else {SpecialTransmission(*dir, Quiescent)}
	//激发态
	} else {
		match (c, is_exist!(Ordinary|Special in e,n,w,s)) {
			(Unexcitable, false) => Unexcitable,
			(Unexcitable, true) => S,
			(S, false) => S0,
			(S0, false) => S00,
			(S00, false) => S000,
			(S0, true) => S01,
			(S, true) => S1,
			(S1, false) => S10,
			(S1, true) => S11,
			(S000, false) => OrdinaryTransmission(East, Quiescent),
			(S000, true) => OrdinaryTransmission(North, Quiescent),
			(S00, true) => OrdinaryTransmission(West, Quiescent),
			(S01, false) => OrdinaryTransmission(South, Quiescent),
			(S01, true) => SpecialTransmission(East, Quiescent),
			(S10, false) => SpecialTransmission(North, Quiescent),
			(S10, true) => SpecialTransmission(West, Quiescent),
			(S11, false) => SpecialTransmission(South, Quiescent),
			(S11, true) => Confluent(Quiescent, Quiescent),
			_ => unreachable!()
		}
	}
}