#[inline(always)]
pub fn u32_print_str(n: u32) -> String {
	if n < 26 {
		std::char::from_u32(0x41u32+n).unwrap().to_string()
	} else {
		format!("[{}]", n)
	}
}

/**
 * 用于计数。
 * `$d`必为`$`，以解决宏内`$`将被转义的问题。
 * `$m`为统计对象；`$e`为待统计对象。
 * 返回一个长度与`$m`相同的`usize`数组。
 * 匹配使用`==`。
 */
macro_rules! count {
	(@nest $($body:tt)*) => {
		macro_rules! __with_dollar_sign { $($body)* }
		__with_dollar_sign!($);
	};
	(@index) => {0};
	(@index $e:tt) => {1};
	(@index $e1:tt, $e2:tt) => {2};
	(@index $e1:tt, $e2:tt, $e3:tt) => {3};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt) => {4};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt, $e5:tt) => {5};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt, $e5:tt, $e6:tt) => {6};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt, $e5:tt, $e6:tt, $e7:tt) => {7};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt, $e5:tt, $e6:tt, $e7:tt, $e8:tt) => {8};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt, $e5:tt, $e6:tt, $e7:tt, $e8:tt, $e9:tt) => {9};
	(@index $e1:tt, $e2:tt, $e3:tt, $e4:tt, $e5:tt, $e6:tt, $e7:tt, $e8:tt, $e9:tt, $e10:tt) => {10};
	(@if $c:expr, $e:expr, $m:pat if $($os:pat),*) => {
		if let $m = $e {$c[count!{@index $($os),*}] += 1;}
	};
	(@if $c:expr, $e:expr, $m:pat, $($ms:pat),+ if $($os:pat),*) => {
		if let $m = $e {$c[count!{@index $($os),*}] += 1;}
		else {count!{@if $c, $e, $($ms),+ if $($os,)* $m}}
	};
	($($m:pat),+ in $($e:expr),*) => {{
		let mut temp: [usize; count!(@index $($m),+)] =
			[0; count!(@index $($m),+)];
		count!{@nest ($d:tt) => {
			macro_rules! __count {
				($d arg:expr) => {
					count!{@if temp, $d arg, $($m),+ if}
				};
			}
		}}
		$(__count!($e);)*
		temp
	}};
}

macro_rules! is_exist {
	($($p:pat)|+ in $i:expr) => {matches!($i, $($p)|+)};
	($($p:pat)|+ in $i:expr, $($is:expr),*) =>
		{matches!($i, $($p)|+) || is_exist!($($p)|+ in $($is),*)};
}