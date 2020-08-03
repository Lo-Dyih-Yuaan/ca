use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::FromStream;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HorizontalDir {
	East,
	West
}
impl HorizontalDir {
	#[inline(always)]
	fn get<'a, T>(&self, e: &'a T, w: &'a T) -> &'a T {
		match self {
			East => e,
			West => w,
		}
	}
}
use self::HorizontalDir::*;
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VerticalDir {
	North,
	South
}
impl VerticalDir {
	#[inline(always)]
	fn get<'a, T>(&self, n: &'a T, s: &'a T) -> &'a T {
		match self {
			North => n,
			South => s,
		}
	}
}
use self::VerticalDir::*;
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Action {
	Inactive,
	Active
}
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
use self::Action::*;
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
	Empty,
	InactiveWire,
	ActiveWire,
	InhibitedWire,
	AuxiliaryAcive,
	AuxiliaryInhibited,
	NorGate(Action),
	OrGate(Action),
	XorGate(Action),
	AndGate(Action),
	TFlipFlop(Action),
	GateOutput(Action),
	Cross(Option<HorizontalDir>,Option<VerticalDir>)
}
impl std::ops::Not for State {
	type Output = State;
	#[inline(always)]
	fn not(self) -> Self::Output {
		match self {
			NorGate(a) => NorGate(!a),
			OrGate(a) => OrGate(!a),
			XorGate(a) => XorGate(!a),
			AndGate(a) => AndGate(!a),
			TFlipFlop(a) => TFlipFlop(!a),
			GateOutput(a) => GateOutput(!a),
			_ => unreachable!()
		}
	}
}
use self::State::*;
/*
impl Debug for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		macro_rules! dir_fmt {
			($e:expr) => {
				match $e {
					Some(East) => ">",
					Some(North) => "^",
					Some(West) => "<",
					Some(South) => "v",
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
					Some(East) => ">",
					Some(North) => "^",
					Some(West) => "<",
					Some(South) => "v",
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
}*/
impl FromStream for State {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		match s.chars().next() {
			Some('_') => Some((1,Empty)),
			Some('A') => Some((1,InactiveWire)),
			Some('B') => Some((1,ActiveWire)),
			Some('C') => Some((1,InhibitedWire)),
			Some('D') => Some((1,Cross(None,None))),
			Some('E') => Some((1,NorGate(Inactive))),
			Some('F') => Some((1,OrGate(Inactive))),
			Some('G') => Some((1,XorGate(Inactive))),
			Some('H') => Some((1,AndGate(Inactive))),
			Some('I') => Some((1,TFlipFlop(Inactive))),
			Some('J') => Some((1,GateOutput(Inactive))),
			Some('K') => Some((1,NorGate(Active))),
			Some('L') => Some((1,OrGate(Active))),
			Some('M') => Some((1,XorGate(Active))),
			Some('N') => Some((1,AndGate(Active))),
			Some('O') => Some((1,TFlipFlop(Active))),
			Some('P') => Some((1,GateOutput(Active))),
			Some('Q') => Some((1,Cross(None,Some(North)))),
			Some('R') => Some((1,Cross(Some(West),Some(South)))),
			Some('S') => Some((1,Cross(None,Some(South)))),
			Some('T') => Some((1,Cross(Some(West),Some(North)))),
			Some('U') => Some((1,AuxiliaryAcive)),
			Some('V') => Some((1,Cross(Some(East),Some(South)))),
			Some('W') => Some((1,Cross(Some(East),Some(North)))),
			Some('X') => Some((1,AuxiliaryInhibited)),
			Some('Y') => Some((1,Cross(Some(East),None))),
			Some('Z') => Some((1,Cross(Some(West),None))),
			_ => None
		}
	}
}

#[allow(dead_code)]
pub fn rule(_nw: &State, n: &State, _ne: &State,
              w: &State, c: &State,   e: &State,
            _sw: &State, s: &State, _se: &State) -> State {
	/*一个状态`$s`向`$d`方向传导的冲激类型*/
	macro_rules! is_exist {
		($($p:pat)|+ in $i:expr) => {matches!($i, $($p)|+)};
		($($p:pat)|+ in $i:expr, $($is:expr),*) =>
			{matches!($i, $($p)|+) || is_exist!($($p)|+ in $($is),*)};
	}
	match c {
		Empty => Empty,
		InactiveWire =>
			if is_exist!(ActiveWire|AuxiliaryAcive in n,e,s,w) {ActiveWire}
			else if let Cross(_, Some(North)) = n {AuxiliaryAcive}
			else if let Cross(Some(East), _) = e {AuxiliaryAcive}
			else if let Cross(_, Some(South)) = s {AuxiliaryAcive}
			else if let Cross(Some(West), _) = w {AuxiliaryAcive}
			else if is_exist!(GateOutput(Active) in n,e,s,w) {ActiveWire}
			else {InactiveWire},
		ActiveWire =>
			if is_exist!(InhibitedWire in n,e,s,w) && !is_exist!{AuxiliaryAcive|Cross(_,_) in n,e,s,w}
				{InhibitedWire}
			else if is_exist!(InhibitedWire in n,e,s,w) && is_exist!{Cross(_,_)|OrGate(Active) in n,e,s,w}
				{AuxiliaryInhibited}
			else if is_exist!(GateOutput(Inactive) in n,e,s,w) {InhibitedWire}
			else {ActiveWire}
		InhibitedWire => InactiveWire,
		AuxiliaryAcive =>
			if count!{$ AuxiliaryAcive,Cross(_,_) in n,e,s,w} == [0,1] {
				if let Cross(_,None) = n {InhibitedWire}
				else if let Cross(None,_) = e {InhibitedWire}
				else if let Cross(_,None) = s {InhibitedWire}
				else if let Cross(None,_) = w {InhibitedWire}
				else {AuxiliaryAcive}
			} else {AuxiliaryAcive},
		AuxiliaryInhibited => InhibitedWire,
		GateOutput(Inactive) => {
			let gates = count!{$
				NorGate(Active),  OrGate(Active),  XorGate(Active),  AndGate(Active),  TFlipFlop(Active),
				NorGate(Inactive),OrGate(Inactive),XorGate(Inactive),AndGate(Inactive),TFlipFlop(Inactive)
			in n,e,s,w};
			let [_, a_or, a_xor, a_and, a_t, i_nor, i_or, i_xor, _, _] = gates;
			let gates = gates.iter().sum();
			if a_xor == 1 && a_xor + i_xor == gates {!*c} //XOR
			else if a_and > 0 && a_and == gates {!*c} //AND
			else if a_or + i_nor > 0 && a_or + i_nor + i_or == gates {!*c} //(N)OR
			else if a_t > 0 {!*c} //T Flip-Flop
			else {*c}
		},
		GateOutput(Active) => {
			let gates = count!{$
				NorGate(Active),  OrGate(Active),  XorGate(Active),  AndGate(Active),  TFlipFlop(Active),
				NorGate(Inactive),OrGate(Inactive),XorGate(Inactive),AndGate(Inactive),TFlipFlop(Inactive)
			in n,e,s,w};
			let [a_nor, _, a_xor, _, _, _, i_or, i_xor, i_and, i_t] = gates;
			let gates = gates.iter().sum();
			if a_xor != 1 && a_xor + i_xor == gates {!*c} //XOR
			else if i_and > 0 {!*c} //AND
			else if i_or + a_nor == gates {!*c} //(N)OR
			else if i_t + a_nor > 0 {!*c} //T Flip-Flop
			else {*c}
		},
		NorGate(Inactive)|OrGate(Inactive)|XorGate(Inactive)|AndGate(Inactive) =>
			if is_exist!(ActiveWire in n,e,s,w) {!*c} else {*c},
		NorGate(Active)|OrGate(Active)|XorGate(Active)|AndGate(Active) =>
			if !is_exist!(ActiveWire|AuxiliaryAcive in n,e,s,w) {!*c} else {*c},
		TFlipFlop(_) => if is_exist!(InhibitedWire in n,e,s,w) {!*c} else {*c},
		Cross(None, None) => {
			let h =
				if *e == ActiveWire {
					if let Cross(_,_)|Empty|InactiveWire|AuxiliaryInhibited = w {Some(East)}
					else {return *c;}
				} else if *w == ActiveWire {
					if let Cross(_,_)|Empty|InactiveWire|AuxiliaryInhibited = e {Some(West)}
					else {return *c;}
				} else {None};
			let v =
				if *n == ActiveWire {
					if let Cross(_,_)|Empty|InactiveWire|AuxiliaryInhibited = s {Some(North)}
					else {return *c;}
				} else if *s == ActiveWire {
					if let Cross(_,_)|Empty|InactiveWire|AuxiliaryInhibited = n {Some(South)}
					else {return *c;}
				} else {None};
			Cross(h, v)
		},
		Cross(Some(d), None) => {
			let h =
				if *e == ActiveWire && *w == ActiveWire {return *c;}
				else if *d.get(e, w) == ActiveWire {Some(*d)}
				else if *e != ActiveWire && *w != ActiveWire {None}
				else if *n != ActiveWire && *s != ActiveWire {None}
				else {return *c};
			let v = match (n, s) {
				(ActiveWire, ActiveWire) => {return *c;},
				(ActiveWire, _) => Some(North),
				(_, ActiveWire) => Some(South),
				_ => None
			};
			Cross(h, v)
		},
		Cross(None, Some(d)) => {
			let h = match (e, w) {
				(ActiveWire, ActiveWire) => {return *c;},
				(ActiveWire, _) => Some(East),
				(_, ActiveWire) => Some(West),
				_ => None
			};
			let v =
				if *n == ActiveWire && *s == ActiveWire {return *c;}
				else if *d.get(n, s) == ActiveWire {Some(*d)}
				else if *n != ActiveWire && *s != ActiveWire {None}
				else if *e != ActiveWire && *w != ActiveWire {None}
				else {return *c};
			Cross(h, v)
		},
		Cross(Some(dh), Some(dv)) => {
			let h = dh.get(e, w);
			let v = dv.get(n, s);
			if is_exist!(AuxiliaryAcive in h,v) {return *c;}
			let h = if let ActiveWire = h {Some(*dh)} else {None};
			let v = if let ActiveWire = v {Some(*dv)} else {None};
			Cross(h, v)
		},
	}
}