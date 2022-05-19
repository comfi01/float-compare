fn helper_u128<T, F>(make: F)
where
	T: Clone
		+ std::fmt::Debug
		+ std::ops::Add<Output = T>
		// + std::ops::Sub<Output = T>
		// + std::ops::Mul<Output = T>
		// + std::ops::Div<Output = T>
		+ PartialEq
		+ PartialOrd,
	F: Fn(u128) -> T,
{
	let a = make(4125636888562548868221559797461449);
	let y = make(4125636888562548868221559797461449 * 2);
	let x = a.clone() + a.clone();
	assert_eq!(x, y);

	let a = make(23768741896345550770650537601358309);
	let y = make(23768741896345550770650537601358309 * 2);
	let x = a.clone() + a.clone();
	assert_eq!(x, y);

	let a = make(u128::MAX);
	let b = make(1);
	let y = a.clone() + b;
	let x = a.clone() + a.clone();
	assert!(x > y);
}

fn helper_f64<T, F>(make: F)
where
	T: Clone
		+ std::fmt::Debug
		+ std::ops::Add<Output = T>
		// + std::ops::Sub<Output = T>
		// + std::ops::Mul<Output = T>
		// + std::ops::Div<Output = T>
		+ PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(f64::MAX);
	let b = make(f64::MAX);
	let _ = a.clone() + b.clone();
}

#[test]
fn overflow_fixedu128() {
	type Number = sp_runtime::FixedU128;
	fn make_f64(x: f64) -> Number {
		Number::from_fraction(x)
	}
	fn make_u128(x: u128) -> Number {
		Number::from(x)
	}
	helper_f64(make_f64);
	helper_u128(make_u128);
}

#[test]
fn overflow_rational_64() {
	type Number = num_rational::Rational64;
	fn make_f64(x: f64) -> Number {
		Number::approximate_float(x).unwrap()
	}
	fn make_u128(x: u128) -> Number {
		Number::from_integer(x as _)
	}
	helper_f64(make_f64);
	helper_u128(make_u128);
}

#[test]
fn overflow_rational_big() {
	type Number = num_rational::BigRational;
	fn make_f64(x: f64) -> Number {
		Number::from_float(x).unwrap()
	}
	fn make_u128(x: u128) -> Number {
		Number::from_integer(x.into())
	}
	helper_f64(make_f64);
	helper_u128(make_u128);
}

#[test]
fn overflow_bigfixed() {
	type Number = bigfixed::BigFixed;
	fn make_f64(x: f64) -> Number {
		bigfixed::BigFixed::from(x)
	}
	fn make_u128(x: u128) -> Number {
		bigfixed::BigFixed::from(x)
	}
	helper_f64(make_f64);
	helper_u128(make_u128);
}

#[test]
fn overflow_bigdecimal() {
	type Number = bigdecimal::BigDecimal;
	fn make_f64(x: f64) -> Number {
		use bigdecimal::FromPrimitive as _;
		Number::from_f64(x).unwrap()
	}
	fn make_u128(x: u128) -> Number {
		use bigdecimal::FromPrimitive as _;
		Number::from_u128(x).unwrap()
	}
	helper_f64(make_f64);
	helper_u128(make_u128);
}

#[test]
fn overflow_native_f64() {
	type Number = f64;
	fn make_f64(x: f64) -> Number {
		x
	}
	fn make_u128(x: u128) -> Number {
		x as _
	}
	helper_f64(make_f64);
	helper_u128(make_u128);
}
