pub trait RuleType<T> : Fn(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T {}
impl<T, F: Fn(&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T> RuleType<T> for F {}
type BoxRule<T> = Box<dyn Fn (&T, &T, &T, &T, &T, &T, &T, &T, &T) -> T>;

#[macro_use]
mod symmetry;
#[macro_use]
mod tools;
pub mod from_stream;

pub mod life;
pub mod generations;
pub mod extended_generations;
pub mod wireworld;
pub mod logic_land;
pub mod no_time_at_all;
pub mod langton_s_ant;
pub mod bsfkl;
pub mod von_neumann29;
pub mod nobili32;
pub mod hutton32;

#[macro_export]
macro_rules! rule {
	(@extgen_a) => {&[]};
	(@extgen_i) => {&[]};
	(@extgen_a A $a:literal $($t:tt)*) => {
		&[&[$a as usize] as &[_], rule!(@extgen_i $($t)*)].concat()
	};
	(@extgen_i I $i:literal $($t:tt)*) => {
		&[&[$i as usize] as &[_], rule!(@extgen_a $($t)*)].concat()
	};
	//规则函数
	(@fun B $($b:literal)* / S $($s:literal)*) => {
		$crate::rules::
			life::rule(&[$($b),*],&[$($s),*])
	};
	(@fun B $($b:literal)* / S $($s:literal)* H) => {
		$crate::rules::
			life::rule_h(&[$($b),*],&[$($s),*])
	};
	(@fun non-totalistic B $b:literal / S $s:literal) => {
		$crate::rules::
			life::non_totalistic_rule($b,$s)
	};
	(@fun non-totalistic B $b:literal / S $s:literal H) => {
		$crate::rules::
			life::non_totalistic_rule_h($b,$s)
	};
	(@fun B $($b:literal)* / S $($s:literal)* / G $g:literal) => {
		$crate::rules::
			generations::rule($g,&[$($b),*],&[$($s),*])
	};
	(@fun B $($b:literal)* / S $($s:literal)* / G $g:literal H) => {
		$crate::rules::
			generations::rule_h($g,&[$($b),*],&[$($s),*])
	};
	(@fun non-totalistic B $b:literal / S $s:literal / G $g:literal) => {
		$crate::rules::
			generations::non_totalistic_rule($g,$b,$s)
	};
	(@fun non-totalistic B $b:literal / S $s:literal / G $g:literal H) => {
		$crate::rules::
			generations::non_totalistic_rule_h($g,$b,$s)
	};
	(@fun B $($b:literal)* / S $($s:literal)* / A $a:literal $($t:tt)*) => {
		$crate::rules::
			extended_generations::rule(
				&[&[$a as usize] as &[_], rule!(@extgen_i $($t)*)].concat(),
				&[$($b),*],&[$($s),*]
			)
	};
	(@fun B $($b:literal)* / S $($s:literal)* / I $i:literal $($t:tt)*) => {
		$crate::rules::
			extended_generations::rule(
				&[&[0,$i as usize] as &[_], rule!(@extgen_a $($t)*)].concat(),
				&[$($b),*],&[$($s),*]
			)
	};
	(@fun non-totalistic B $b:literal / S $s:literal / A $a:literal $($t:tt)*) => {
		$crate::rules::
			extended_generations::non_totalistic_rule(
				&[&[$a as usize] as &[_], rule!(@extgen_i $($t)*)].concat(),
				$b,$s
			)
	};
	(@fun non-totalistic B $b:literal / S $s:literal / I $i:literal $($t:tt)*) => {
		$crate::rules::
			extended_generations::non_totalistic_rule(
				&[&[0,$i as usize] as &[_], rule!(@extgen_a $($t)*)].concat(),
				$b,$s
			)
	};
	(@fun B $($b:literal)* / S $($s:literal)* / F $($f:literal)* / K $($k:literal)* / L $($l:literal)*) => {
		$crate::rules::
			bsfkl::rule(&[$($b),*],&[$($s),*],&[$($f),*],&[$($k),*],&[$($l),*])
	};
	(@fun Langton's Ant $s:literal) => {
		$crate::rules::
			langton_s_ant::rule($s)
	};
	(@fun Langton's Ant $($t:tt)+) => {
		$crate::rules::
			langton_s_ant::rule(concat!{$(stringify!($t)),+})
	};
	(@fun WireWorld) => {$crate::rules::wireworld::rule};
	(@fun LogicLand) => {$crate::rules::logic_land::rule};
	(@fun NoTimeAtAll) => {
		$crate::rules::
			no_time_at_all::rule(&[1],&[1])
	};
	(@fun NoTimeAtAll - B $($b:literal)* / T $($t:literal)*) => {
		$crate::rules::
			no_time_at_all::rule(&[$($b),*],&[$($t),*])
	};
	(@fun von Neumann 29) => {$crate::rules::von_neumann29::rule};
	(@fun Nobili 32) => {$crate::rules::nobili32::rule};
	(@fun Hutton 32) => {$crate::rules::hutton32::rule};
	//规则字符串
	(@str non-totalistic $($t:tt)*) =>
		{rule!(@str $($t)*)};
	(@str Langton's Ant $s:literal) => {concat!{"Langton's Ant ", $s}};
	(@str Langton's Ant $($t:tt)+) =>
		{concat!{"Langton's Ant ", $(stringify!($t)),+}};
	(@str von Neumann 29) => {"von Neumann 29"};
	(@str $($t:tt)*) => {concat!{$(stringify!($t)),+}};
	//输出
	(@display B $($b:literal)* / S $($s:literal)* H) => {"{:x}"};
	(@display non-totalistic B $b:literal / S $s:literal H) => {"{:x}"};
	(@display B $($b:literal)* / S $($s:literal)* / G $g:literal H) => {"{:x}"};
	(@display non-totalistic B $b:literal / S $s:literal / G $g:literal H) => {"{:x}"};
	(@display $($t:tt)*) => {"{}"};
	//输入
	($($t:tt)+) => {(rule!{@fun $($t)+}, rule!{@str $($t)+}, rule!{@display $($t)+})};
}
/*
use proc_macro::TokenStream;
#[proc_macro]
pub fn rule_string(item: TokenStream) -> TokenStream {
	let item = item.to_string();
	let is_hex = item.ends_with("H");
	if is_hex {item.strip_suffix("H").unwrap()} else {item};
	let items = item.split('/').collect();
	if items.len() == 2 && items[0].starts_with("B") && items[1].starts_with("S") {
		format!("rules::life::non_totalistic_rule({:?},{:?})",
			item[0].strip_prefix("B").unwrap(),
			item[1].strip_prefix("S").unwrap())
	} else {unreachable!()}.parse().unwrap()
}*/