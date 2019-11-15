#[derive(PartialEq, Eq, Clone)]
pub enum Direction {
	East,
	North,
	West,
	South
}
#[derive(PartialEq, Eq, Clone)]
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
	match c {
		State::Unexcitable => State::S,
	}
}