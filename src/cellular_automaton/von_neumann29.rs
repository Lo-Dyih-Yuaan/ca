#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
	East,
	North,
	West,
	South
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Excitable {
	Quiescent,
	Excited
}
#[derive(PartialEq, Eq, Clone)]
pub enum State {
	Unexcitable,
	OrdinaryTransmission(Direction, Excitable),
	SpecialTransmission(Direction, Excitable),
	Confluent(Excitable, Excitable),
	/*Sensitized*/
	S, S0, S1, S00, S01, S10, S11, S000
}

#[allow(dead_code)]
pub fn rule(_nw: &State, n: &State, _ne: &State,
              w: &State, c: &State,   e: &State,
            _sw: &State, s: &State, _se: &State) -> State {
	#[derive(PartialEq, Eq, Clone)]
	enum Stimulus {
		Ordinary,
		Special,
		Confluent,
		Nil
	};
	/*一个状态`$s`向`$d`方向传导的冲激类型*/
	macro_rules! get_stimulus {
		($s:expr, $d:pat) => {
			match $s {
				State::Confluent(Excitable::Excited, _) =>
					Stimulus::Confluent,
				State::OrdinaryTransmission($d, Excitable::Excited) =>
					Stimulus::Ordinary,
				State::SpecialTransmission($d, Excitable::Excited) =>
					Stimulus::Special,
				_ => Stimulus::Nil
			}
		};
	}
	//e、n、w、s分别替换为从此处来的冲激
	let e = get_stimulus!(e, Direction::West);
	let n = get_stimulus!(n, Direction::South);
	let w = get_stimulus!(w, Direction::East);
	let s = get_stimulus!(s, Direction::North);
	macro_rules! is_exist {
		($e:expr; $i:expr) => {$e == $i};
		($e:expr; $i:expr, $($is:expr),*) =>
			{$e == $i || is_exist!($e; $($is),*)};
		($e1:expr, $e2:expr; $i:expr) => {$e1 == $i || $e2 == $i};
		($e1:expr, $e2:expr; $i:expr, $($is:expr),*) =>
			{$e1 == $i || $e2 == $i || is_exist!($e1, $e2; $($is),*)};
	}
	macro_rules! not_dir_is_exist {
		($($e:expr),*; not $d:expr) => {
			match $d {
				Direction::East => is_exist!($($e),*; n,w,s),
				Direction::North => is_exist!($($e),*; e,w,s),
				Direction::West => is_exist!($($e),*; e,n,s),
				Direction::South => is_exist!($($e),*; e,n,w),
			}
		};
	}
	if let State::Confluent(_, next) = c {
		if is_exist!(Stimulus::Special; e,n,w,s) {
			State::Unexcitable
		} else if is_exist!(Stimulus::Ordinary; e,n,w,s) {
			State::Confluent(*next, Excitable::Excited)
		} else {State::Confluent(*next, Excitable::Quiescent)}
	} else if let State::OrdinaryTransmission(dir, _) = c {
		if is_exist!(Stimulus::Special; e,n,w,s) {
			State::Unexcitable
		} else if not_dir_is_exist!(Stimulus::Ordinary,Stimulus::Confluent; not dir) {
			State::OrdinaryTransmission(*dir, Excitable::Excited)
		} else {State::OrdinaryTransmission(*dir, Excitable::Quiescent)}
	} else if let State::SpecialTransmission(dir, _) = c {
		if is_exist!(Stimulus::Ordinary; e,n,w,s) {
			State::Unexcitable
		} else if not_dir_is_exist!(Stimulus::Special,Stimulus::Confluent; not dir) {
			State::SpecialTransmission(*dir, Excitable::Excited)
		} else {State::SpecialTransmission(*dir, Excitable::Quiescent)}
	} else {
		match (c, is_exist!(Stimulus::Ordinary,Stimulus::Special; e,n,w,s)) {
			(State::Unexcitable, false) => State::Unexcitable,
			(State::Unexcitable, true) => State::S,
			(State::S, false) => State::S0,
			(State::S0, false) => State::S00,
			(State::S00, false) => State::S000,
			(State::S0, true) => State::S01,
			(State::S, true) => State::S1,
			(State::S1, false) => State::S10,
			(State::S1, true) => State::S11,
			(State::S000, false) => State::OrdinaryTransmission(Direction::East, Excitable::Quiescent),
			(State::S000, true) => State::OrdinaryTransmission(Direction::North, Excitable::Quiescent),
			(State::S00, true) => State::OrdinaryTransmission(Direction::West, Excitable::Quiescent),
			(State::S01, false) => State::OrdinaryTransmission(Direction::South, Excitable::Quiescent),
			(State::S01, true) => State::SpecialTransmission(Direction::East, Excitable::Quiescent),
			(State::S10, false) => State::SpecialTransmission(Direction::North, Excitable::Quiescent),
			(State::S10, true) => State::SpecialTransmission(Direction::West, Excitable::Quiescent),
			(State::S11, false) => State::SpecialTransmission(Direction::North, Excitable::Quiescent),
			(State::S11, true) => State::Confluent(Excitable::Quiescent, Excitable::Quiescent),
			_ => unreachable!()
		}
	}
}