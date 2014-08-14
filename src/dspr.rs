// dspr.rs  A rust DSP library
// Copyright (c) Nick Twyman, 2014
// All rights reserved
extern crate num;
extern crate std;

mod signals {
	use num::complex::Complex;
	use std::num::from_int;
	use std::num::Zero;

	impl<T: ToPrimative + FromPrimative> Complex<T> {
	/// Calculate |self|
	#[inline]
	pub fn norm(&self) -> T {
		T::from_f64(Complex.new(self.re.to_f64(), self.im.to_f64()).norm()).unwrap()
	}
	

	pub trait DiscreteSignal<N: Num> {
		fn get(&self, n: int) -> Complex<N>;
		fn range(&self) -> (int, int);
		fn max_amplitude(&self) -> N;
	}

	pub struct Delta<N> {
		offset: int,
		amplitude: Complex<N>
	}

	impl<N: Zero> DiscreteSignal<N> for Delta<N> {
		fn get(&self, n: int) -> Complex<N> {
			if n == self.offset { self.amplitude }
			else { Complex{re: Zero::zero(), im: Zero::zero()} }
		}

		fn range(&self) -> (int, int) {
			(self.offset, self.offset)
		}

		fn max_amplitude(&self) -> Some<N> {
			match *self {
				Delta<FloatMath>(_,_) => {
					self.max_amplitude.norm()
				}
				Delta<ToPrimative+FromPrimative>(re, im) => {
					re.to_
				}
			}
		}
	}

	pub struct Step<N> {
		offset: int,
		amplitude: Complex<N>		
	}

	impl<N: Zero + FloatMath> DiscreteSignal<N> for Step<N> {
		fn get(& self, n: int) -> Complex<N> {
			if n >= self.offset { self.amplitude }
			else { Complex{re: Zero::zero(), im: Zero::zero()} }
		}

		fn range(&self) -> (int, int) {
			(self.offset, ::std::int::MAX)
		}

		fn max_amplitude(&self) -> N {
			return self.amplitude.norm()
		}
	}



	#[cfg(test)]
	mod test {
		use signals;
		use num::complex::Complex;

		fn delta_test<N>(delta: &signals::DiscreteSignal<N>, offset: int, scale: Complex<N>) {

			let (min, max) = delta.range();
			assert!(min == offset);
			assert!(max == offset);

			assert!(delta.max_amplitude() == scale.norm());
			for i in range(-1000, 1000) {
				let val = delta.get(i);
				if i == offset {
					assert!(val == scale)
				} else {
					assert!(val == 0)
				}
			}
		}

		#[test]
		fn test_delta() {
			let delta = signals::Delta{offset: 13, amplitude:(2, 3)};
			delta_test(delta, 13, (2, 3));
			let minus_delta = signals::Delta{offset: -23, amplitude: (-3.4f64, 2f64)};
			delta_test(minus_delta, -23, (-3.4f64, 2f64));
		}
		
	}
}