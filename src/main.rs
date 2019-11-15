mod cellular_automaton;

use std::convert::TryFrom;
use cellular_automaton::Pattern;

macro_rules! rule {
	//规则函数
	(@fun B $($b:literal)* / S $($s:literal)*) =>
		{cellular_automaton::life::rule(&[$($b),*],&[$($s),*])};
	(@fun non-totalistic B $b:literal / S $s:literal) =>
		{cellular_automaton::life::non_totalistic_rule($b,$s)};
	(@fun B $($b:literal)* / S $($s:literal)* / G $g:literal) =>
		{cellular_automaton::generations::rule($g,&[$($b),*],&[$($s),*])};
	(@fun non-totalistic B $b:literal / S $s:literal / G $g:literal) =>
		{cellular_automaton::generations::non_totalistic_rule($g,$b,$s)};
	(@fun B $($b:literal)* / S $($s:literal)* / F $($f:literal)* / K $($k:literal)* / L $($l:literal)*) =>
		{cellular_automaton::bsfkl::rule(&[$($b),*],&[$($s),*],&[$($f),*],&[$($k),*],&[$($l),*])};
	(@fun Langton's Ant $($t:tt)+) =>
		{cellular_automaton::langton_s_ant::rule(concat!{$(stringify!($t)),+})};
	(@fun WireWorld) => {cellular_automaton::wireworld::rule};
	//规则字符串
	(@str non-totalistic B $b:literal / S $s:literal) =>
		{format!("B{}/S{}",$b,$s)};
	(@str non-totalistic B $b:literal / S $s:literal / G $g:literal) =>
		{format!("B{}/S{}/G{}",$b,$s,$g)};
	(@str Langton's Ant $($t:tt)+) =>
		{concat!{"Langton's Ant ", $(stringify!($t)),+}};
	(@str $($t:tt)*) => {concat!{$(stringify!($t)),+}};
	//输入
	($($t:tt)+) => {(rule!{@fun $($t)+}, rule!{@str $($t)+})};
}
macro_rules! run_ca {
	($rule:expr, $p:expr, $n:expr) => {
		let (rule, _rule_str) = $rule;
		let mut p = Pattern::try_from($p).ok().unwrap();
		for _n in 0..=$n {
			println!("{}第{}代\n", p, _n);
			p = p.tessellate_evolve(&rule);
			
		}
	};
	($rule:expr, $ground:expr, $p:expr, $n:expr) => {
		let (rule, _rule_str) = $rule;
		let mut p = (
			Pattern::try_from($p).ok().unwrap(),
			$ground,
			0, 0
		);
		let mut x_offset : isize = 0;
		let mut y_offset : isize = 0;
		for _n in 0..=$n {
			println!("{}[{}] ({}, {}) 第{}代\n", p.0, p.1, x_offset, y_offset, _n);
			p = p.0.infinte_evolve(&p.1, &rule);
			x_offset += p.2;
			y_offset += p.3;
		}
	};
}

macro_rules! check {
	(oscillator $rule:expr, $ground:expr, $p:expr, $period:expr) => {
		let (rule, rule_str) = $rule;
		let ground = $ground;
		let p = Pattern::try_from($p).ok().unwrap();
		println!("{}[{}]{}", p, ground, rule_str);
		if p.is_oscillator(&ground, &rule, $period) {
			println!("该图样是{}周期震荡子", $period);
		} else {
			println!("该图样不是{}周期震荡子", $period);
		}
	};
	(spaceship $rule:expr, $ground:expr, $p:expr, $period:expr) => {
		let (rule, rule_str) = $rule;
		let ground = $ground;
		let p = Pattern::try_from($p).ok().unwrap();
		println!("{}[{}]{}", p, ground, rule_str);
		let temp = p.is_spaceship(&ground, &rule, $period);
		if let Some((x, y)) = temp {
			println!("该图样是{}周期飞船，每循环周期内水平方向移动{}个单位、垂直方向移动{}个单位",
				$period, x, y);
		} else {
			println!("该图样不是{}周期飞船", $period);
		}
	};
}

fn main() {
	/*run_ca!{
		cellular_automaton::life::Cell,
		".#.$#.#$.#.$",
		cellular_automaton::life::rule(&[3],&[2,3]),
		2
	};
	run_ca!{
		cellular_automaton::life::Cell,
		".#.$..#$###$",
		cellular_automaton::life::Cell::Dead,
		cellular_automaton::life::rule(&[3],&[2,3]),
		10
	};
	run_ca!{
		cellular_automaton::generations::Cell,
		"..#..$.A##.$BC###$.A##.$..#..$",
		cellular_automaton::generations::Cell::Dead,
		cellular_automaton::generations::rule(3,&[3,5,7],&[3,4,5,7]),
		30
	};
	
	run_ca!{
		cellular_automaton::life::Cell,
		concat!{
			"######.##$",
			"######.##$",
			".......##$",
			"##.....##$",
			"##.....##$",
			"##.....##$",
			"##.......$",
			"##.######$",
			"##.######$"
		},
		cellular_automaton::life::Cell::Dead,
		cellular_automaton::life::rule(&[3],&[2,3]),
		8
	};*/
	
	run_ca!{
		rule!{Langton's Ant R L},
		cellular_automaton::langton_s_ant::Cell::NoAnt(0),
		concat!{
			"ABAAAAA$",
			"BBBB<AAA$",
			"BBBABAA$",
			"ABAABAA$",
			"ABABABA$",
			"BAABABB$",
			"BABBABA$",
			"ABBBBAA$",
			"AABBAAA$",
		},
		208
	};
	run_ca!{
		rule!{B 3 5 7 / S 3 4 5 7 / G 3},
		cellular_automaton::generations::Cell::Dead,
		concat!{
			".####..####..####.$",
			".####..####..####.$",
			"##BB####BB####BB##$",
			"BA..#CC#..#CC#..AB$",
			".C###CC####CC###C.$",
			"..BAA###CC###AAB..$",
			"...##..####..##...$",
			"...C....##....C...$"
		},
		28
	};
	check!{oscillator
		rule!{B 3 5 / S 2 3 4 6 / F 0 1 2 3 4 / K 2 3 / L 0 1 2 3 4},
		cellular_automaton::bsfkl::Cell::Dead,
		".##$###$###$.##$",
		8
	};
	check!{spaceship
		rule!{B 0 1 3 4 6 8 / S 2 3},
		cellular_automaton::life::Cell::Dead,
		"#.$.#$.#$#.$",
		2
	};
	check!{oscillator
		rule!{WireWorld},
		cellular_automaton::wireworld::Cell::Empty,
		".#.$@.-$.-.$",
		4
	};
	check!{oscillator
		rule!{B 3 / S 2 3},
		cellular_automaton::life::Cell::Dead,
		"###$",
		2
	};
	check!{spaceship
		rule!{B 3 / S 2 3},
		cellular_automaton::life::Cell::Dead,
		".#.$..#$###$",
		4
	};
	check!{oscillator
		rule!{Langton's Ant R L},
		cellular_automaton::langton_s_ant::Cell::NoAnt(0),
		"A^A^A$AAA$BAA$BBB$",
		40
	};
	check!{oscillator
		rule!{Langton's Ant R L},
		cellular_automaton::langton_s_ant::Cell::NoAnt(0),
		"A^A^$",
		28
	};
	check!{oscillator
		rule!{Langton's Ant R L},
		cellular_automaton::langton_s_ant::Cell::NoAnt(0),
		"A^A^$BA$",
		16
	};
	check!{spaceship
		rule!{non-totalistic B "2ce3ejnr4aejk5cry6ack" / S "2ci3-acq4aiqrt5-acek78"},
		cellular_automaton::life::Cell::Dead,
		"##..$####$#.#.$",
		23
	};
	check!{oscillator
		rule!{non-totalistic B "3e4aqw5y6ak" / S "2cik3ijr4aenqwy5ijk6ce" / G 2},
		cellular_automaton::generations::Cell::Dead,
		".#.#.$#A#.#$#BB#.$#BBA#$.###.$",
		11
	};
}
