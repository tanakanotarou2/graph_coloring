mod procon_utils;
pub mod main;

/**
 * vector 初期化
 * e.g.)
 *  mat![1,2,3]; // [1, 2, 3]
 *  mat![false; N; M]; // false で初期化された N * M の vector
 */
#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

