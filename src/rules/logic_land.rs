use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use super::from_stream::*;

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
		//
	}
}
impl Display for State {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		//
	}
}
impl FromStream for State {
	fn from_stream(s: &str) -> Option<(usize, Self)> {
		//
	}
}*/

#[allow(dead_code)]
pub fn rule(_nw: &State, n: &State, _ne: &State,
              w: &State, c: &State,   e: &State,
            _sw: &State, s: &State, _se: &State) -> State {
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
		ActiveWire => {
			let has_ihw = is_exist!(InhibitedWire in n,e,s,w);
			if has_ihw && !is_exist!{AuxiliaryAcive|Cross(_,_) in n,e,s,w} {InhibitedWire}
			else if has_ihw && is_exist!{Cross(_,_)|OrGate(Active) in n,e,s,w} {AuxiliaryInhibited}
			else if is_exist!(GateOutput(Inactive) in n,e,s,w) {InhibitedWire}
			else {ActiveWire}
		},
		InhibitedWire => InactiveWire,
		AuxiliaryAcive =>
			if count!{AuxiliaryAcive,Cross(_,_) in n,e,s,w} == [0,1] {
				if let Cross(_,None) = n {InhibitedWire}
				else if let Cross(None,_) = e {InhibitedWire}
				else if let Cross(_,None) = s {InhibitedWire}
				else if let Cross(None,_) = w {InhibitedWire}
				else {AuxiliaryAcive}
			} else {AuxiliaryAcive},
		AuxiliaryInhibited => InhibitedWire,
		GateOutput(Inactive) => {
			let gates = count!{
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
			let gates = count!{
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
		TFlipFlop(_) =>
			if is_exist!(InhibitedWire in n,e,s,w) {!*c} else {*c},
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
			if (n == s && *s == ActiveWire) || (e == w && *w == ActiveWire) {
				return *c;
			}
			let v = match (n, s) {
				(ActiveWire, _) => Some(North),
				(_, ActiveWire) => Some(South),
				_ => None
			};
			if *d.get(e, w) == ActiveWire {Cross(Some(*d), v)}
			else if *e != ActiveWire && *w != ActiveWire {Cross(None, v)}
			else if *n != ActiveWire && *s != ActiveWire {Cross(None, v)}
			else {*c}
		},
		Cross(None, Some(d)) => {
			if (e == w && *w == ActiveWire) || (n == s && *s == ActiveWire) {
				return *c;
			}
			let h = match (e, w) {
				(ActiveWire, _) => Some(East),
				(_, ActiveWire) => Some(West),
				_ => None
			};
			if *d.get(n, s) == ActiveWire {Cross(h, Some(*d))}
			else if *n != ActiveWire && *s != ActiveWire {Cross(h, None)}
			else if *e != ActiveWire && *w != ActiveWire {Cross(h, None)}
			else {*c}
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