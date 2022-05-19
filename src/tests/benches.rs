#[allow(dead_code)]
fn bench_helper<T, F>(make: F)
where
	T: Clone
		+ std::fmt::Debug
		+ std::ops::Add<Output = T>
		+ std::ops::Sub<Output = T>
		+ std::ops::Mul<Output = T>
		+ std::ops::Div<Output = T>
		+ PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(4.3_f64);
	let b = make(2.5_f64);
	let sum = make(06.80_f64);
	let sub = make(01.80_f64);
	let mul = make(10.75_f64);
	let div = make(01.72_f64);
	dbg!(&sum, &sub, &mul, &div, &a, &b);
	// sum
	let x = a.clone() + b.clone();
	assert_eq!(x, sum);
	// sub
	let x = a.clone() - b.clone();
	assert_eq!(x, sub);
	// mul
	let x = a.clone() * b.clone();
	assert_eq!(x, mul);
	// div
	let x = a.clone() / b.clone();
	assert_eq!(x, div);
}

fn bench_helper_div<T, F>(make: F) -> (T, T)
where
	T: Clone + std::fmt::Debug + std::ops::Div<Output = T> + PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(4.3_f64);
	let b = make(2.5_f64);
	let div = make(01.72_f64);
	// div
	let x = a.clone() / b.clone();
	(x, div)
}

fn bench_helper_mul<T, F>(make: F) -> (T, T)
where
	T: Clone + std::fmt::Debug + std::ops::Mul<Output = T> + PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(4.3_f64);
	let b = make(2.5_f64);
	let mul = make(10.75_f64);
	// mul
	let x = a.clone() * b.clone();
	(x, mul)
}

fn bench_helper_sub<T, F>(make: F) -> (T, T)
where
	T: Clone + std::fmt::Debug + std::ops::Sub<Output = T> + PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(4.3_f64);
	let b = make(2.5_f64);
	let sub = make(01.80_f64);
	// sub
	let x = a.clone() - b.clone();
	(x, sub)
}

fn bench_helper_sum<T, F>(make: F) -> (T, T)
where
	T: Clone + std::fmt::Debug + std::ops::Add<Output = T> + PartialEq,
	F: Fn(f64) -> T,
{
	let a = make(4.3_f64);
	let b = make(2.5_f64);
	let sum = make(06.80_f64);
	// sum
	let x = a.clone() + b.clone();
	(x, sum)
}

#[bench]
fn bench_fixedu128(bench: &mut test::Bencher) {
	type Number = sp_runtime::FixedU128;
	fn make(x: f64) -> Number {
		Number::from_fraction(x)
	}
	bench.iter(|| {
		bench_helper_sum(make);
		bench_helper_sub(make);
		bench_helper_mul(make);
		bench_helper_div(make);
	});
}

#[bench]
fn bench_rational_64(bench: &mut test::Bencher) {
	type Number = num_rational::Rational64;
	fn make(x: f64) -> Number {
		Number::approximate_float(x).unwrap()
	}
	bench.iter(|| {
		bench_helper_sum(make);
		bench_helper_sub(make);
		bench_helper_mul(make);
		bench_helper_div(make);
	});
}

#[bench]
fn bench_rational_big(bench: &mut test::Bencher) {
	type Number = num_rational::BigRational;
	fn make(x: f64) -> Number {
		Number::from_float(x).unwrap()
	}
	bench.iter(|| {
		bench_helper_sum(make);
		bench_helper_sub(make);
		bench_helper_mul(make);
		bench_helper_div(make);
	});
}

#[bench]
fn bench_bigfixed(bench: &mut test::Bencher) {
	type Number = bigfixed::BigFixed;
	fn make(x: f64) -> Number {
		bigfixed::BigFixed::from(x)
	}
	bench.iter(|| {
		bench_helper_sum(make);
		bench_helper_sub(make);
		bench_helper_mul(make);
		// bench_helper_div(make);
	});
}

#[bench]
fn bench_bigdecimal(bench: &mut test::Bencher) {
	type Number = bigdecimal::BigDecimal;
	fn make(x: f64) -> Number {
		use bigdecimal::FromPrimitive as _;
		Number::from_f64(x).unwrap()
	}
	bench.iter(|| {
		bench_helper_sum(make);
		bench_helper_sub(make);
		bench_helper_mul(make);
		// bench_helper_div(make);
	});
}

#[bench]
fn bench_native_f64(bench: &mut test::Bencher) {
	type Number = f64;
	fn make(x: f64) -> Number {
		x
	}
	bench.iter(|| {
		bench_helper_sum(make);
		bench_helper_sub(make);
		bench_helper_mul(make);
		// bench_helper_div(make);
	});
}
