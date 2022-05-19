fn helper<T, F>(make: F)
where
	T: Clone
		+ std::fmt::Debug
		+ std::ops::Add<Output = T>
		// + std::ops::Sub<Output = T>
		+ std::ops::Mul<Output = T>
		// + std::ops::Div<Output = T>
		+ PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(10.0_f64);
	let b = make(0.10_f64);
	let c = make(0.20_f64);
	let x = a.clone() * (b.clone() + c.clone());
	let y = a.clone() * b.clone() + a.clone() * c.clone();
	assert_eq!(x, y);

	let a = make(-9.0_f64);
	let b = make(0.001_f64);
	let y = make(-0.009_f64);
	let x = a.clone() * b.clone();
	assert_eq!(x, y);

	let a = make(0.1_f64);
	let b = make(0.2_f64);
	let y = make(0.3_f64);
	let x = a.clone() + b.clone();
	assert_eq!(x, y);

	let a = make(0.2_f64);
	let y = make(0.6_f64);
	let x = a.clone() + a.clone() + a.clone();
	assert_eq!(x, y);
}

#[test]
fn losses_fixedu128() {
	type Number = sp_runtime::FixedU128;
	fn make(x: f64) -> Number {
		Number::from_fraction(x)
	}
	helper(make);
}

#[test]
fn losses_rational_64() {
	type Number = num_rational::Rational64;
	fn make(x: f64) -> Number {
		Number::approximate_float(x).unwrap()
	}
	helper(make);
}

#[test]
fn losses_rational_big() {
	type Number = num_rational::BigRational;
	fn make(x: f64) -> Number {
		Number::from_float(x).unwrap()
	}
	helper(make);
}

#[test]
fn losses_bigfixed() {
	type Number = bigfixed::BigFixed;
	fn make(x: f64) -> Number {
		bigfixed::BigFixed::from(x)
	}
	helper(make);
}

#[test]
fn losses_bigdecimal() {
	type Number = bigdecimal::BigDecimal;
	fn make(x: f64) -> Number {
		use bigdecimal::FromPrimitive as _;
		Number::from_f64(x).unwrap()
	}
	helper(make);
}

#[test]
fn losses_native_f64() {
	type Number = f64;
	fn make(x: f64) -> Number {
		x
	}
	helper(make);
}
