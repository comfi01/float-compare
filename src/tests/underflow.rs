fn helper<T, F>(make: F)
where
	T: Clone
		+ std::fmt::Debug
		+ std::ops::Add<Output = T>
		+ std::ops::Sub<Output = T>
		+ std::ops::Mul<Output = T>
		// + std::ops::Div<Output = T>
		+ PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(f64::MIN);
	let b = make(f64::MAX);
	let _ = a.clone() - b.clone();
}

#[test]
fn underflow_fixedu128() {
	type Number = sp_runtime::FixedU128;
	fn make(x: f64) -> Number {
		Number::from_fraction(x)
	}
	helper(make);
}

#[test]
fn underflow_rational_big() {
	type Number = num_rational::BigRational;
	fn make(x: f64) -> Number {
		Number::from_float(x).unwrap()
	}
	helper(make);
}

#[test]
fn underflow_rational_64() {
	type Number = num_rational::Rational64;
	fn make(x: f64) -> Number {
		Number::approximate_float(x).unwrap()
	}
	helper(make);
}

#[test]
fn underflow_bigfixed() {
	type Number = bigfixed::BigFixed;
	fn make(x: f64) -> Number {
		bigfixed::BigFixed::from(x)
	}
	helper(make);
}

#[test]
fn underflow_bigdecimal() {
	type Number = bigdecimal::BigDecimal;
	fn make(x: f64) -> Number {
		use bigdecimal::FromPrimitive as _;
		Number::from_f64(x).unwrap()
	}
	helper(make);
}

#[test]
fn underflow_native_f64() {
	type Number = f64;
	fn make(x: f64) -> Number {
		x
	}
	helper(make);
}
