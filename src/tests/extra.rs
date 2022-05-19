fn helper<T, F>(make: F)
where
	T: Clone
		+ std::fmt::Debug
		// + std::ops::Add<Output = T>
		// + std::ops::Sub<Output = T>
		// + std::ops::Mul<Output = T>
		// + std::ops::Div<Output = T>
		+ PartialEq,
	F: Fn(f64) -> T,
{
	let _ = make(f64::INFINITY);
	let _ = make(f64::NEG_INFINITY);
	let _ = make(f64::NAN);
	let _ = make(f64::MIN);
	let _ = make(f64::MIN_POSITIVE);
	let _ = make(f64::MAX);
}

#[test]
fn extra_fixedu128() {
	type Number = sp_runtime::FixedU128;
	fn make(x: f64) -> Number {
		Number::from_fraction(x)
	}
	helper(make);
}

#[test]
fn extra_rational_64() {
	type Number = num_rational::Rational64;
	fn make(x: f64) -> Number {
		Number::approximate_float(x).unwrap()
	}
	helper(make);
}

#[test]
fn extra_rational_big() {
	type Number = num_rational::BigRational;
	fn make(x: f64) -> Number {
		Number::from_float(x).unwrap()
	}
	helper(make);
}

#[test]
fn extra_bigfixed() {
	type Number = bigfixed::BigFixed;
	fn make(x: f64) -> Number {
		bigfixed::BigFixed::from(x)
	}
	helper(make);
}

#[test]
fn extra_bigdecimal() {
	type Number = bigdecimal::BigDecimal;
	fn make(x: f64) -> Number {
		use bigdecimal::FromPrimitive as _;
		Number::from_f64(x).unwrap()
	}
	helper(make);
}

#[test]
fn extra_native_f64() {
	type Number = f64;
	fn make(x: f64) -> Number {
		x
	}
	helper(make);
}
