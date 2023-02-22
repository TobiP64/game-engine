// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![warn(clippy::all)]
#![allow(
    clippy::many_single_char_names,
    clippy::field_reassign_with_default,
    clippy::from_over_into,
    clippy::too_many_arguments
)]

use core::ops::*;

pub use self::{
    vec::*,
    quat::*,
    mat::*,
    curves2d::*,
    shapes2d::*,
    shapes3d::*,
    types::*,
    ops::*,
    poly::*,
    misc::*,
    sdf::*,
    pbr::*,
};

pub mod vec {
    #![allow(clippy::len_without_is_empty)]

    use super::*;

    macro_rules! vec_impl {
	    ( $name:ident; $( $xn:ident: $x:tt ),* ) => {

	  		impl<T> $name<T> {
				pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> $name<U> {
					$name( $( f(self.$x) ),* )
				}

				pub fn for_each(self, mut f: impl FnMut(T)) {
					$( f(self.$x); )*
				}

				pub fn any(&self, f: impl Fn(&T) -> bool) -> bool {
					$( f(&self.$x) )||*
				}

				pub fn all(&self, f: impl Fn(&T) -> bool) -> bool {
					$( f(&self.$x) )&&*
				}

				pub fn min(self, other: Self) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x < other.$x { self.$x } else { other.$x } ),* )
				}

				pub fn mins(self, other: T) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x < other { self.$x } else { other } ),* )
				}

				pub fn mincw(self) -> T where T: PartialOrd + Copy {
					let mut v = self.0;
					$( if self.$x < v { v = self.$x } )*
					v
				}

				pub fn max(self, other: Self) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x > other.$x { self.$x } else { other.$x } ),* )
				}

				pub fn maxs(self, other: T) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x > other { self.$x } else { other } ),* )
				}

				pub fn maxcw(self) -> T where T: PartialOrd + Copy {
					let mut v = self.0;
					$( if self.$x > v { v = self.$x } )*
					v
				}

				pub fn clamp(self, lower: Self, upper: Self) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x < lower.$x { lower.$x } else if self.$x > upper.$x { upper.$x } else { self.$x } ),* )
				}

				pub fn clamps(self, lower: T, upper: T) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x < lower { lower } else if self.$x > upper { upper } else { self.$x } ),* )
				}

				pub fn clampvs(self, lower: Self, upper: T) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x < lower.$x { lower.$x } else if self.$x > upper { upper } else { self.$x } ),* )
				}

				pub fn clampsv(self, lower: T, upper: Self) -> Self where T: PartialOrd + Copy {
					Self( $( if self.$x < lower { lower } else if self.$x > upper.$x { upper.$x } else { self.$x } ),* )
				}

				pub fn dot(self, other: Self) -> T where T: Add<Output = T> + Mul<Output = T> + Copy + Default {
					T::default() $( + self.$x * other.$x )*
				}

				pub fn mix(self, a: Self, b: Self) -> Self where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {
					Self( $( a.$x + (b.$x - a.$x) * self.$x ),* )
				}

				pub fn mixs(self, a: T, b: T) -> Self where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {
					Self( $( a + (b - a) * self.$x ),* )
				}

				pub fn addc(self) -> T where T: Add<Output = T> + Sub<Output = T> + Copy {
					self.0 - self.0 $( + self.$x )*
				}

				pub fn mulc(self) -> T where T: Mul<Output = T> + Div<Output = T> + Copy {
					self.0 / self.0 $( * self.$x )*
				}

				pub fn cast<U>(self) -> $name<U> where $name<U>: From<Self> {
					self.into()
				}
			}

			impl<T: $crate::Float> $name<T> {
				pub fn abs(self) -> Self {
					Self( $( self.$x.abs() ),* )
				}

				pub fn powf(self, n: Self) -> Self {
					Self( $( self.$x.powf(n.$x) ),* )
				}

				pub fn powi(self, n: $name<i32>) -> Self {
					Self( $( self.$x.powi(n.$x) ),* )
				}

				pub fn sqrt(self) -> Self {
					Self( $( self.$x.sqrt() ),* )
				}

				pub fn cbrt(self) -> Self {
					Self( $( self.$x.cbrt() ),* )
				}

				pub fn log(self, base: Self) -> Self {
					Self( $( self.$x.log(base.$x) ),* )
				}

				pub fn len2(self) -> T {
					self.dot(self)
				}

				pub fn len(self) -> T {
					self.dot(self).sqrt()
				}

				pub fn normalize(self, len: T) -> Self {
					self * (len / self.len())
				}

				pub fn distance2(self, other: Self) -> T {
					(other - self).len2()
				}

				pub fn distance(self, other: Self) -> T {
					(other - self).len()
				}

				pub fn angle(self, other: Self) -> T {
					(self.dot(other) / (self.len() * other.len())).acos()
				}

				pub fn reflect(self, n: Self) -> Self {
					n * T::val(2.0) * self.dot(n) - self
				}
			}

	    	impl $name<bool> {
				pub fn and(self) -> bool { $( self.$x )&&* }
				pub fn or(self) -> bool { $( self.$x )||* }
				pub fn xor(self) -> bool { $( self.$x )^* }
			}

			impl<T: Copy> From<T> for $name<T> {
				fn from(v: T) -> Self {
					Self( $( vec_impl!(@ignore v; $xn),  )* )
				}
			}

			impl<T> From<[T; 0 $( + vec_impl!(@ignore 1; $xn) )* ]> for $name<T> {
				fn from([ $( $xn, )* ]: [T; 0 $( + vec_impl!(@ignore 1; $xn) )* ]) -> Self {
					Self( $( $xn, )* )
				}
			}

			impl<T> From<( $( vec_impl!(@ignore T; $xn), )* )> for $name<T> {
				fn from(( $( $xn, )* ): ( $( vec_impl!(@ignore T; $xn), )* )) -> Self {
					Self( $( $xn, )* )
				}
			}

			impl<T: Add + Copy> Add for $name<T> {
				type Output = $name<T::Output>;

				fn add(self, rhs: Self) -> Self::Output {
					$name( $( self.$x + rhs.$x ),* )
				}
			}

			impl<T: Sub + Copy> Sub for $name<T> {
				type Output = $name<T::Output>;

				fn sub(self, rhs: Self) -> Self::Output {
					$name( $( self.$x - rhs.$x ),* )
				}
			}

			impl<T: Mul + Copy> Mul for $name<T> {
				type Output = $name<T::Output>;

				fn mul(self, rhs: Self) -> Self::Output {
					$name( $( self.$x * rhs.$x ),* )
				}
			}

			impl<T: Mul + Copy> Mul<T> for $name<T> {
				type Output = $name<T::Output>;

				fn mul(self, rhs: T) -> Self::Output {
					$name( $( self.$x * rhs ),* )
				}
			}

			impl<T: Div + Copy> Div for $name<T> {
				type Output = $name<T::Output>;

				fn div(self, rhs: Self) -> Self::Output {
					$name( $( self.$x / rhs.$x ),* )
				}
			}

			impl<T: Div + Copy> Div<T> for $name<T> {
				type Output = $name<T::Output>;

				fn div(self, rhs: T) -> Self::Output {
					$name( $( self.$x / rhs ),* )
				}
			}

			impl<T: Rem + Copy> Rem for $name<T> {
				type Output = $name<T::Output>;

				fn rem(self, rhs: Self) -> Self::Output {
					$name( $( self.$x % rhs.$x ),* )
				}
			}

			impl<T: Rem + Copy> Rem<T> for $name<T> {
				type Output = $name<T::Output>;

				fn rem(self, rhs: T) -> Self::Output {
					$name( $( self.$x % rhs ),* )
				}
			}

			impl<T: Neg + Copy> Neg for $name<T> {
				type Output = $name<T::Output>;

				fn neg(self) -> Self::Output {
					$name( $( -self.$x ),* )
				}
			}

			impl<T: Not + Copy> Not for $name<T> {
				type Output = $name<T::Output>;

				fn not(self) -> Self::Output {
					$name( $( !self.$x ),* )
				}
			}

			impl<T: BitAnd + Copy> BitAnd for $name<T> {
				type Output = $name<T::Output>;

				fn bitand(self, rhs: Self) -> Self::Output {
					$name( $( self.$x & rhs.$x ),* )
				}
			}

			impl<T: BitAnd + Copy> BitAnd<T> for $name<T> {
				type Output = $name<T::Output>;

				fn bitand(self, rhs: T) -> Self::Output {
					$name( $( self.$x & rhs ),* )
				}
			}

			impl<T: BitOr + Copy> BitOr for $name<T> {
				type Output = $name<T::Output>;

				fn bitor(self, rhs: Self) -> Self::Output {
					$name( $( self.$x | rhs.$x ),* )
				}
			}

			impl<T: BitOr + Copy> BitOr<T> for $name<T> {
				type Output = $name<T::Output>;

				fn bitor(self, rhs: T) -> Self::Output {
					$name( $( self.$x | rhs ),* )
				}
			}

			impl<T: BitXor + Copy> BitXor for $name<T> {
				type Output = $name<T::Output>;

				fn bitxor(self, rhs: Self) -> Self::Output {
					$name( $( self.$x ^ rhs.$x ),* )
				}
			}

			impl<T: BitXor + Copy> BitXor<T> for $name<T> {
				type Output = $name<T::Output>;

				fn bitxor(self, rhs: T) -> Self::Output {
					$name( $( self.$x ^ rhs ),* )
				}
			}

			impl<T: Shl + Copy> Shl for $name<T> {
				type Output = $name<T::Output>;

				fn shl(self, rhs: Self) -> Self::Output {
					$name( $( self.$x << rhs.$x ),* )
				}
			}

			impl<T: Shl + Copy> Shl<T> for $name<T> {
				type Output = $name<T::Output>;

				fn shl(self, rhs: T) -> Self::Output {
					$name( $( self.$x << rhs ),* )
				}
			}

			impl<T: Shr + Copy> Shr for $name<T> {
				type Output = $name<T::Output>;

				fn shr(self, rhs: Self) -> Self::Output {
					$name( $( self.$x >> rhs.$x ),* )
				}
			}

			impl<T: Shr + Copy> Shr<T> for $name<T> {
				type Output = $name<T::Output>;

				fn shr(self, rhs: T) -> Self::Output {
					$name( $( self.$x >> rhs ),* )
				}
			}

			impl<T: AddAssign + Copy> AddAssign for $name<T> {
				fn add_assign(&mut self, rhs: Self) {
					$( self.$x += rhs.$x; )*
				}
			}

			impl<T: SubAssign + Copy> SubAssign for $name<T> {
				fn sub_assign(&mut self, rhs: Self) {
					$( self.$x -= rhs.$x; )*
				}
			}

			impl<T: MulAssign + Copy> MulAssign for $name<T> {
				fn mul_assign(&mut self, rhs: Self) {
					$( self.$x *= rhs.$x; )*
				}
			}

			impl<T: MulAssign + Copy> MulAssign<T> for $name<T> {
				fn mul_assign(&mut self, rhs: T) {
					$( self.$x *= rhs; )*
				}
			}

			impl<T: DivAssign + Copy> DivAssign for $name<T> {
				fn div_assign(&mut self, rhs: Self) {
					$( self.$x /= rhs.$x; )*
				}
			}

			impl<T: DivAssign + Copy> DivAssign<T> for $name<T> {
				fn div_assign(&mut self, rhs: T) {
					$( self.$x /= rhs; )*
				}
			}

			impl<T: RemAssign + Copy> RemAssign for $name<T> {
				fn rem_assign(&mut self, rhs: Self) {
					$( self.$x %= rhs.$x; )*
				}
			}

			impl<T: RemAssign + Copy> RemAssign<T> for $name<T> {
				fn rem_assign(&mut self, rhs: T) {
					$( self.$x %= rhs; )*
				}
			}

			impl<T: BitAndAssign + Copy> BitAndAssign for $name<T> {
				fn bitand_assign(&mut self, rhs: Self) {
					$( self.$x &= rhs.$x; )*
				}
			}

			impl<T: BitAndAssign + Copy> BitAndAssign<T> for $name<T> {
				fn bitand_assign(&mut self, rhs: T) {
					$( self.$x &= rhs; )*
				}
			}

			impl<T: BitOrAssign + Copy> BitOrAssign for $name<T> {
				fn bitor_assign(&mut self, rhs: Self) {
					$( self.$x |= rhs.$x; )*
				}
			}

			impl<T: BitOrAssign + Copy> BitOrAssign<T> for $name<T> {
				fn bitor_assign(&mut self, rhs: T) {
					$( self.$x |= rhs; )*
				}
			}

			impl<T: BitXorAssign + Copy> BitXorAssign for $name<T> {
				fn bitxor_assign(&mut self, rhs: Self) {
					$( self.$x ^= rhs.$x; )*
				}
			}

			impl<T: BitXorAssign + Copy> BitXorAssign<T> for $name<T> {
				fn bitxor_assign(&mut self, rhs: T) {
					$( self.$x ^= rhs; )*
				}
			}

			impl<T: ShlAssign + Copy> ShlAssign for $name<T> {
				fn shl_assign(&mut self, rhs: Self) {
					$( self.$x <<= rhs.$x; )*
				}
			}

			impl<T: ShlAssign + Copy> ShlAssign<T> for $name<T> {
				fn shl_assign(&mut self, rhs: T) {
					$( self.$x <<= rhs; )*
				}
			}

			impl<T: ShrAssign + Copy> ShrAssign for $name<T> {
				fn shr_assign(&mut self, rhs: Self) {
					$( self.$x >>= rhs.$x; )*
				}
			}

			impl<T: ShrAssign + Copy> ShrAssign<T> for $name<T> {
				fn shr_assign(&mut self, rhs: T) {
					$( self.$x >>= rhs; )*
				}
			}

			impl<A: Default> core::iter::FromIterator<A> for $name<A> {
				fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
					let mut iter = iter.into_iter();
					// Self( $( iter.next().unwrap() ),* )
					let mut self_: Self = Self::default();
					$( self_.$x = iter.next().unwrap(); )*
					self_
				}
			}

			impl<T: Clone> Iterator for Iter<$name<T>> {
				type Item = T;

				fn next(&mut self) -> Option<Self::Item> {
					self.1 += 1;
					match self.1 - 1 {
						$( $x => Some((self.0).$x.clone()), )*
						_ => None,
					}
				}
			}

			impl<T: Clone> core::iter::IntoIterator for $name<T> {
				type Item = T;
				type IntoIter = Iter<$name<T>>;

				fn into_iter(self) -> Self::IntoIter {
					Iter(self, 0)
				}
			}

			impl<'a, T> Iterator for IterRef<'a, $name<T>> {
				type Item = &'a T;

				fn next(&mut self) -> Option<Self::Item> {
					self.1 += 1;
					match self.1 - 1 {
						$( $x => Some(&(self.0).$x), )*
						_ => None,
					}
				}
			}

			impl<'a, T> core::iter::IntoIterator for &'a $name<T> {
				type Item = &'a T;
				type IntoIter = IterRef<'a, $name<T>>;

				fn into_iter(self) -> Self::IntoIter {
					IterRef(self, 0)
				}
			}

			/*impl<'a, T> Iterator for IterMut<'a, $name<T>> {
				type Item = &'a mut T;

				fn next(&mut self) -> Option<Self::Item> {
					self.1 += 1;
					match self.1 - 1 {
						$( $x => Some(&mut*(&mut (self.0).$x as *mut T)), )*
						_ => None,
					}
				}
			}

			impl<'a, T> core::iter::IntoIterator for &'a mut $name<T> {
				type Item = &'a mut T;
				type IntoIter = IterMut<'a, $name<T>>;

				fn into_iter(self) -> Self::IntoIter {
					IterMut(self, 0)
				}
			}*/

			impl<T> Index<usize> for $name<T> {
				type Output = T;

				fn index(&self, index: usize) -> &Self::Output {
					match index {
						$( $x => &self.$x, )*
						_ => panic!()
					}
				}
			}

			impl<T> IndexMut<usize> for $name<T> {
				fn index_mut(&mut self, index: usize) -> &mut Self::Output {
					match index {
						$( $x => &mut self.$x, )*
						_ => panic!()
					}
				}
			}

			vec_impl!(@pcast $name, f32, usize; $( $xn, )* );
			vec_impl!(@pcast $name, f64, usize; $( $xn, )* );
			vec_impl!(@pcast $name, f32, isize; $( $xn, )* );
			vec_impl!(@pcast $name, f64, isize; $( $xn, )* );
	    };
	    (@pcast $name:ident, $ty0:ty, $ty1:ty; $( $xn:ident, )* ) => {
	    	impl From<$name<$ty0>> for $name<$ty1> {
	    		fn from($name( $( $xn, )* ): $name<$ty0>) -> Self {
	    			Self( $( $xn as _, )* )
	    		}
	    	}

	    	impl From<$name<$ty1>> for $name<$ty0> {
	    		fn from($name( $( $xn, )* ): $name<$ty1>) -> Self {
	    			Self( $( $xn as _, )* )
	    		}
	    	}
	    };
	    (@ignore $tt0:tt; $tt1:tt ) => { $tt0 };
	}

    #[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
    #[cfg_attr(target_arch = "spirv", repr(simd))]
    pub struct Vec2<T>(pub T, pub T);

    impl<T: Copy> Vec2<T> {
        pub fn x(self) -> T { self.0 }
        pub fn y(self) -> T { self.1 }
        pub fn xx(self) -> Vec2<T> { Vec2(self.0, self.0) }
        pub fn xy(self) -> Vec2<T> { Vec2(self.0, self.1) }
        pub fn yx(self) -> Vec2<T> { Vec2(self.1, self.0) }
        pub fn yy(self) -> Vec2<T> { Vec2(self.1, self.1) }
        pub fn xxx(self) -> Vec3<T> { Vec3(self.0, self.0, self.0) }
        pub fn xxy(self) -> Vec3<T> { Vec3(self.0, self.0, self.1) }
        pub fn xyx(self) -> Vec3<T> { Vec3(self.0, self.1, self.0) }
        pub fn xyy(self) -> Vec3<T> { Vec3(self.0, self.1, self.1) }
        pub fn yxx(self) -> Vec3<T> { Vec3(self.1, self.0, self.0) }
        pub fn yxy(self) -> Vec3<T> { Vec3(self.1, self.0, self.1) }
        pub fn yyx(self) -> Vec3<T> { Vec3(self.1, self.1, self.0) }
        pub fn yyy(self) -> Vec3<T> { Vec3(self.1, self.1, self.1) }
    }

    vec_impl!(Vec2; v0: 0, v1: 1);

    #[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
    #[cfg_attr(target_arch = "spirv", repr(simd))]
    pub struct Vec3<T>(pub T, pub T, pub T);

    impl<T: Copy> Vec3<T> {
        pub fn x(self) -> T { self.0 }
        pub fn y(self) -> T { self.1 }
        pub fn z(self) -> T { self.2 }
        pub fn xx(self) -> Vec2<T> { Vec2(self.0, self.0) }
        pub fn xy(self) -> Vec2<T> { Vec2(self.0, self.1) }
        pub fn xz(self) -> Vec2<T> { Vec2(self.0, self.2) }
        pub fn yx(self) -> Vec2<T> { Vec2(self.1, self.0) }
        pub fn yy(self) -> Vec2<T> { Vec2(self.1, self.1) }
        pub fn yz(self) -> Vec2<T> { Vec2(self.1, self.2) }
        pub fn zx(self) -> Vec2<T> { Vec2(self.2, self.0) }
        pub fn zy(self) -> Vec2<T> { Vec2(self.2, self.1) }
        pub fn zz(self) -> Vec2<T> { Vec2(self.2, self.2) }
        pub fn xxx(self) -> Vec3<T> { Vec3(self.0, self.0, self.0) }
        pub fn xxy(self) -> Vec3<T> { Vec3(self.0, self.0, self.1) }
        pub fn xxz(self) -> Vec3<T> { Vec3(self.0, self.0, self.2) }
        pub fn xyx(self) -> Vec3<T> { Vec3(self.0, self.1, self.0) }
        pub fn xyy(self) -> Vec3<T> { Vec3(self.0, self.1, self.1) }
        pub fn xyz(self) -> Vec3<T> { Vec3(self.0, self.1, self.2) }
        pub fn xzx(self) -> Vec3<T> { Vec3(self.0, self.2, self.0) }
        pub fn xzy(self) -> Vec3<T> { Vec3(self.0, self.2, self.1) }
        pub fn xzz(self) -> Vec3<T> { Vec3(self.0, self.2, self.2) }
        pub fn yxx(self) -> Vec3<T> { Vec3(self.1, self.0, self.0) }
        pub fn yxy(self) -> Vec3<T> { Vec3(self.1, self.0, self.1) }
        pub fn yxz(self) -> Vec3<T> { Vec3(self.1, self.0, self.2) }
        pub fn yyx(self) -> Vec3<T> { Vec3(self.1, self.1, self.0) }
        pub fn yyy(self) -> Vec3<T> { Vec3(self.1, self.1, self.1) }
        pub fn yyz(self) -> Vec3<T> { Vec3(self.1, self.1, self.2) }
        pub fn yzx(self) -> Vec3<T> { Vec3(self.1, self.2, self.0) }
        pub fn yzy(self) -> Vec3<T> { Vec3(self.1, self.2, self.1) }
        pub fn yzz(self) -> Vec3<T> { Vec3(self.1, self.2, self.2) }
        pub fn zxx(self) -> Vec3<T> { Vec3(self.2, self.0, self.0) }
        pub fn zxy(self) -> Vec3<T> { Vec3(self.2, self.0, self.1) }
        pub fn zxz(self) -> Vec3<T> { Vec3(self.2, self.0, self.2) }
        pub fn zyx(self) -> Vec3<T> { Vec3(self.2, self.1, self.0) }
        pub fn zyy(self) -> Vec3<T> { Vec3(self.2, self.1, self.1) }
        pub fn zyz(self) -> Vec3<T> { Vec3(self.2, self.1, self.2) }
        pub fn zzx(self) -> Vec3<T> { Vec3(self.2, self.2, self.0) }
        pub fn zzy(self) -> Vec3<T> { Vec3(self.2, self.2, self.1) }
        pub fn zzz(self) -> Vec3<T> { Vec3(self.2, self.2, self.2) }

        pub fn cross(self, rhs: Self) -> Self where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> {
            Self(self.1 * rhs.2 - self.2 * rhs.1,
                 self.2 * rhs.0 - self.0 * rhs.2,
                 self.0 * rhs.1 - self.1 * rhs.0)
        }
    }

    impl<T> From<(Vec2<T>, T)> for Vec3<T> {
        fn from((Vec2(v0, v1), v2): (Vec2<T>, T)) -> Self {
            Vec3(v0, v1, v2)
        }
    }

    impl<T> From<(T, Vec2<T>)> for Vec3<T> {
        fn from((v0, Vec2(v1, v2)): (T, Vec2<T>)) -> Self {
            Vec3(v0, v1, v2)
        }
    }

    impl<T> Into<Vec2<T>> for Vec3<T> {
        fn into(self) -> Vec2<T> {
            Vec2(self.0, self.1)
        }
    }

    vec_impl!(Vec3; v0: 0, v1: 1, v2: 2);

    #[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
    #[cfg_attr(target_arch = "spirv", repr(simd))]
    pub struct Vec4<T>(pub T, pub T, pub T, pub T);

    impl<T: Copy> Vec4<T> {
        pub fn xx(self) -> Vec2<T> { Vec2(self.0, self.0) }
        pub fn xy(self) -> Vec2<T> { Vec2(self.0, self.1) }
        pub fn xz(self) -> Vec2<T> { Vec2(self.0, self.2) }
        pub fn yx(self) -> Vec2<T> { Vec2(self.1, self.0) }
        pub fn yy(self) -> Vec2<T> { Vec2(self.1, self.1) }
        pub fn yz(self) -> Vec2<T> { Vec2(self.1, self.2) }
        pub fn zx(self) -> Vec2<T> { Vec2(self.2, self.0) }
        pub fn zy(self) -> Vec2<T> { Vec2(self.2, self.1) }
        pub fn zz(self) -> Vec2<T> { Vec2(self.2, self.2) }
        pub fn xxx(self) -> Vec3<T> { Vec3(self.0, self.0, self.0) }
        pub fn xxy(self) -> Vec3<T> { Vec3(self.0, self.0, self.1) }
        pub fn xxz(self) -> Vec3<T> { Vec3(self.0, self.0, self.2) }
        pub fn xyx(self) -> Vec3<T> { Vec3(self.0, self.1, self.0) }
        pub fn xyy(self) -> Vec3<T> { Vec3(self.0, self.1, self.1) }
        pub fn xyz(self) -> Vec3<T> { Vec3(self.0, self.1, self.2) }
        pub fn xzx(self) -> Vec3<T> { Vec3(self.0, self.2, self.0) }
        pub fn xzy(self) -> Vec3<T> { Vec3(self.0, self.2, self.1) }
        pub fn xzz(self) -> Vec3<T> { Vec3(self.0, self.2, self.2) }
        pub fn yxx(self) -> Vec3<T> { Vec3(self.1, self.0, self.0) }
        pub fn yxy(self) -> Vec3<T> { Vec3(self.1, self.0, self.1) }
        pub fn yxz(self) -> Vec3<T> { Vec3(self.1, self.0, self.2) }
        pub fn yyx(self) -> Vec3<T> { Vec3(self.1, self.1, self.0) }
        pub fn yyy(self) -> Vec3<T> { Vec3(self.1, self.1, self.1) }
        pub fn yyz(self) -> Vec3<T> { Vec3(self.1, self.1, self.2) }
        pub fn yzx(self) -> Vec3<T> { Vec3(self.1, self.2, self.0) }
        pub fn yzy(self) -> Vec3<T> { Vec3(self.1, self.2, self.1) }
        pub fn yzz(self) -> Vec3<T> { Vec3(self.1, self.2, self.2) }
        pub fn zxx(self) -> Vec3<T> { Vec3(self.2, self.0, self.0) }
        pub fn zxy(self) -> Vec3<T> { Vec3(self.2, self.0, self.1) }
        pub fn zxz(self) -> Vec3<T> { Vec3(self.2, self.0, self.2) }
        pub fn zyx(self) -> Vec3<T> { Vec3(self.2, self.1, self.0) }
        pub fn zyy(self) -> Vec3<T> { Vec3(self.2, self.1, self.1) }
        pub fn zyz(self) -> Vec3<T> { Vec3(self.2, self.1, self.2) }
        pub fn zzx(self) -> Vec3<T> { Vec3(self.2, self.2, self.0) }
        pub fn zzy(self) -> Vec3<T> { Vec3(self.2, self.2, self.1) }
        pub fn zzz(self) -> Vec3<T> { Vec3(self.2, self.2, self.2) }

        pub fn normalize_vec3(self) -> Self where T: Float {
            Self::from((self.xyz().normalize(T::val(1f32)), self.3))
        }

        pub fn cross_vec3(self, rhs: Vec3<T>) -> Self where T: Float {
            Self::from((self.xyz().cross(rhs), self.3))
        }
    }

    impl<T> From<(Vec2<T>, T, T)> for Vec4<T> {
        fn from((Vec2(v0, v1), v2, v3): (Vec2<T>, T, T)) -> Self {
            Self(v0, v1, v2, v3)
        }
    }

    impl<T> From<(T, Vec2<T>, T)> for Vec4<T> {
        fn from((v0, Vec2(v1, v2), v3): (T, Vec2<T>, T)) -> Self {
            Self(v0, v1, v2, v3)
        }
    }

    impl<T> From<(T, T, Vec2<T>)> for Vec4<T> {
        fn from((v0, v1, Vec2(v2, v3)): (T, T, Vec2<T>)) -> Self {
            Self(v0, v1, v2, v3)
        }
    }

    impl<T> From<(Vec3<T>, T)> for Vec4<T> {
        fn from((Vec3(v0, v1, v2), v3): (Vec3<T>, T)) -> Self {
            Self(v0, v1, v2, v3)
        }
    }

    impl<T> From<(T, Vec3<T>)> for Vec4<T> {
        fn from((v0, Vec3(v1, v2, v3)): (T, Vec3<T>)) -> Self {
            Self(v0, v1, v2, v3)
        }
    }

    impl<T> Into<Vec3<T>> for Vec4<T> {
        fn into(self) -> Vec3<T> {
            Vec3(self.0, self.1, self.2)
        }
    }

    vec_impl!(Vec4; v0: 0, v1: 1, v2: 2, v3: 3);

    pub struct Iter<T>(T, usize);
    pub struct IterRef<'a, T>(&'a T, usize);
    pub struct IterMut<'a, T>(&'a mut T, usize);

    pub type Vec2f32 = Vec2<f32>;
    pub type Vec2f64 = Vec2<f64>;
    pub type Vec2i32 = Vec2<i32>;
    pub type Vec2i64 = Vec2<i64>;
    pub type Vec2u32 = Vec2<u32>;
    pub type Vec2u64 = Vec2<u64>;
}

pub mod quat {
    use super::*;

    pub type Quat<T> = Vec4<T>;
    pub type Quat32 = Quat<f32>;
    pub type Quat64 = Quat<f64>;

    impl<T: Float> Quat<T> {
        pub fn from_rotation_x(angle: T) -> Self {
            Self((angle * T::val(0.5)).sin(), T::val(0.0), T::val(0.0), (angle * T::val(0.5)).cos())
        }

        pub fn from_rotation_y(angle: T) -> Self {
            Self(T::val(0.0), (angle * T::val(0.5)).sin(), T::val(0.0), (angle * T::val(0.5)).cos())
        }

        pub fn from_rotation_z(angle: T) -> Self {
            Self(T::val(0.0), T::val(0.0), (angle * T::val(0.5)).sin(), (angle * T::val(0.5)).cos())
        }

        pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let inv_len = T::val(1.0) / axis.len();
            Self(axis.0 * inv_len * sin,
                 axis.1 * inv_len * sin,
                 axis.2 * inv_len * sin,
                 (angle * T::val(0.5)).cos())
        }

        pub fn to_axis_angle(self) -> (Vec3<T>, T) {
            let acos = self.3.acos();
            let inv_sqrt = T::val(1.0) / (T::val(1.0) - self.3 * self.3).sqrt();
            (Vec3(self.0 * inv_sqrt, self.1 * inv_sqrt, self.3 * inv_sqrt), acos + acos)
        }

        pub fn rotate(self, other: Self) -> Self {
            let (axis, angle) = other.to_axis_angle();
            self.rotate_axis(axis, angle)
        }

        pub fn rotate_local(self, _other: Self) -> Self {
            unimplemented!()
        }

        pub fn rotate_x_local(self, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let cos = (angle * T::val(0.5)).cos();
            Self(self.0 * cos + self.3 * sin,
                 self.1 * cos - self.2 * sin,
                 self.2 * cos + self.1 * sin,
                 self.3 * cos - self.0 * sin)
        }

        pub fn rotate_y_local(self, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let cos = (angle * T::val(0.5)).cos();
            Self(self.0 * cos + self.2 * sin,
                 self.1 * cos + self.3 * sin,
                 self.2 * cos - self.0 * sin,
                 self.3 * cos - self.1 * sin)
        }

        pub fn rotate_z_local(self, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let cos = (angle * T::val(0.5)).cos();
            Self(self.0 * cos - self.1 * sin,
                 self.1 * cos + self.0 * sin,
                 self.2 * cos + self.3 * sin,
                 self.3 * cos - self.2 * sin)
        }

        pub fn rotate_x(self, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let cos = (angle * T::val(0.5)).cos();
            Self(self.3 * sin + self.0 * cos,
                 self.1 * cos + self.2 * sin,
                 self.2 * cos - self.1 * sin,
                 self.3 * cos - self.0 * sin)
        }

        pub fn rotate_y(self, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let cos = (angle * T::val(0.5)).cos();
            Self(self.0 * cos - self.2 * sin,
                 self.3 * sin + self.1 * cos,
                 self.0 * sin + self.2 * cos,
                 self.3 * cos - self.1 * sin)
        }

        pub fn rotate_z(self, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let cos = (angle * T::val(0.5)).cos();
            Self(self.0 * cos + self.1 * sin,
                 self.1 * cos - self.0 * sin,
                 self.3 * sin + self.2 * cos,
                 self.3 * cos - self.2 * sin)
        }

        pub fn rotate_axis(self, axis: Vec3<T>, angle: T) -> Self {
            let sin = (angle * T::val(0.5)).sin();
            let inv_len = T::val(1.0) / axis.len();
            let rx = axis.0 * inv_len * sin;
            let ry = axis.1 * inv_len * sin;
            let rz = axis.2 * inv_len * sin;
            let rw = (angle * T::val(0.5)).cos();
            Self(self.3 * rx + self.0 * rw + self.1 * rz - self.2 * ry,
                 self.3 * ry + self.0 * rz + self.1 * rw - self.2 * rx,
                 self.3 * rz + self.0 * ry + self.1 * rx - self.2 * rw,
                 self.3 * rw + self.0 * rx + self.1 * ry - self.2 * rz)
        }
    }
}

pub mod mat {
    use super::*;

    macro_rules! mat_impl {
	    ( $name:ident, $vec:ident; $( $xn:ident: $x:tt ),* ) => {
	    	impl<T, U: Into<$vec<T>>> From<[U; 0 $( + mat_impl!(@ignore 1; $xn) )* ]> for $name<T> {
				fn from([ $( $xn, )* ]: [U; 0 $( + mat_impl!(@ignore 1; $xn) )* ]) -> Self {
					Self( $( $xn.into(), )* )
				}
			}

			impl<T, U: Into<$vec<T>>> From<( $( mat_impl!(@ignore U; $xn), )* )> for $name<T> {
				fn from(( $( $xn, )* ): ( $( mat_impl!(@ignore U; $xn), )* )) -> Self {
					Self( $( $xn.into(), )* )
				}
			}

			impl<T: Add<Output = U> + Copy, U> Add for $name<T> {
				type Output = $name<U>;

				fn add(self, rhs: Self) -> Self::Output {
					$name( $( self.$x + rhs.$x, )* )
				}
			}

			impl<T: Sub<Output = U> + Copy, U> Sub for $name<T> {
				type Output = $name<U>;

				fn sub(self, rhs: Self) -> Self::Output {
					$name( $( self.$x - rhs.$x, )* )
				}
			}

	    	impl<T> core::ops::Index<usize> for $name<T> {
				type Output = $vec<T>;

				fn index(&self, index: usize) -> &Self::Output {
					match index {
						$( $x => &self.$x, )*
						_ => panic!()
					}
				}
			}

			impl<T> core::ops::IndexMut<usize> for $name<T> {
				fn index_mut(&mut self, index: usize) -> &mut Self::Output {
					match index {
						$( $x => &mut self.$x, )*
						_ => panic!()
					}
				}
			}

			impl<T: AddAssign + Copy> AddAssign for $name<T> {
				fn add_assign(&mut self, rhs: Self) {
					$( self.$x += rhs.$x; )*
				}
			}

			impl<T: SubAssign + Copy> SubAssign for $name<T> {
				fn sub_assign(&mut self, rhs: Self) {
					$( self.$x -= rhs.$x; )*
				}
			}

			impl<T: Neg<Output = U> + Copy, U> Neg for $name<T> {
				type Output = $name<U>;

				fn neg(self) -> Self::Output {
					$name( $( -self.$x, )* )
				}
			}
	    };
	    (@ignore $tt0:tt; $tt1:tt ) => { $tt0 };
	}

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub struct Mat2<T>(pub Vec2<T>, pub Vec2<T>);

    impl Default for Mat2<f32> {
        fn default() -> Self {
            Self(Vec2(1f32, 0f32),
                 Vec2(0f32, 1f32))
        }
    }

    impl Default for Mat2<f64> {
        fn default() -> Self {
            Self(Vec2(1f64, 0f64),
                 Vec2(0f64, 1f64))
        }
    }

    impl<T> From<[T; 4]> for Mat2<T> {
        fn from([v0, v1, v2, v3]: [T; 4]) -> Self {
            Self(Vec2(v0, v1), Vec2(v2, v3))
        }
    }

    mat_impl!(Mat2, Vec2; v0: 0, v1: 1);

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub struct Mat3<T>(pub Vec3<T>, pub Vec3<T>, pub Vec3<T>);

    impl Default for Mat3<f32> {
        fn default() -> Self {
            Self(Vec3(1f32, 0f32, 0f32),
                 Vec3(0f32, 1f32, 0f32),
                 Vec3(0f32, 0f32, 1f32))
        }
    }

    impl Default for Mat3<f64> {
        fn default() -> Self {
            Self(Vec3(1f64, 0f64, 0f64),
                 Vec3(0f64, 1f64, 0f64),
                 Vec3(0f64, 0f64, 1f64))
        }
    }

    impl<T> From<[T; 9]> for Mat3<T> {
        fn from([v0, v1, v2, v3, v4, v5, v6, v7, v8]: [T; 9]) -> Self {
            Self(Vec3(v0, v1, v2), Vec3(v3, v4, v5), Vec3(v6, v7, v8))
        }
    }

    mat_impl!(Mat3, Vec3; v0: 0, v1: 1, v2: 2);

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub struct Mat4<T>(pub Vec4<T>, pub Vec4<T>, pub Vec4<T>, pub Vec4<T>);

    impl<T: Copy> Mat4<T> {
        pub fn from_translation(trans: Vec3<T>) -> Self where Self: Default {
            let mut mat = Self::default();
            (mat.3).0 = trans.0;
            (mat.3).1 = trans.1;
            (mat.3).2 = trans.2;
            mat
        }

        pub fn from_rotation(quat: Quat<T>) -> Self where Self: Default, T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> {
            let mut mat = Self::default();
            let w2 = quat.3 * quat.3;
            let x2 = quat.0 * quat.0;
            let y2 = quat.1 * quat.1;
            let z2 = quat.2 * quat.2;
            let zw = quat.2 * quat.3;
            let xy = quat.0 * quat.1;
            let xz = quat.0 * quat.2;
            let yw = quat.1 * quat.3;
            let yz = quat.1 * quat.2;
            let xw = quat.0 * quat.3;
            (mat.0).0 = w2 + x2 - z2 - y2;
            (mat.0).1 = xy + zw + zw + xy;
            (mat.0).2 = xz - yw + xz - yw;
            (mat.1).0 = -zw + xy - zw + xy;
            (mat.1).1 = y2 - z2 + w2 - x2;
            (mat.1).2 = yz + yz + xw + xw;
            (mat.2).0 = yw + xz + xz + yw;
            (mat.2).1 = yz + yz - xw - xw;
            (mat.2).2 = z2 - y2 - x2 + w2;
            mat
        }

        pub fn from_rotation_axis_angle(axis: Vec3<T>, angle: T) -> Self where T: Float {
            let axis = axis.normalize(T::val(1.0));
            let sin = angle.sin();
            let cos = angle.cos();
            let c = T::val(1.0) - cos;
            let xyc = axis.0 * axis.1 * c;
            let xzc = axis.0 * axis.2 * c;
            let yzc = axis.1 * axis.2 * c;
            let xs = axis.0 * sin;
            let ys = axis.1 * sin;
            let zs = axis.2 * sin;
            Self(
                Vec4(cos + axis.0 * axis.0 * c, xyc + zs, xzc - ys, T::val(0.0)),
                Vec4(xyc - zs, cos + axis.1 * axis.1 * c, yzc + xs, T::val(0.0)),
                Vec4(xzc + ys, yzc - xs, cos + axis.2 * axis.2 * c, T::val(0.0)),
                Vec4(T::val(0.0), T::val(0.0), T::val(0.0), T::val(1.0))
            )
        }

        pub fn from_scale(scale: Vec3<T>) -> Self where Self: Default {
            let mut mat = Self::default();
            (mat.0).0 = scale.0;
            (mat.1).1 = scale.1;
            (mat.2).2 = scale.2;
            mat
        }

        pub fn from_transform(t: Vec3<T>, q: Quat<T>, s: Vec3<T>) -> Self where Self: Default, T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> {
            let mut mat = Self::default();
            let dqx = q.0 + q.0;
            let dqy = q.1 + q.1;
            let dqz = q.2 + q.2;
            let q00 = dqx * q.0;
            let q11 = dqy * q.1;
            let q22 = dqz * q.2;
            let q01 = dqx * q.1;
            let q02 = dqx * q.2;
            let q03 = dqx * q.3;
            let q12 = dqy * q.2;
            let q13 = dqy * q.3;
            let q23 = dqz * q.3;
            (mat.0).0 = s.0 - (q11 + q22) * s.0;
            (mat.0).1 = (q01 + q23) * s.0;
            (mat.0).2 = (q02 - q13) * s.0;
            (mat.1).0 = (q01 - q23) * s.1;
            (mat.1).1 = s.1 - (q22 + q00) * s.1;
            (mat.1).2 = (q12 + q03) * s.1;
            (mat.2).0 = (q02 + q13) * s.2;
            (mat.2).1 = (q12 - q03) * s.2;
            (mat.2).2 = s.2 - (q11 + q00) * s.2;
            (mat.3).0 = t.0;
            (mat.3).1 = t.1;
            (mat.3).2 = t.2;
            mat
        }

        pub fn from_perspective(
            y_fov:         T,
            aspect:        T,
            z_near:        T,
            z_far:         T,
            z_zero_to_one: bool
        ) -> Self where T: Float {
            let mut mat = Self::default();
            let h = (y_fov * T::val(0.5)).tan();
            (mat.0).0 = T::val(1.0) / (h * aspect);
            (mat.1).1 = T::val(1.0) / h;

            if z_far > T::val(0.0) && (z_far == T::INF || z_far == T::NEG_INF) {
                (mat.2).2 = T::val(1E-6) - T::val(1.0);
                (mat.3).2 = (T::val(1E-6) - if z_zero_to_one { T::val(1.0) } else { T::val(2.0) }) * z_near;
            } else if z_near > T::val(0.0) && (z_near == T::INF || z_near == T::NEG_INF) {
                (mat.2).2 = (if z_zero_to_one { T::val(0.0) } else { T::val(1.0) }) - T::val(1E-6);
                (mat.3).2 = ((if z_zero_to_one { T::val(1.0) } else { T::val(2.0) }) - T::val(1E-6)) * z_far;
            } else {
                (mat.2).2 = (if z_zero_to_one { z_far } else { z_far + z_near }) / (z_near - z_far);
                (mat.3).2 = (if z_zero_to_one { z_far } else { z_far + z_far }) * z_near / (z_near - z_far);
            }

            (mat.2).3 = T::val(-1.0);
            (mat.3).3 = T::val(0.0);
            mat
        }

        pub fn from_ortho(
            left:          T,
            right:         T,
            bottom:        T,
            top:           T,
            z_near:        T,
            z_far:         T,
            z_zero_to_one: bool
        ) -> Self where T: Float {
            let mut mat = Self::default();
            (mat.0).0 = T::val(2.0) / (right - left);
            (mat.1).1 = T::val(2.0) / (top - bottom);
            (mat.2).2 = (if z_zero_to_one { T::val(1.0) } else { T::val(2.0) }) / (z_near - z_far);
            (mat.3).0 = (right + left) / (left - right);
            (mat.3).1 = (top + bottom) / (bottom - top);
            (mat.3).2 = (if z_zero_to_one { z_near } else { z_far + z_near }) / (z_near - z_far);
            mat
        }

        pub fn from_ortho_symmetric(
            width:         T,
            height:        T,
            z_near:        T,
            z_far:         T,
            z_zero_to_one: bool
        ) -> Self where T: Float {
            let mut mat = Self::default();
            (mat.0).0 = T::val(2.0) / width;
            (mat.1).1 = T::val(2.0) / height;
            (mat.2).2 = (if z_zero_to_one { T::val(1.0) } else { T::val(2.0) }) / (z_near - z_far);
            (mat.3).2 = (if z_zero_to_one { z_near } else { z_far + z_near }) / (z_near - z_far);
            mat
        }

        pub fn transform(&self, vec: Vec4<T>) -> Vec4<T> where T: Add<Output = T> + Mul<Output = T> {
            self.0 * vec.0 + self.1 * vec.1 + self.2 * vec.2 + self.3 * vec.3
        }

        pub fn transform_pos(&self, vec: Vec3<T>) -> Vec3<T> where T: Add<Output = T> + Mul<Output = T> {
            self.0.xyz() * vec.0 + self.1.xyz() * vec.1 + self.2.xyz() * vec.2 + self.3.xyz()
        }

        pub fn transform_dir(&self, vec: Vec3<T>) -> Vec3<T> where T: Add<Output = T> + Mul<Output = T> {
            self.0.xyz() * vec.0 + self.1.xyz() * vec.1 + self.2.xyz() * vec.2
        }

        pub fn translate(self, t: Vec3<T>) -> Self where T: Add<Output = T> + Mul<Output = T> {
            Self(self.0, self.1, self.3, self.0 * t.0 + self.1 * t.1 + self.2 * t.2 + self.3)
        }

        pub fn translate_local(self, t: Vec3<T>) -> Self where T: Float {
            let t = Vec4::<T>::from((t, T::val(0.0)));
            Self(
                self.0 + t * (self.0).3,
                self.1 + t * (self.1).3,
                self.2 + t * (self.2).3,
                self.3 + t * (self.3).3
            )
        }

        pub fn rotate(self, axis: Vec3<T>, angle: T) -> Self where T: Float {
            let sin = angle.sin();
            let cos = angle.cos();
            let c = T::val(1.0) - cos;
            let m00 = axis.0 * axis.0 * c + cos;
            let m01 = axis.0 * axis.1 * c + axis.2 * sin;
            let m02 = axis.0 * axis.2 * c - axis.1 * sin;
            let m10 = axis.0 * axis.1 * c - axis.2 * sin;
            let m11 = axis.1 * axis.1 * c + cos;
            let m12 = axis.1 * axis.2 * c + axis.0 * sin;
            let m20 = axis.0 * axis.2 * c + axis.1 * sin;
            let m21 = axis.1 * axis.2 * c - axis.0 * sin;
            let m22 = axis.2 * axis.2 * c + cos;

            Self(
                self.0 * m00 + self.1 * m01 + self.2 * m02,
                self.0 * m10 + self.1 * m11 + self.2 * m12,
                self.0 * m20 + self.1 * m21 + self.2 * m22,
                self.3
            )
        }

        pub fn rotate_local(self, axis: Vec3<T>, angle: T) -> Self where T: Float {
            let sin = angle.sin();
            let cos = angle.cos();
            let c = T::val(1.0) - cos;
            let m00 = axis.0 * axis.0 * c + cos;
            let m01 = axis.0 * axis.1 * c + axis.2 * sin;
            let m02 = axis.0 * axis.2 * c - axis.1 * sin;
            let m10 = axis.0 * axis.1 * c - axis.2 * sin;
            let m11 = axis.1 * axis.1 * c + cos;
            let m12 = axis.1 * axis.2 * c + axis.0 * sin;
            let m20 = axis.0 * axis.2 * c + axis.1 * sin;
            let m21 = axis.1 * axis.2 * c - axis.0 * sin;
            let m22 = axis.2 * axis.2 * c + cos;

            Self(Vec4(m00 * (self.0).0 + m10 * (self.0).1 + m20 * (self.0).2,
                      m01 * (self.0).0 + m11 * (self.0).1 + m21 * (self.0).2,
                      m02 * (self.0).0 + m12 * (self.0).1 + m22 * (self.0).2,
                      (self.0).3),
                 Vec4(m00 * (self.1).0 + m10 * (self.1).1 + m20 * (self.1).2,
                      m01 * (self.1).0 + m11 * (self.1).1 + m21 * (self.1).2,
                      m02 * (self.1).0 + m12 * (self.1).1 + m22 * (self.1).2,
                      (self.1).3),
                 Vec4(m00 * (self.2).0 + m10 * (self.2).1 + m20 * (self.2).2,
                      m01 * (self.2).0 + m11 * (self.2).1 + m21 * (self.2).2,
                      m02 * (self.2).0 + m12 * (self.2).1 + m22 * (self.2).2,
                      (self.2).3),
                 Vec4(m00 * (self.3).0 + m10 * (self.3).1 + m20 * (self.3).2,
                      m01 * (self.3).0 + m11 * (self.3).1 + m21 * (self.3).2,
                      m02 * (self.3).0 + m12 * (self.3).1 + m22 * (self.3).2,
                      (self.3).3))
        }

        pub fn scale(self, scale: Vec3<T>) -> Self where T: Mul<Output = T> {
            Self(self.0 * scale.0, self.1 * scale.1, self.2 * scale.2, self.3)
        }

        pub fn scale_uniform(self, scale: T) -> Self where T: Mul<Output = T> {
            self.scale(Vec3::from(scale))
        }

        pub fn scale_local(self, scale: Vec3<T>) -> Self where T: Mul<Output = T> {
            Self(Vec4(scale.0 * (self.0).0, scale.1 * (self.0).1, scale.2 * (self.0).2, (self.0).3),
                 Vec4(scale.0 * (self.1).0, scale.1 * (self.1).1, scale.2 * (self.1).2, (self.1).3),
                 Vec4(scale.0 * (self.2).0, scale.1 * (self.2).1, scale.2 * (self.2).2, (self.2).3),
                 Vec4(scale.0 * (self.3).0, scale.1 * (self.3).1, scale.2 * (self.3).2, (self.3).3))
        }

        pub fn scale_local_uniform(self, scale: T) -> Self where T: Mul<Output = T> {
            self.scale_local(Vec3::from(scale))
        }

        pub fn get_translation(&self) -> Vec3<T> {
            Vec3((self.3).0, (self.3).1, (self.3).2)
        }

        pub fn get_rotation(&self) -> Quat<T> where T: Float {
            let tr = (self.0).0 + (self.1).1 + (self.2).2;

            if tr >= T::val(0.0) {
                let t = (tr + T::val(1.0)).sqrt();
                let w = T::val(0.5) * t;
                let t = T::val(0.5) / t;
                Vec4(((self.1).2 - (self.2).1) * t,
                     ((self.2).0 - (self.0).2) * t,
                     ((self.0).1 - (self.1).0) * t,
                     w)
            } else if (self.0).0 >= (self.1).1 && (self.0).0 >= (self.2).2 {
                let t = ((self.0).0 - ((self.1).1 + (self.2).2) + T::val(1.0)).sqrt();
                let x = T::val(0.5) * t;
                let t = T::val(0.5) / t;
                Vec4(x,
                     ((self.1).0 + (self.0).1) * t,
                     ((self.0).2 + (self.2).0) * t,
                     ((self.1).2 - (self.2).1) * t)
            } else if (self.1).1 > (self.2).2 {
                let t = ((self.1).1 - ((self.2).2 + (self.0).0) + T::val(1.0)).sqrt();
                let y = T::val(0.5) * t;
                let t = T::val(0.5) / t;
                Vec4(((self.1).0 + (self.0).1) * t,
                     y,
                     ((self.2).1 + (self.1).2) * t,
                     ((self.2).0 - (self.0).2) * t)
            } else {
                let t = ((self.2).2 - ((self.0).0 + (self.1).1) + T::val(1.0)).sqrt();
                let z = T::val(0.5) * t;
                let t = T::val(0.5) / t;
                Vec4(((self.0).2 + (self.2).0) * t,
                     ((self.2).1 + (self.1).2) * t,
                     z,
                     ((self.0).1 - (self.1).0) * t)
            }
        }

        pub fn get_rotation_axis_angle(self) -> (Vec3<T>, T) where T: Float {
            if ((self.1).0 - (self.0).1).abs() < T::val(1E-4)
                && ((self.2).0 - (self.0).2).abs() < T::val(1E-4)
                && ((self.2).1 - (self.1).2).abs() < T::val(1E-4) {
                let xx = ((self.0).0 + T::val(1.0)) / T::val(2.0);
                let yy = ((self.1).1 + T::val(1.0)) / T::val(2.0);
                let zz = ((self.2).2 + T::val(1.0)) / T::val(2.0);
                let xy = ((self.1).0 + (self.0).1) / T::val(4.0);
                let xz = ((self.2).0 + (self.0).2) / T::val(4.0);
                let yz = ((self.2).1 + (self.1).2) / T::val(4.0);
                (if xx > yy && xx > zz {
                    let x = xx.sqrt();
                    Vec3(x, xy / x, xz / x)
                } else if yy > zz {
                    let y = yy.sqrt();
                    Vec3(xy / y, y, yz / y)
                } else {
                    let z = zz.sqrt();
                    Vec3(xz / z, yz / z, z)
                }, T::PI)
            } else {
                let s = (((self.1).2 - (self.2).1) * ((self.1).2 - (self.2).1)
                    + ((self.2).0 - (self.0).2) * ((self.2).0 - (self.0).2)
                    + ((self.0).1 - (self.1).0) * ((self.0).1 - (self.1).0)).sqrt();
                (Vec3(((self.1).2 - (self.2).1) / s,
                      ((self.1).2 - (self.2).1) / s,
                      ((self.0).1 - (self.1).0) / s),
                 (((self.0).0 + (self.1).1 + (self.2).2 - T::val(1.0)) / T::val(2.0)).acos())
            }
        }

        pub fn get_scale(&self) -> Vec3<T> where T: Float {
            Vec3(self.0.xyz().len(), self.1.xyz().len(), self.2.xyz().len())
        }

        pub fn mul_component_wise(self, rhs: Self) -> Mat4<T::Output> where T: Mul {
            Mat4(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, self.3 * rhs.3)
        }

        pub fn div_component_wise(self, rhs: Self) -> Mat4<T::Output> where T: Div {
            Mat4(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2, self.3 / rhs.3)
        }

        pub fn mul_assign_component_wise(&mut self, rhs: Self) where T: MulAssign {
            self.0 *= rhs.0;
            self.1 *= rhs.1;
            self.2 *= rhs.2;
            self.3 *= rhs.3;
        }

        pub fn div_assign_component_wise(&mut self, rhs: Self) where T: DivAssign {
            self.0 /= rhs.0;
            self.1 /= rhs.1;
            self.2 /= rhs.2;
            self.3 /= rhs.3;
        }

        pub fn det(&self) -> T where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> {
            ((self.0).0 * (self.1).1 - (self.0).1 * (self.1).0) * ((self.2).2 * (self.3).3 - (self.2).3 * (self.3).2)
                + ((self.0).2 * (self.1).0 - (self.0).0 * (self.1).2) * ((self.2).1 * (self.3).3 - (self.2).3 * (self.3).1)
                + ((self.0).0 * (self.1).3 - (self.0).3 * (self.1).0) * ((self.2).1 * (self.3).2 - (self.2).2 * (self.3).1)
                + ((self.0).1 * (self.1).2 - (self.0).2 * (self.1).1) * ((self.2).0 * (self.3).3 - (self.2).3 * (self.3).0)
                + ((self.0).3 * (self.1).1 - (self.0).1 * (self.1).3) * ((self.2).0 * (self.3).2 - (self.2).2 * (self.3).0)
                + ((self.0).2 * (self.1).3 - (self.0).3 * (self.1).2) * ((self.2).0 * (self.3).1 - (self.2).1 * (self.3).0)
        }

        pub fn normalize3x3(self) -> Self where T: Float {
            Self(
                self.0.normalize_vec3(),
                self.1.normalize_vec3(),
                self.2.normalize_vec3(),
                self.3
            )
        }
    }

    impl<T: Float> Default for Mat4<T> {
        fn default() -> Self {
            Self(
                Vec4(T::val(1f32), T::val(0f32), T::val(0f32), T::val(0f32)),
                Vec4(T::val(0f32), T::val(1f32), T::val(0f32), T::val(0f32)),
                Vec4(T::val(0f32), T::val(0f32), T::val(1f32), T::val(0f32)),
                Vec4(T::val(0f32), T::val(0f32), T::val(0f32), T::val(1f32))
            )
        }
    }

    impl<T: Copy> From<[T; 16]> for Mat4<T> {
        fn from(v: [T; 16]) -> Self {
            Self(
                Vec4(v[0], v[1], v[2], v[3]),
                Vec4(v[4], v[5], v[6], v[7]),
                Vec4(v[8], v[9], v[10], v[11]),
                Vec4(v[12], v[13], v[14], v[15])
            )
        }
    }

    impl<T: Add<Output=T> + Mul<Output=T> + Copy> Mul for Mat4<T> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self(
                self.0 * (rhs.0).0 + self.1 * (rhs.0).1 + self.2 * (rhs.0).2 + self.3 * (rhs.0).3,
                self.0 * (rhs.1).0 + self.1 * (rhs.1).1 + self.2 * (rhs.1).2 + self.3 * (rhs.1).3,
                self.0 * (rhs.2).0 + self.1 * (rhs.2).1 + self.2 * (rhs.2).2 + self.3 * (rhs.2).3,
                self.0 * (rhs.3).0 + self.1 * (rhs.3).1 + self.2 * (rhs.3).2 + self.3 * (rhs.3).3
            )
        }
    }

    impl<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Neg<Output=T> + Copy> Div for Mat4<T> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self * (!rhs)
        }
    }

    impl<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Neg<Output=T> + Copy> Not for Mat4<T> {
        type Output = Self;

        fn not(self) -> Self {
            let a = (self.0).0 * (self.1).1 - (self.0).1 * (self.1).0;
            let b = (self.0).0 * (self.1).2 - (self.0).2 * (self.1).0;
            let c = (self.0).0 * (self.1).3 - (self.0).3 * (self.1).0;
            let d = (self.0).1 * (self.1).2 - (self.0).2 * (self.1).1;
            let e = (self.0).1 * (self.1).3 - (self.0).3 * (self.1).1;
            let f = (self.0).2 * (self.1).3 - (self.0).3 * (self.1).2;
            let g = (self.2).0 * (self.3).1 - (self.2).1 * (self.3).0;
            let h = (self.2).0 * (self.3).2 - (self.2).2 * (self.3).0;
            let i = (self.2).0 * (self.3).3 - (self.2).3 * (self.3).0;
            let j = (self.2).1 * (self.3).2 - (self.2).2 * (self.3).1;
            let k = (self.2).1 * (self.3).3 - (self.2).3 * (self.3).1;
            let l = (self.2).2 * (self.3).3 - (self.2).3 * (self.3).2;
            let det = a * l - b * k + c * j + d * i - e * h + f * g;
            Self(Vec4(((self.1).1 * l - (self.1).2 * k + (self.1).3 * j) / det,
                      (-(self.0).1 * l + (self.0).2 * k - (self.0).3 * j) / det,
                      ((self.3).1 * f - (self.3).2 * e + (self.3).3 * d) / det,
                      (-(self.2).1 * f + (self.2).2 * e - (self.2).3 * d) / det),
                 Vec4((-(self.1).0 * l + (self.1).2 * i - (self.1).3 * h) / det,
                      ((self.0).0 * l - (self.0).2 * i + (self.0).3 * h) / det,
                      (-(self.3).0 * f + (self.3).2 * c - (self.3).3 * b) / det,
                      ((self.2).0 * f - (self.2).2 * c + (self.2).3 * b) / det),
                 Vec4(((self.1).0 * k - (self.1).1 * i + (self.1).3 * g) / det,
                      (-(self.0).0 * k + (self.0).1 * i - (self.0).3 * g) / det,
                      ((self.3).0 * e - (self.3).1 * c + (self.3).3 * a) / det,
                      (-(self.2).0 * e + (self.2).1 * c - (self.2).3 * a) / det),
                 Vec4((-(self.1).0 * j + (self.1).1 * h - (self.1).2 * g) / det,
                      ((self.0).0 * j - (self.0).1 * h + (self.0).2 * g) / det,
                      (-(self.3).0 * d + (self.3).1 * b - (self.3).2 * a) / det,
                      ((self.2).0 * d - (self.2).1 * b + (self.2).2 * a) / det))
        }
    }

    mat_impl!(Mat4, Vec4; v0: 0, v1: 1, v2: 2, v3: 3);

    pub type Mat2f32 = Mat2<f32>;
    pub type Mat2f64 = Mat2<f32>;
    pub type Mat3f32 = Mat2<f32>;
    pub type Mat3f64 = Mat2<f32>;
    pub type Mat4f32 = Mat2<f32>;
    pub type Mat4f64 = Mat2<f32>;
}

pub mod curves2d {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub struct Bezier1(pub Vec2<f32>, pub Vec2<f32>);

    impl Bezier1 {
        /// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
		/// https://stackoverflow.com/questions/563198
        pub fn intersections(self, l0: Vec2<f32>, l1: Vec2<f32>) -> usize {
            Bezier2(self.0, self.0 + (self.1 - self.0) * 0.5, self.1).intersections(l0, l1)
        }

        pub fn distance(self, p: Vec2<f32>) -> f32 {
            let Self(p0, p1) = self;
            let d = p1 - p0;
            p.distance(p0 + d * (d.dot(p - p0) / d.dot(d)).clamp(0f32, 1f32))
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Bezier2(pub Vec2<f32>, pub Vec2<f32>, pub Vec2<f32>);

    impl Bezier2 {
        pub fn b(self, t: f32) -> Vec2<f32> {
            debug_assert!((0.0..=1.0).contains(&t));
            let Bezier2(p0, p1, p2) = self;
            let tinv = 1.0 - t;
            p0 * (tinv * tinv)
                + p1 * 2.0 * t * tinv
                + p2 * (t * t)
        }

        /// https://github.com/w8r/bezier-intersect/blob/master/src/quadratic.js
        pub fn intersections(self, l0: Vec2<f32>, l1: Vec2<f32>) -> usize {
            let Bezier2(p0, p1, p2) = self;

            // Convert line to normal form: ax + by + c = 0
            let n = Vec2(l0.1 - l1.1, l1.0 - l0.0); // Find normal to line: negative inverse of original line's slope
            let cl = l0.0 * l1.1 - l1.0 * l0.1;      // Determine new c coefficient

            let min = l0.0.min(l1.0);
            let max = l0.0.max(l1.0);

            // solve quadratic equation
            Polynomial([
                n.dot(p0 + p1 * -2.0 + p2),
                n.dot(p0 * -2.0 + p1 * 2.0),
                n.dot(p0) + cl
            ]).roots_iter()
                .filter(|t| *t >= 0.0 && *t <= 1.0) // check if point is on bezier
                .map(|t| self.b(t).0)                       // calculate point on bezier
                .filter(|b| *b >= min && *b <= max) // check if point is on line segment
                .count()                                           // count intersections
        }

        /// http://blog.gludion.com/2009/08/distance-to-quadratic-bezier-curve.html
		/// alternative: https://www.academia.edu/34616746/An_algorithm_for_computing_the_shortest_distance_between_a_point_and_quadratic_Bezier_curve
        pub fn distance2(self, p: Vec2<f32>) -> f32 {
            let Bezier2(p0, p1, p2) = self;

            let a = p1 - p0;
            let b = p2 - p1 - a;
            let c = p0 - p;

            Polynomial([
                b.dot(b),
                3.0 * a.dot(b),
                2.0 * a.dot(a) + c.dot(b),
                c.dot(a)
            ]).roots_iter()
                .filter(|t| *t >= 0.0 && *t <= 1.0)
                .map(|t| p.distance2(self.b(t)))
                .fold(p.distance2(p0), f32::min)
                .min(p.distance2(p2))
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Bezier3(pub Vec2<f32>, pub Vec2<f32>, pub Vec2<f32>, pub Vec2<f32>);

    impl From<Bezier2> for Bezier3 {
        /// Based on https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Properties
        fn from(Bezier2(p0, p1, p2): Bezier2) -> Self {
            const FRAC_2_3: f32 = 2.0 / 3.0;
            Self(
                p0,
                p0 * (1f32 - FRAC_2_3) + p1 * FRAC_2_3,
                p2 * (1f32 - FRAC_2_3) + p1 * FRAC_2_3,
                p2
            )
        }
    }

    impl Bezier3 {
        pub fn b(self, t: f32) -> Vec2<f32> {
            debug_assert!((0.0..=1.0).contains(&t));
            let Self(p0, p1, p2, p3) = self;
            let t2 = t * t;
            let t3 = t * t2;
            let tinv = 1.0 - t;
            let tinv2 = tinv * tinv;
            let tinv3 = tinv * tinv2;
            p0 * tinv3
                + p1 * 3.0 * tinv2 * t
                + p2 * 3.0 * tinv2 * t2
                + p3 * t3
        }

        /// https://github.com/w8r/bezier-intersect/blob/master/src/cubic.js
        pub fn intersections(self, l0: Vec2<f32>, l1: Vec2<f32>) -> usize {
            let Self(p0, p1, p2, p3) = self;

            // Convert line to normal form: ax + by + c = 0
            let n = Vec2(l1.1 - l0.1, l0.0 - l1.0); // Find normal to line: negative inverse of original line's slope
            let cl = l0.0 * (l0.1 - l1.1) + l0.1 * (l1.0 - l0.0);//p.0 * r.1 - r.0 * p.1; // Determine new c coefficient

            let min = l0.min(l1);
            let max = l0.max(l1);

            // ?Rotate each cubic coefficient using line for new coordinate system?
            // Find roots of rotated cubic
            Polynomial([
                n.dot(-p0 + p1 * 3.0 - p2 * 3.0 + p3),
                n.dot(p0 * 3.0 - p1 * 6.0 + p2 * 3.0),
                n.dot(p0 * -3.0 + p1 * 3.0),
                n.dot(p0) + cl
            ]).roots_iter()
                .filter(|t| *t >= 0.0 && *t <= 1.0)            // check if t in [0;1] i.e. intersects Bezier
                .map(|t| self.b(t))                                    // calculate point on Bezier
                .filter(|b| b.0 >= min.0 && b.1 >= min.1
                    && b.0 <= max.0 && b.1 <= max.1)                          // check if point is on line segment
                .count()                                                      // count intersections
        }

        // alternative: https://hal.inria.fr/file/index/docid/518379/filename/Xiao-DiaoChen2007c.pdf
        /// brute force, accurate to 8 places
        pub fn distance2(self, p: Vec2<f32>) -> f32 {
            const STEPS: usize = 0x2000;
            let mut distance = self.3.distance2(p);
            for i in 0..STEPS {
                distance = distance.min(self.b(1.0 / STEPS as f32 * i as f32).distance2(p));
            }
            distance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn b1_intersections() {
            let b = Bezier1(Vec2(0.0, 0.0), Vec2(1.0, 0.0));
            assert_eq!(0, b.intersections(Vec2(0.0, 1.0), Vec2(1.0, 1.0)));
            assert_eq!(1, b.intersections(Vec2(0.0, -0.5), Vec2(1.0, 0.5)));
        }

        #[test]
        fn b1_distance() {
            let b = Bezier1(Vec2(0.0, 0.0), Vec2(1.0, 0.0));
            assert_eq!(1.0, b.distance(Vec2(0.5, 1.0)));
        }

        #[test]
        fn b2_intersections() {
            let b = Bezier2(Vec2(0.0, 0.0), Vec2(1.0, 0.0), Vec2(2.0, 0.0));
            assert_eq!(0, b.intersections(Vec2(0.0, 1.0), Vec2(1.0, 1.0)));
            assert_eq!(1, b.intersections(Vec2(0.0, -0.5), Vec2(1.0, 0.5)));
        }

        #[test]
        fn b2_distance() {
            let b = Bezier2(Vec2(0.0, 0.0), Vec2(1.0, 0.0), Vec2(2.0, 0.0));
            assert_eq!(1.0, b.distance2(Vec2(0.5, 1.0)));
        }

        #[test]
        fn b3_intersections() {
            let b = Bezier3(Vec2(0.0, 0.0), Vec2(1.0, 0.0), Vec2(2.0, 0.0), Vec2(3.0, 0.0));
            assert_eq!(0, b.intersections(Vec2(0.0, 1.0), Vec2(1.0, 1.0)));
            assert_eq!(1, b.intersections(Vec2(0.0, -0.5), Vec2(1.0, 0.5)));
        }

        #[test]
        fn b3_distance() {
            let b = Bezier3(Vec2(0.0, 0.0), Vec2(1.0, 0.0), Vec2(2.0, 0.0), Vec2(3.0, 0.0));
            assert!((b.distance2(Vec2(0.5, 1.0)) - 1.0).abs() < 1e-8);
        }
    }
}

pub mod shapes2d {
    use super::*;

    pub type EquTri2d   = RegularPolygon2d<3>;
    pub type Quad2d     = RegularPolygon2d<4>;
    pub type Pentagon2d = RegularPolygon2d<5>;
    pub type Hexagon2d  = RegularPolygon2d<6>;
    pub type Octagon2d  = RegularPolygon2d<7>;

    #[derive(Copy, Clone, Debug)]
    pub struct RegularPolygon2d<const N: usize> {
        pub pos: Vec2<f32>,
        pub rad: f32
    }

    #[derive(Copy, Clone)]
    pub struct Polygon2d<const N: usize> {
        pub pos: Vec2<f32>,
        pub v:   [Vec2<f32>; N]
    }
}

pub mod shapes3d {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub struct Sphere {
        pub radius: f32
    }

    impl Distance<Vec3<f32>> for Sphere {
        fn distance(&self, p: &Vec3<f32>) -> f32 {
            (*p).len() - self.radius
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Cuboid(pub Vec3<f32>);

    impl Distance<Vec3<f32>> for Cuboid {
        fn distance(&self, p: &Vec3<f32>) -> f32 {
            let d = (*p).abs() - self.0;
            d.maxs(0f32).len() + d.0.max(d.1).max(d.2).min(0f32)
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Plane(pub Vec4<f32>);

    impl Distance<Vec3<f32>> for Plane {
        fn distance(&self, p: &Vec3<f32>) -> f32 {
            //let n = self.0.normalize(1.0);
            (*p).dot(self.0.xyz()) + (self.0).3
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Cylinder(pub Vec2<f32>);

    impl Distance<Vec3<f32>> for Cylinder {
        fn distance(&self, p: &Vec3<f32>) -> f32 {
            let d = Vec2((*p).xz().len(), p.1).abs() - self.0;
            d.0.max(d.1).min(0.0) + d.maxs(0.0).len()
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Triangle {
        pub a: Vec3<f32>,
        pub b: Vec3<f32>,
        pub c: Vec3<f32>,
    }

    impl Distance2<Vec3<f32>> for Triangle {
        fn distance2(&self, p: &Vec3<f32>) -> f32 {
            let Triangle { a, b, c } = *self;
            let p = *p;
            let ba = b - a;
            let pa = p - a;
            let cb = c - b;
            let pb = p - b;
            let ac = a - c;
            let pc = p - c;
            let nor = ba.cross(ac);

            if ba.cross(nor).dot(pa).sign() +
                cb.cross(nor).dot(pb).sign() +
                ac.cross(nor).dot(pc).sign() < 2 {
                (ba * (ba.dot(pa) / ba.dot(ba)).clamp(0f32, 1f32) - pa).len2()
                    .min((cb * (cb.dot(pb) / cb.dot(cb)).clamp(0f32, 1f32) - pb).len2())
                    .min((ac * (ac.dot(pc) / ac.dot(ac)).clamp(0f32, 1f32) - pc).len2())
            } else {
                nor.dot(pa) * nor.dot(pa) / nor.dot(nor)
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Quad {
        pub a: Vec3<f32>,
        pub b: Vec3<f32>,
        pub c: Vec3<f32>,
        pub d: Vec3<f32>,
    }

    impl Distance2<Vec3<f32>> for Quad {
        fn distance2(&self, p: &Vec3<f32>) -> f32 {
            let Quad { a, b, c, d } = *self;
            let p = *p;
            let ba = b - a;
            let pa = p - a;
            let cb = c - b;
            let pb = p - b;
            let dc = d - c;
            let pc = p - c;
            let ad = a - d;
            let pd = p - d;
            let nor = ba.cross(ad);

            if ba.cross(nor).dot(pa).sign() +
                cb.cross(nor).dot(pb).sign() +
                dc.cross(nor).dot(pc).sign() +
                ad.cross(nor).dot(pd).sign() < 3 {
                (ba * (ba.dot(pa) / ba.dot(ba)).clamp(0f32, 1f32) - pa).len2()
                    .min((cb * (cb.dot(pb) / cb.dot(cb)).clamp(0f32, 1f32) - pb).len2())
                    .min((dc * (dc.dot(pc) / dc.dot(dc)).clamp(0f32, 1f32) - pc).len2())
                    .min((ad * (ad.dot(pd) / ad.dot(ad)).clamp(0f32, 1f32) - pd).len2())
            } else {
                nor.dot(pa) * nor.dot(pa) / nor.dot(nor)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn distance_sphere() {
            assert_eq!(1f32, Sphere { radius: 0.5 }.distance(&Vec3(0.0, 0.0, 1.5)))
        }

        #[test]
        fn distance_cuboid() {
            assert_eq!(1f32, Cuboid(Vec3(0.5, 0.5, 0.5)).distance(&Vec3(0.0, 0.0, 1.5)))
        }

        #[test]
        fn distance_plane() {
            assert_eq!(1f32, Plane(Vec4(0.0, 0.0, 1.0, 0.0)).distance(&Vec3(0.0, 0.0, 1.0)))
        }

        #[test]
        fn distance_cylinder() {
            assert_eq!(1f32, Cylinder(Vec2(0.5, 0.5)).distance(&Vec3(0.0, 0.0, 1.5)))
        }

        #[test]
        fn distance_triangle() {
            assert_eq!(1f32, Triangle {
                a: Vec3(1.0, 0.0, 0.0),
                b: Vec3(0.0, 0.0, 0.0),
                c: Vec3(0.0, 1.0, 0.0),
            }.distance(&Vec3(0.0, 0.0, 1.0)))
        }

        #[test]
        fn distance_quad() {
            assert_eq!(1f32, Quad {
                a: Vec3(0.0, 0.0, 0.0),
                b: Vec3(1.0, 0.0, 0.0),
                c: Vec3(1.0, 1.0, 0.0),
                d: Vec3(0.0, 1.0, 0.0),
            }.distance(&Vec3(0.0, 0.0, 1.0)))
        }
    }
}

#[allow(non_camel_case_types)]
pub mod types {
    macro_rules! type_impl {
	    ($ty:ident, $repr:ty, $eq:ty, $min:expr, $max:expr, $sca:expr) => {
	    	#[repr(transparent)]
			#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
			pub struct $ty(pub $repr);

	    	impl From<$eq> for $ty {
				fn from(v: $eq) -> Self {
					Self((v.clamp($min, $max) * $sca) as _)
				}
			}

			impl Into<$eq> for $ty {
				fn into(self) -> $eq {
					self.0 as $eq / $sca
				}
			}

	    	impl core::ops::Add for $ty {
				type Output = Self;

				fn add(self, rhs: Self) -> Self::Output {
					Self(self.0 + rhs.0)
				}
			}

			impl core::ops::Sub for $ty {
				type Output = Self;

				fn sub(self, rhs: Self) -> Self::Output {
					Self(self.0 - rhs.0)
				}
			}

			impl core::ops::Mul for $ty {
				type Output = Self;

				fn mul(self, rhs: Self) -> Self::Output {
					Self(self.0 * rhs.0)
				}
			}

			impl core::ops::Div for $ty {
				type Output = Self;

				fn div(self, rhs: Self) -> Self::Output {
					Self(self.0 / rhs.0)
				}
			}

			impl core::ops::AddAssign for $ty {
				fn add_assign(&mut self, rhs: Self) {
					self.0 += rhs.0
				}
			}

			impl core::ops::SubAssign for $ty {
				fn sub_assign(&mut self, rhs: Self) {
					self.0 -= rhs.0
				}
			}

			impl core::ops::MulAssign for $ty {
				fn mul_assign(&mut self, rhs: Self) {
					self.0 *= rhs.0
				}
			}

			impl core::ops::DivAssign for $ty {
				fn div_assign(&mut self, rhs: Self) {
					self.0 /= rhs.0
				}
			}
	    };
	}

    type_impl!(un8,  u8,  f32, 0f32,  1f32, core::u8::MAX as f32);
    type_impl!(sn8,  i8,  f32, -1f32, 1f32, core::i8::MAX as f32);
    type_impl!(un16, u16, f32, 0f32,  1f32, core::u16::MAX as f32);
    type_impl!(sn16, i16, f32, -1f32, 1f32, core::i16::MAX as f32);
    type_impl!(un32, u32, f32, 0f32,  1f32, core::u32::MAX as f32);
    type_impl!(sn32, i32, f32, -1f32, 1f32, core::i32::MAX as f32);
    type_impl!(un64, u64, f64, 0f64,  1f64, core::u64::MAX as f64);
    type_impl!(sn64, i64, f64, -1f64, 1f64, core::i64::MAX as f64);

    pub trait Float: Copy + Clone + Default + core::fmt::Debug
    + core::cmp::PartialEq + core::cmp::PartialOrd
    + core::ops::Add<Output = Self> + core::ops::AddAssign
    + core::ops::Sub<Output = Self> + core::ops::SubAssign
    + core::ops::Mul<Output = Self> + core::ops::MulAssign
    + core::ops::Div<Output = Self>+ core::ops::DivAssign
    {
        const INF: Self;
        const NEG_INF: Self;
        const PI: Self;

        fn sin(self) -> Self;

        fn cos(self) -> Self;

        fn acos(self) -> Self;

        fn tan(self) -> Self;

        fn powf(self, n: Self) -> Self;

        fn powi(self, n: i32) -> Self;

        fn sqrt(self) -> Self;

        fn cbrt(self) -> Self;

        fn log(self, base: Self) -> Self;

        fn abs(self) -> Self;

        fn val(val: f32) -> Self;
    }

    impl Float for f32 {
        const INF:     Self = core::f32::INFINITY;
        const NEG_INF: Self = core::f32::NEG_INFINITY;
        const PI:      Self = core::f32::consts::PI;

        fn sin(self) -> Self { self.sin() }

        fn cos(self) -> Self { self.cos() }

        fn acos(self) -> Self { self.acos() }

        fn tan(self) -> Self { self.tan() }

        fn powf(self, rhs: Self) -> Self { self.powf(rhs) }

        fn powi(self, n: i32) -> Self { self.powi(n) }

        fn sqrt(self) -> Self { self.sqrt() }

        fn cbrt(self) -> Self { self.cbrt() }

        fn log(self, base: Self) -> Self { self.log(base) }

        fn abs(self) -> Self { self.abs() }

        fn val(val: f32) -> Self { val }
    }

    impl Float for f64 {
        const INF:     Self = core::f64::INFINITY;
        const NEG_INF: Self = core::f64::NEG_INFINITY;
        const PI:      Self = core::f64::consts::PI;

        fn sin(self) -> Self { self.sin() }

        fn cos(self) -> Self { self.cos() }

        fn acos(self) -> Self { self.acos() }

        fn tan(self) -> Self { self.tan() }

        fn powf(self, rhs: Self) -> Self { self.powf(rhs) }

        fn powi(self, n: i32) -> Self { self.powi(n) }

        fn sqrt(self) -> Self { self.sqrt() }

        fn cbrt(self) -> Self { self.cbrt() }

        fn log(self, base: Self) -> Self { self.log(base) }

        fn abs(self) -> Self { self.abs() }

        fn val(val: f32) -> Self { val as _ }
    }
}

pub mod ops {
    pub trait Distance<T> {
        fn distance(&self, _: &T) -> f32;
    }

    pub trait Distance2<T> {
        fn distance2(&self, _: &T) -> f32;
    }

    impl<T: Distance2<U>, U> Distance<U> for T {
        fn distance(&self, v: &U) -> f32 {
            self.distance2(v).sqrt()
        }
    }

    pub trait Intersections<T> {
        fn intersections(&self, _: &T) -> u32;

        fn intersects(&self, v: &T) -> bool {
            self.intersections(v) > 0
        }
    }
}

pub mod sdf {
    use super::{*, raytracing::norm_to_uvw};
    pub use ops::*;

    /// Ray marches the given sdf.
	///
	/// # Parameters
	/// - lookup:        function to lookup the sdf
	/// - sample:        function that is called whenever the sdf is looked up
	/// - pos:           initial position
	/// - dir:           direction of the ray
	/// - min_distance:  the minimum distance that counts as a hit
	/// - max_distance:  the maximum distance after which the ray left the sdf
	/// - step_size:     the size of a single step
	/// - max_steps:     the maximum steps performed
	///
	/// # Returns
	/// - Some(pos): if something was hit
	/// - None:      if nothing was hit
    pub fn ray_march_sdf(
        lookup:        impl Fn(Vec3<f32>) -> f32,
        mut sample:    impl FnMut(Vec3<f32>, f32),
        mut pos:       Vec3<f32>,
        dir:           Vec3<f32>,
        min_distance:  f32,
        max_distance:  f32,
        step_size:     f32,
        max_steps:     usize
    ) -> Option<Vec3<f32>> {
        for _ in 0..max_steps {
            let distance = lookup(pos);
            sample(pos, distance);

            if distance < min_distance {
                return Some(pos);
            } else if distance > max_distance {
                return None;
            }

            pos += dir * step_size;
        }

        None
    }

    /// Sphere traces the given sdf.
	///
	/// # Parameters
	/// - lookup:        function to lookup the sdf
	/// - pos:           initial position
	/// - dir:           direction of the ray
	/// - min_distance:  the minimum distance that counts as a hit
	/// - max_distance:  the maximum distance after which the ray left the sdf
	/// - min_step_size: the minimum size of a step
	/// - min_steps:     the minimum step count performed
	/// - max_steps:     the maximum step count performed
	///
	/// # Returns
	/// - Some(pos): if something was hit
	/// - None:      if nothing was hit
    pub fn sphere_trace_sdf(
        lookup:        impl Fn(Vec3<f32>) -> f32,
        mut pos:       Vec3<f32>,
        dir:           Vec3<f32>,
        min_distance:  f32,
        max_distance:  f32,
        min_step_size: f32,
        min_steps:     usize,
        max_steps:     usize
    ) -> Option<Vec3<f32>> {
        for step in 0..max_steps {
            let distance = lookup(pos);

            if distance <= min_distance && step >= min_steps {
                return Some(pos);
            } else if distance > max_distance {
                return None;
            }

            pos += dir * distance.max(min_step_size);
        }

        None
    }

    /// Calculates the normal of the given point in the given sdf.
    pub fn calc_normal(
        lookup: impl Fn(Vec3<f32>) -> f32,
        pos:    Vec3<f32>,
        eps:    f32
    ) -> Vec3<f32> {
        let eps = Vec3(1.0, 0.0, 0.0) * eps;
        Vec3::<f32>(
            lookup(pos - eps.xyz()) - lookup(pos + eps.xyz()),
            lookup(pos - eps.zxy()) - lookup(pos + eps.zxy()),
            lookup(pos - eps.yzx()) - lookup(pos + eps.yzx())
        ).normalize(-1f32)
    }

    pub trait Sdf: Distance<Vec3<f32>> {
        fn lookup(&self, pos: Vec3<f32>) -> f32;

        fn ray_march(
            &self,
            sample:        &mut dyn FnMut(Vec3<f32>, f32),
            pos:           Vec3<f32>,
            dir:           Vec3<f32>,
            min_distance:  f32,
            max_distance:  f32,
            step_size:     f32,
            max_steps:     usize
        ) -> Option<Vec3<f32>> {
            ray_march_sdf(
                |v| self.lookup(v),
                sample,
                pos,
                dir,
                min_distance,
                max_distance,
                step_size,
                max_steps
            )
        }

        fn sphere_trace(
            &self,
            pos:           Vec3<f32>,
            dir:           Vec3<f32>,
            min_distance:  f32,
            max_distance:  f32,
            min_step_size: f32,
            min_steps:     usize,
            max_steps:     usize
        ) -> Option<Vec3<f32>> {
            sphere_trace_sdf(
                |v| self.lookup(v),
                pos,
                dir,
                min_distance,
                max_distance,
                min_step_size,
                min_steps,
                max_steps
            )
        }

        fn calc_normal(&self, pos: Vec3<f32>, eps: f32) -> Vec3<f32> {
            calc_normal(|v| self.lookup(v), pos, eps)
        }
    }

    impl<T: Distance<Vec3<f32>>> Sdf for T {
        fn lookup(&self, pos: Vec3<f32>) -> f32 {
            self.distance(&pos)
        }
    }

    pub trait SdfExt: Sdf + Sized {
        fn union<T: Sdf>(self, other: T) -> Union<Self, T> {
            Union(self, other)
        }

        fn subtraction<T: Sdf>(self, other: T) -> Subtraction<Self, T> {
            Subtraction(self, other)
        }

        fn intersection<T: Sdf>(self, other: T) -> Intersection<Self, T> {
            Intersection(self, other)
        }

        fn smooth_union<T: Sdf>(self, other: T, r: f32) -> SmoothUnion<Self, T> {
            SmoothUnion(self, other, r)
        }

        fn smooth_subtraction<T: Sdf>(self, other: T, r: f32) -> SmoothSubtraction<Self, T> {
            SmoothSubtraction(self, other, r)
        }

        fn smooth_intersection<T: Sdf>(self, other: T, r: f32) -> SmoothIntersection<Self, T> {
            SmoothIntersection(self, other, r)
        }

        fn round<T: Sdf>(self, r: f32) -> Round<Self> {
            Round(self, r)
        }

        fn translate(self, pos: Vec3<f32>) -> Translation<Self> {
            Translation(self, pos)
        }

        fn rotate(self, rot: Quat32) -> Rotation<Self> {
            Rotation(self, rot)
        }

        fn scale(self, scale: f32) -> Scale<Self> {
            Scale(self, scale)
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct OwnedSdf {
        pub data: Box<[f32]>,
        pub dim:  Vec3<usize>
    }

    impl OwnedSdf {
        pub fn new(dim: Vec3<usize>, val: f32) -> Self {
            Self { data: vec![val; dim.0 * dim.1 * dim.2].into_boxed_slice(), dim }
        }

        pub fn generate(f: impl Fn(Vec3<f32>) -> f32, dim: Vec3<usize>) -> Self {
            let mut data = vec![0f32; dim.0 * dim.1 * dim.2].into_boxed_slice();
            let scale = Vec3::from(1f32) / dim.cast::<f32>();

            for x in 0..dim.0 {
                for y in 0..dim.1 {
                    for z in 0..dim.2 {
                        data[x + dim.0 * y + dim.1 * z] = f(Vec3(x, y, z).cast::<f32>() * scale);
                    }
                }
            }

            Self { data, dim }
        }
    }

    impl Distance<Vec3<f32>> for OwnedSdf {
        fn distance(&self, pos: &Vec3<f32>) -> f32 {
            let dim = self.dim.cast::<f32>();
            let dim_factors = Vec3::from(2.0) - dim / dim.maxcw();
            let idx = *pos * dim_factors;

            // if pos is not in the texture bounds, calculate the closest distance to the texture box
            match norm_to_uvw((idx + Vec3::from(1.0)) / 2.0, self.dim) {
                Some(uvw) => {
                    fn idx(uvw: Vec3<f32>, dim: Vec3<usize>) -> usize {
                        let uvw = uvw.map(f32::round).cast::<usize>().clamp(Vec3::from(0), dim);
                        (uvw.0 + dim.0 * uvw.1 + dim.0 * dim.1 * uvw.2) as _
                    }

                    let uvwf = uvw.map(f32::floor);
                    let uvwc = uvw.map(f32::ceil);

                    let c00 = mix(self.data[idx(uvwf                        , self.dim)], self.data[idx(Vec3(uvwc.0, uvwf.1, uvwf.2), self.dim)], uvw.0 - uvwf.0);
                    let c01 = mix(self.data[idx(Vec3(uvwf.0, uvwc.1, uvwf.2), self.dim)], self.data[idx(Vec3(uvwc.0, uvwc.1, uvwf.2), self.dim)], uvw.0 - uvwf.0);
                    let c10 = mix(self.data[idx(Vec3(uvwf.0, uvwf.1, uvwc.2), self.dim)], self.data[idx(Vec3(uvwc.0, uvwf.1, uvwc.2), self.dim)], uvw.0 - uvwf.0);
                    let c11 = mix(self.data[idx(Vec3(uvwf.0, uvwc.1, uvwf.2), self.dim)], self.data[idx(Vec3(uvwc.0, uvwc.1, uvwc.2), self.dim)], uvw.0 - uvwf.0);
                    let c0 = mix(c00, c01, uvw.1 - uvwf.1);
                    let c1 = mix(c10, c11, uvw.1 - uvwf.1);
                    mix(c0, c1, uvw.2 - uvwf.2)
                },
                None => {
                    let q = pos.abs() - dim_factors * 0.5;
                    q.maxs(0.0).len() + q.maxcw().min(0.0)
                }
            }
        }
    }

    /*macro_rules! sdf_impl {
		($ty:ty) => {

		};
	}*/

    pub mod ops {
        use super::*;

        pub fn elongate(f: impl FnOnce(Vec3<f32>) -> f32, p: Vec3<f32>, h: Vec3<f32>) -> f32 {
            f(p - p.clamp(-h, h))
        }

        pub fn round(d: f32, rad: f32) -> f32 {
            d - rad
        }

        pub fn onion(d: f32, thickness: f32) -> f32 {
            d.abs() - thickness
        }

        pub fn extrusion(f: impl FnOnce(Vec2<f32>) -> f32, p: Vec3<f32>, h: f32) -> f32 {
            let d = f(p.xy());
            let w = Vec2(d, p.2.abs() - h);
            w.0.max(w.1).min(0f32) + w.max(Vec2::from(0f32)).len()
        }

        pub fn revolution(f: impl FnOnce(Vec2<f32>) -> f32, p: Vec3<f32>, o: f32) -> f32 {
            f(Vec2(p.xz().len() - o, p.1))
        }

        pub fn union(d1: f32, d2: f32) -> f32 {
            d1.min(d2)
        }

        pub fn subtraction(d1: f32, d2: f32) -> f32 {
            (-d1).max(d2)
        }

        pub fn intersection(d1: f32, d2: f32) -> f32 {
            d1.max(d2)
        }

        pub fn smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
            let h = (0.5f32 + 0.5f32 * (d2 - d1) / k).clamp(0f32, 1f32);
            mix(d2, d1, h) - k * h * (1f32 - h)
        }

        pub fn smooth_subtraction(d1: f32, d2: f32, k: f32) -> f32 {
            let h = (0.5f32 - 0.5f32 * (d2 + d1) / k).clamp(0f32, 1f32);
            mix(d2, -d1, h) + k * h * (1f32 - h)
        }

        pub fn smooth_intersection(d1: f32, d2: f32, k: f32) -> f32 {
            let h = (0.5f32 - 0.5f32 * (d2 - d1) / k).clamp(0f32, 1f32);
            mix(d2, d1, h) + k * h * (1f32 - h)
        }

        pub struct Union<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>>(pub T, pub U);

        impl<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Union<T, U> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                union(self.0.distance(point), self.1.distance(point))
            }
        }

        pub struct Subtraction<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>>(pub T, pub U);

        impl<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Subtraction<T, U> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                subtraction(self.0.distance(point), self.1.distance(point))
            }
        }

        pub struct Intersection<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>>(pub T, pub U);

        impl<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Intersection<T, U> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                intersection(self.0.distance(point), self.1.distance(point))
            }
        }

        pub struct SmoothUnion<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>>(pub T, pub U, pub f32);

        impl<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>> Distance<Vec3<f32>> for SmoothUnion<T, U> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                smooth_union(self.0.distance(point), self.1.distance(point), self.2)
            }
        }

        pub struct SmoothSubtraction<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>>(pub T, pub U, pub f32);

        impl<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>> Distance<Vec3<f32>> for SmoothSubtraction<T, U> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                smooth_subtraction(self.0.distance(point), self.1.distance(point), self.2)
            }
        }

        pub struct SmoothIntersection<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>>(pub T, pub U, pub f32);

        impl<T: Distance<Vec3<f32>>, U: Distance<Vec3<f32>>> Distance<Vec3<f32>> for SmoothIntersection<T, U> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                smooth_intersection(self.0.distance(point), self.1.distance(point), self.2)
            }
        }

        pub struct Round<T: Distance<Vec3<f32>>>(pub T, pub f32);

        impl<T: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Round<T> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                self.0.distance(point) - self.1
            }
        }

        pub struct Translation<T: Distance<Vec3<f32>>>(pub T, pub Vec3<f32>);

        impl<T: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Translation<T> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                self.0.distance(&(*point + self.1))
            }
        }

        pub struct Rotation<T: Distance<Vec3<f32>>>(pub T, pub Quat32);

        impl<T: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Rotation<T> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                self.0.distance(&Mat4::<f32>::from_rotation(self.1).transform_pos(*point))
            }
        }

        pub struct Scale<T: Distance<Vec3<f32>>>(pub T, pub f32);

        impl<T: Distance<Vec3<f32>>> Distance<Vec3<f32>> for Scale<T> {
            fn distance(&self, point: &Vec3<f32>) -> f32 {
                self.0.distance(&(*point / self.1)) * self.1
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn calc_normal_sphere() {
            const EPS: f32 = 0.1;
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(1.0, 0.0, 0.0), EPS), Vec3(1.0, 0.0, 0.0));
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(0.0, 1.0, 0.0), EPS), Vec3(0.0, 1.0, 0.0));
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(0.0, 0.0, 1.0), EPS), Vec3(0.0, 0.0, 1.0));
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(-1.0, 0.0, 0.0), EPS), Vec3(-1.0, 0.0, 0.0));
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(0.0, -1.0, 0.0), EPS), Vec3(0.0, -1.0, 0.0));
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(0.0, 0.0, -1.0), EPS), Vec3(0.0, 0.0, -1.0));

            let ang: f32 = 45f32.to_radians();
            assert_eq!(Sphere { radius: 1.0 }.calc_normal(Vec3(ang.cos(), ang.sin(), 0.0), EPS),
                       Vec3(core::f32::consts::FRAC_1_SQRT_2, core::f32::consts::FRAC_1_SQRT_2, 0.0));
        }
    }
}

pub mod poly {
    const EPS: f32 = 1E-6;
    const FRAC_1_3: f32 = 1.0 / 3.0;

    pub struct Polynomial<const N: usize>(pub [f32; N]);

    impl Polynomial<3> {
        pub fn new(c0: f32, c1: f32, c2: f32) -> Self {
            Self([c0, c1, c2])
        }

        pub fn roots(self) -> ([f32; 2], usize) {
            let (c0, c1, c2) = (self.0[0], self.0[1], self.0[2]);

            if c0.abs() < EPS {
                return ([-c2 / c1, 0.0], 1);
            }

            let discriminant = c1 * c1 - (4.0 * c0 * c2);
            ([
                 (-c1 + discriminant.sqrt()) / (2.0 * c0),
                 (-c1 - discriminant.sqrt()) / (2.0 * c0)
             ], match discriminant {
                v if v > EPS => 2,
                v if v < EPS => 0,
                _ => 1
            })
        }

        pub fn roots_iter(self) -> RootsIter<2> {
            let (roots, count) = self.roots();
            RootsIter { roots, count, index: 0 }
        }
    }

    impl Polynomial<4> {
        pub fn new(c0: f32, c1: f32, c2: f32, c3: f32) -> Self {
            Self([c0, c1, c2, c3])
        }

        /// https://github.com/nical/lyon/blob/master/geom/src/utils.rs
        pub fn roots(self) -> ([f32; 3], usize) {
            let (c0, mut c1, mut c2, mut c3)
                = (self.0[0], self.0[1], self.0[2], self.0[3]);

            // check if it's a quadratic equation to avoid div by 0
            if c0.abs() < EPS {
                let (roots, count) = Polynomial([c1, c2, c3]).roots();
                return ([roots[0], roots[1], 0.0], count);
            }

            c1 /= c0;
            c2 /= c0;
            c3 /= c0;

            let d0 = (3.0 * c2 - c1 * c1) / 9.0;
            let d1 = (9.0 * c1 * c2 - 27.0 * c3 - 2.0 * c1 * c1 * c1) / 54.0;
            let d = d0 * d0 * d0 + d1 * d1;

            if d > EPS {
                let p = d1 + d.sqrt();
                let m = d1 - d.sqrt();
                let s = p.signum() * p.abs().powf(FRAC_1_3);
                let t = m.signum() * m.abs().powf(FRAC_1_3);

                if (s - t).abs() < EPS && (s + t) > EPS {
                    ([-c1 * FRAC_1_3 + s + t, -c1 * FRAC_1_3 - (s + t) * 0.5, 0.0], 2)
                } else {
                    ([-c1 * FRAC_1_3 + s + t, 0.0, 0.0], 1)
                }
            } else {
                let theta = (d1 / (-d0 * d0 * d0).sqrt()).acos();
                let d0 = 2.0 * (-d0).sqrt();
                ([
                     d0 * (theta                                * FRAC_1_3).cos() - c1 * FRAC_1_3,
                     d0 * ((theta + 2.0 * core::f32::consts::PI) * FRAC_1_3).cos() - c1 * FRAC_1_3,
                     d0 * ((theta + 4.0 * core::f32::consts::PI) * FRAC_1_3).cos() - c1 * FRAC_1_3,
                 ], 3)
            }
        }

        pub fn roots_iter(self) -> RootsIter<3> {
            let (roots, count) = self.roots();
            RootsIter { roots, count, index: 0, }
        }
    }

    pub struct RootsIter<const N: usize> {
        roots: [f32; N],
        count: usize,
        index: usize,
    }

    impl<const N: usize> Iterator for RootsIter<{N}> {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.count {
                let v = Some(self.roots[self.index]);
                self.index += 1;
                v
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn quadratic() {
            assert_eq!(([0.5, -3.0], 2), Polynomial([2.0, 5.0, -3.0]).roots());
        }

        #[test]
        fn cubic() {
            let (roots, count) = Polynomial([2.0, -4.0, -22.0, 24.0]).roots();
            assert_eq!(3, count);
            assert!((roots[0] - 4.0).abs() < EPS);
            assert!((roots[1] + 3.0).abs() < EPS);
            assert!((roots[2] - 1.0).abs() < EPS);
        }
    }
}

pub mod pbr {
    use super::*;

    pub fn scene_to_local(v: Vec3<f32>, normal: Vec3<f32>, dpdu: Vec3<f32>) -> Vec3<f32> {
        let ts = normal.cross(dpdu);
        Vec3(v.dot(dpdu), v.dot(ts), v.dot(normal))
    }

    pub fn sin2_theta(v: Vec3<f32>) -> f32 {
        0f32.max(1.0 - v.2 * v.2)
    }

    pub fn sin_theta(v: Vec3<f32>) -> f32 {
        sin2_theta(v).sqrt()
    }

    pub fn tan2_theta(v: Vec3<f32>) -> f32 {
        sin2_theta(v) / (v.2 * v.2)
    }

    pub fn tan_theta(v: Vec3<f32>) -> f32 {
        sin_theta(v) / v.2
    }

    pub fn cos_phi(v: Vec3<f32>) -> f32 {
        let sin_theta = sin_theta(v);
        if sin_theta == 0.0 { 1.0 } else { (v.0 / sin_theta).clamp(-1.0, 1.0) }
    }

    pub fn sin_phi(v: Vec3<f32>) -> f32 {
        let sin_theta = sin_theta(v);
        if sin_theta == 0.0 { 0.0 } else { (v.1 / sin_theta).clamp(-1.0, 1.0) }
    }

    #[allow(clippy::eq_op)]
    pub fn fresnel_dielectric(mut cos_theta_in: f32, mut ior_in: f32, mut ior_tr: f32) -> f32 {
        cos_theta_in = cos_theta_in.clamp(-1.0, 1.0);

        if cos_theta_in > 0.0 {
            core::mem::swap(&mut ior_in, &mut ior_tr);
            cos_theta_in = cos_theta_in.abs();
        }

        let sin_theta_in = 0f32.max(1.0 - cos_theta_in * cos_theta_in).sqrt();
        let sin_theta_tr = ior_in / ior_tr * sin_theta_in;
        if sin_theta_tr >= 1.0 { return 1.0; } // total internal reflection
        let cos_theta_tr = 0f32.max(1.0 - sin_theta_tr * sin_theta_tr).sqrt();

        let r_par_l = ((ior_tr * cos_theta_in) - (ior_in * cos_theta_tr))
            / ((ior_tr * cos_theta_in) - (ior_in * cos_theta_tr));
        let r_per_p = ((ior_in * cos_theta_in) - (ior_tr * cos_theta_tr))
            / ((ior_in * cos_theta_in) - (ior_tr * cos_theta_tr));

        (r_par_l * r_par_l + r_per_p * r_per_p) * 0.5
    }

    pub fn specular_reflection(r#in: Vec3<f32>, r: Vec3<f32>, ior_in: f32, ior_tr: f32) -> Vec3<f32> {
        r * fresnel_dielectric(r#in.2, ior_in, ior_tr) / r#in.2.abs()
    }

    pub fn specular_reflection_scene(mut r#in: Vec3<f32>, normal: Vec3<f32>, r: Vec3<f32>, ior_in: f32, ior_tr: f32) -> Vec3<f32> {
        r#in.2 = r#in.angle(normal).cos();
        specular_reflection(r#in, r, ior_in, ior_tr)
    }

    /// lambertian perfect diffuse reflection
    pub fn diffuse_reflection_lambertian(r: Vec3<f32>) -> Vec3<f32> {
        r * core::f32::consts::FRAC_1_PI
    }

    pub fn diffuse_reflection_oren_nayar_ab(sigma: f32) -> (f32, f32) {
        let sigma2 = sigma * sigma;
        (1.0 - (sigma2 / (2.0 * (sigma2 + 0.33))), 0.45 * sigma2 / (sigma2 + 0.09))
    }

    pub fn diffuse_reflection_oren_nayar(
        r#in:   Vec3<f32>,
        out:    Vec3<f32>,
        r:      Vec3<f32>,
        (a, b): (f32, f32)
    ) -> Vec3<f32> {
        let sin_theta_in = 0f32.max(1.0 - r#in.2 * r#in.2).sqrt();
        let sin_theta_out = 0f32.max(1.0 - out.2 * out.2).sqrt();

        let max_cos = if sin_theta_in > 1e-4 && sin_theta_out > 1e-4 {
            0f32.max(cos_phi(r#in) * cos_phi(out)
                + sin_phi(r#in) * sin_phi(out))
        } else {
            0.0
        };

        let (sin_alpha, tan_beta) = if r#in.2.abs() > out.2.abs() {
            (sin_theta_out, sin_theta_in / r#in.2.abs())
        } else {
            (sin_theta_in, sin_theta_out / out.2.abs())
        };

        r * core::f32::consts::FRAC_1_PI * (a + b * max_cos * sin_alpha * tan_beta)
    }

    pub fn distribution_beckmann(half: Vec3<f32>, alpha: Vec2<f32>) -> f32 {
        let tan2_theta = tan2_theta(half);
        if tan2_theta.is_infinite() { return 0.0; }
        let cos4_theta = half.2.powi(4);
        let e = tan2_theta * (cos_phi(half).powi(2) / alpha.0.powi(2)
            + sin_phi(half).powi(2) / alpha.1.powi(2));

        (-e).exp() / (core::f32::consts::PI * alpha.0 * alpha.1 * cos4_theta)
    }

    pub fn distribution_ggx(half: Vec3<f32>, alpha: Vec2<f32>) -> f32 {
        let tan2_theta = tan2_theta(half);
        if tan2_theta.is_infinite() { return 0.0; }
        let cos4_theta = half.2.powi(4);
        let e = tan2_theta * (cos_phi(half).powi(2) / alpha.0.powi(2)
            + sin_phi(half).powi(2) / alpha.1.powi(2));

        1.0 / (core::f32::consts::PI * alpha.0 * alpha.1 * cos4_theta * (1.0 + e) * (1.0 + e))
    }

    pub fn geometric_lambda_beckmann(v: Vec3<f32>, alpha: Vec2<f32>) -> f32 {
        let tan_theta = tan_theta(v).abs();
        if tan_theta.is_infinite() { return 0.0; }

        let a = 1.0 / (tan_theta * (
            cos_phi(v).powi(2) * alpha.0.powi(2) +
                sin_phi(v).powi(2) * alpha.1.powi(2)
        ).sqrt());
        if a >= 1.6 { return 0.0; }

        (1.0 - 1.259 * a + 0.396 * a * a) / (3.535 * a + 2.181 * a * a)
    }

    pub fn geometric_lambda_ggx(v: Vec3<f32>, alpha: Vec2<f32>) -> f32 {
        let tan_theta = tan_theta(v).abs();
        if tan_theta.is_infinite() { return 0.0; }

        let alpha = (
            cos_phi(v).powi(2) * alpha.0.powi(2) +
                sin_phi(v).powi(2) * alpha.1.powi(2)
        ).sqrt();

        (-1.0 + (1.0 + (alpha * tan_theta).powi(2)).sqrt()) * 0.5
    }

    pub fn geometric(r#in: Vec3<f32>, out: Vec3<f32>, lambda: impl Fn(Vec3<f32>) -> f32) -> f32 {
        1.0 / (1.0 + lambda(r#in) + lambda(out))
    }

    pub fn brdf_torrance_sparrow(
        r#in:                Vec3<f32>,
        out:                 Vec3<f32>,
        reflectance:         Vec3<f32>,
        normal_distribution: impl Fn(Vec3<f32>) -> f32,
        geometric_shadowing: impl Fn(Vec3<f32>, Vec3<f32>) -> f32,
        fresnel:             impl Fn(f32) -> f32
    ) -> Vec3<f32> {
        let cos_theta_out = out.2.abs(); // normal.dot(out)
        let cos_theta_in = r#in.2.abs(); // normal.dot(r#in)
        let half = r#in + out;

        if cos_theta_in == 0.0 || cos_theta_out == 0.0 || (half == Vec3::from(0.0)) {
            return Vec3::from(0.0);
        }

        let half = half.normalize(1.0);
        reflectance
            * normal_distribution(half)
            * geometric_shadowing(r#in, out)
            * fresnel(r#in.dot(half))
            / (4.0 * cos_theta_in * cos_theta_out)
    }

    pub fn brdf_default(
        r#in:                Vec3<f32>,
        out:                 Vec3<f32>,
        reflectance:         Vec3<f32>,
        alpha:               Vec2<f32>,
        ior_in:              f32,
        ior_tr:              f32
    ) -> Vec3<f32> {
        brdf_torrance_sparrow(
            r#in, out, reflectance,
            |half| distribution_beckmann(half, alpha),
            |i, o| geometric(i, o, |v| geometric_lambda_beckmann(v, alpha)),
            |v| fresnel_dielectric(v, ior_in, ior_tr)
        )
    }
}

pub mod raytracing {
    use super::*;
    use rand::Rng;
    use core::sync::atomic::*;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum DbgMode {
        Hit,
        Depth,
        Pos,
        Normal,
        Out,
        Emissive,
        Material,
        All
    }

    pub fn render(
        camera:  &impl Camera,
        scene:   &impl Scene,
        output:  &mut impl Output,
        extent:  Vec2<usize>,
        samples: usize,
        bounces: usize,
        mode:    Option<DbgMode>
    ) {
        if mode.is_some() { println!("render_dbg progress: 0.00% (0.00s)"); }
        let t0 = std::time::Instant::now();
        let mut last = t0;

        for y in 0..extent.1 {
            for x in 0..extent.0 {
                let mut col = Vec3::default();
                let mut spl = samples;

                for _ in 0..samples {
                    let uv = Vec2(x, y).cast::<f32>() / extent.cast::<f32>();
                    match color(camera.get_ray(uv), bounces, scene, mode) {
                        Some(v) => col += v,
                        None => spl -= 1
                    }
                }

                output.store(Vec2(x, y), if spl == 0 { None } else { Some(col / spl as f32) });

                if mode.is_some() && last.elapsed() > core::time::Duration::from_secs(2) {
                    println!("render_dbg progress: {:.02}% ({:.02}s)",
                             (x + y * extent.0) as f32 / extent.mulc() as f32 * 100.0,
                             t0.elapsed().as_secs_f32());
                    last = std::time::Instant::now();
                }
            }
        }

        if mode.is_some() { println!("render_dbg progress: 100.00% ({:.02}s)", t0.elapsed().as_secs_f32()); }
    }

    /// Calculates the color for the given ray, performing `bounces` bounces.
    #[allow(clippy::unnecessary_operation)]
    pub fn color(
        ray:     (Vec3<f32>, Vec3<f32>),
        bounces: usize,
        scene:   &impl Scene,
        mode:    Option<DbgMode>
    ) -> Option<Vec3<f32>> {
        if bounces == 0 { return None; }
        let (pos, nor, obj) = scene.trace(ray)?;
        let out = -ray.1;

        'outer: { return Some(match mode {
            Some(DbgMode::Hit)      => Vec3::from(1.0),
            Some(DbgMode::Depth)    => (pos - ray.0).len().xxx(),
            Some(DbgMode::Pos)      => (pos + Vec3::from(1.0)) / 2.0,
            Some(DbgMode::Out)      => (out + Vec3::from(1.0)) / 2.0,
            Some(DbgMode::Normal)   => (nor + Vec3::from(1.0)) / 2.0,
            Some(DbgMode::Emissive) => obj.emissive(pos, nor, out),
            Some(DbgMode::Material) => obj.material(pos, nor, out.reflect(nor), out),
            _                       => break 'outer
        }) };

        let (color, count) = core::iter::from_fn(|| Some(scene.sample(pos, nor, out))).take(4)
            .chain(core::iter::from_fn(|| Some(obj.sample(pos, nor, out))).take(4))
            .map(|dir| color((pos, dir), bounces - 1, scene, mode).unwrap_or_default()
                * obj.material(pos, nor, dir, out)
                * dir.dot(nor).abs()
            ).fold((Vec3::default(), 0), |(v1, i), v2| (v1 + v2, i + 1));

        Some(obj.emissive(pos, nor, out) + color / count as f32)
    }

    /// The light transport (rendering) equation
    pub fn light_out(
        pos:            Vec3<f32>,
        nor:            Vec3<f32>,
        out:            Vec3<f32>,
        in_iter:        impl IntoIterator<Item = Vec3<f32>>,
        light_emissive: impl Fn(Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        light_in:       impl Fn(Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        material:       impl Fn(Vec3<f32>, Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
    ) -> Vec3<f32> {
        let (color, count) = in_iter.into_iter()
            .map(|r#in| light_in(pos, r#in) * material(pos, r#in, out) * r#in.dot(nor).abs())
            .fold((Vec3::default(), 0), |(v1, i), v2| (v1 + v2, i + 1));

        light_emissive(pos, out) + color / count as f32
    }

    /// The light transfer equation
    pub fn light_source(
        pos:            Vec3<f32>,
        out:            Vec3<f32>,
        in_iter:        impl IntoIterator<Item = Vec3<f32>>,
        light_emissive: impl Fn(Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        light_in:       impl Fn(Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        material:       impl Fn(Vec3<f32>, Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        scatter:        impl Fn(Vec3<f32>, Vec3<f32>) -> Vec3<f32>
    ) -> Vec3<f32> {
        let (color, count) = in_iter.into_iter()
            .map(|r#in| light_in(pos, r#in) * material(pos, r#in, out))
            .fold((Vec3::default(), 0), |(v1, i), v2| (v1 + v2, i + 1));

        light_emissive(pos, out) + scatter(pos, out) * (color / count as f32)
    }

    pub fn get_index3(uvw: Vec3<f32>, dim: Vec3<usize>) -> Option<usize> {
        if uvw.any(|v| *v < 0.0 || *v > 1.0) { return None; }
        let uvw = (uvw * (dim - Vec3::from(1)).cast::<f32>())
            .cast::<usize>();
        Some(uvw.0 + dim.0 * uvw.1 + dim.0 * dim.1 * uvw.2)
    }

    pub fn norm_to_uvw(uvw: Vec3<f32>, dim: Vec3<usize>) -> Option<Vec3<f32>> {
        if uvw.any(|v| *v < 0.0 || *v > 1.0) { return None; }
        Some(uvw * (dim - Vec3::from(1)).cast::<f32>())
    }

    pub trait Camera {
        fn get_ray(&self, uv: Vec2<f32>) -> (Vec3<f32>, Vec3<f32>);
    }

    impl<T: Fn(Vec2<f32>) -> (Vec3<f32>, Vec3<f32>)> Camera for T {
        fn get_ray(&self, uv: Vec2<f32>) -> (Vec3<f32>, Vec3<f32>) {
            self(uv)
        }
    }

    pub trait Scene {
        type Hitable: Hitable + Send + Sync;

        fn trace(&self, ray: (Vec3<f32>, Vec3<f32>)) -> Option<(Vec3<f32>, Vec3<f32>, &Self::Hitable)>;

        fn sample(&self, pos: Vec3<f32>, nor: Vec3<f32>, out: Vec3<f32>) -> Vec3<f32>;
    }

    /*impl<T: Fn((Vec3<f32>, Vec3<f32>)) -> Option<(Vec3<f32>, Vec3<f32>, &'static (dyn Hitable + Send + Sync))>> Scene for T {
		fn trace(&self, ray: (Vec3<f32>, Vec3<f32>)) -> Option<(Vec3<f32>, Vec3<f32>, &(dyn Hitable + Send + Sync))> {
			self(ray)
		}
	}*/

    pub trait Hitable {
        /// Returns an iterator generating samples.
        fn sample(&self, pos: Vec3<f32>, nor: Vec3<f32>, out: Vec3<f32>) -> Vec3<f32>;

        /// Calculates the emitted color, given a ray.
        fn emissive(&self, _pos: Vec3<f32>, _nor: Vec3<f32>, _out: Vec3<f32>) -> Vec3<f32> {
            Vec3::default()
        }

        /// Calculates the reflected color, given the incoming and the outgoing direction.
        fn material(&self, _pos: Vec3<f32>, _nor: Vec3<f32>, _in: Vec3<f32>, _out: Vec3<f32>) -> Vec3<f32> {
            Vec3::default()
        }
    }

    impl <
        T: Fn(Vec3<f32>, Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        U: Fn(Vec3<f32>, Vec3<f32>, Vec3<f32>, Vec3<f32>) -> Vec3<f32>,
        V: Fn(Vec3<f32>, Vec3<f32>, Vec3<f32>) -> Vec3<f32>
    > Hitable for (T, U, V) {
        fn sample(&self, pos: Vec3<f32>, nor: Vec3<f32>, out: Vec3<f32>) -> Vec3<f32> {
            (self.2)(pos, nor, out)
        }

        fn emissive(&self, pos: Vec3<f32>, nor: Vec3<f32>, out: Vec3<f32>) -> Vec3<f32> {
            (self.0)(pos, nor, out)
        }

        fn material(&self, pos: Vec3<f32>, nor: Vec3<f32>, r#in: Vec3<f32>, out: Vec3<f32>) -> Vec3<f32> {
            (self.1)(pos, nor, r#in, out)
        }
    }

    pub trait Output {
        fn store(&mut self, uv: Vec2<usize>, color: Option<Vec3<f32>>);
    }

    impl<T: FnMut(Vec2<usize>, Option<Vec3<f32>>)> Output for T {
        fn store(&mut self, uv: Vec2<usize>, color: Option<Vec3<f32>>) {
            self(uv, color);
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct BasicCamera {
        pos:        Vec3<f32>,
        lower_left: Vec3<f32>,
        horizontal: Vec3<f32>,
        vertical:   Vec3<f32>
    }

    impl BasicCamera {
        pub fn new(
            pos: Vec3<f32>,
            dir: Vec3<f32>,
            vup: Vec3<f32>,
            fov: f32,
            asp: f32
        ) -> Self {
            let half_height = (fov / 2f32).tan();
            let half_width = asp * half_height;
            let w = (pos - dir).normalize(1f32);
            let u = vup.cross(w).normalize(1f32);
            let v = w.cross(u);

            Self {
                pos,
                lower_left: pos - u * half_width - v * half_height - w,
                horizontal: u * 2f32 * half_width,
                vertical:   v * 2f32 * half_height
            }
        }
    }

    impl Default for BasicCamera {
        fn default() -> Self {
            Self::new(
                Vec3::default(),
                Vec3(1.0, 0.0, 0.0),
                Vec3(0.0, 0.0, 1.0),
                90f32.to_radians(),
                1.0
            )
        }
    }

    impl Camera for BasicCamera {
        fn get_ray(&self, uv: Vec2<f32>) -> (Vec3<f32>, Vec3<f32>) {
            (self.pos, (self.lower_left + self.horizontal * uv.0
                + self.vertical * uv.1 - self.pos).normalize(1.0))
        }
    }

    #[derive(Copy, Clone, Debug, Default)]
    pub struct BasicMaterial {
        pub albedo:    Vec3<f32>,
        pub emissive:  Vec3<f32>,
        pub metallic:  f32,
        pub roughness: f32
    }

    impl BasicMaterial {
        pub fn new(albedo: Vec3<f32>, emissive: Vec3<f32>, metallic: f32, roughness: f32) -> Self {
            Self { albedo, emissive, metallic, roughness }
        }
    }

    impl Hitable for BasicMaterial {
        fn sample(&self, _pos: Vec3<f32>, _nor: Vec3<f32>, _out: Vec3<f32>) -> Vec3<f32> {
            (_out.reflect(_nor) + Vec3(
                ::rand::thread_rng().gen_range(-1f32..1f32),
                ::rand::thread_rng().gen_range(-1f32..1f32),
                ::rand::thread_rng().gen_range(-1f32..1f32)
            )).normalize(1f32)
        }

        fn emissive(&self, _pos: Vec3<f32>, _nor: Vec3<f32>, _dir: Vec3<f32>) -> Vec3<f32> {
            self.emissive
        }

        fn material(&self, _pos: Vec3<f32>, nor: Vec3<f32>, r#in: Vec3<f32>, out: Vec3<f32>) -> Vec3<f32> {
            (if (out.reflect(nor) - r#in).len() < 1e-4 {
                specular_reflection_scene(r#in, nor, self.albedo, 1.00029,  2.42)
            } else {
                Vec3::from(0.0)
            }) + diffuse_reflection_lambertian(self.albedo)

            //self.albedo
        }
    }

    pub const SCENE_ID_MULTIPLE: u16 = core::u16::MAX;

    pub struct SdfScene<T: Hitable> {
        pub sdf:                   OwnedSdf,
        pub ids:                   Box<[u16]>,
        pub ids_dim:               Vec3<usize>,
        pub instances:             Vec<SdfSceneInstance>,
        pub sdfs:                  Vec<Box<dyn Sdf + Send + Sync>>,
        pub materials:             Vec<(T, f32)>,
        pub cfg_min_step_size:     f32,
        pub cfg_min_dist:          f32,
        pub cfg_min_dist_global:   f32,
        pub cfg_max_dist:          f32,
        pub cfg_min_steps:         usize,
        pub cfg_max_steps:         usize,
        pub cfg_normal_polls:      usize,
        pub cfg_normal_smoothness: f32,
        pub metrics_trace_ns:      AtomicUsize,
        pub metrics_update_ns:     AtomicUsize,
    }

    #[derive(Copy, Clone, Default, Debug)]
    pub struct SdfSceneInstance {
        pub sdf:   usize,
        pub mat:   usize,
        pub trans: Mat4<f32>
    }

    impl<T: Hitable> SdfScene<T> {
        pub fn new_default(sdf_dim: Vec3<usize>, ids_dim: Vec3<usize>) -> Self {
            Self::new(
                sdf_dim,
                ids_dim,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            )
        }

        pub fn new(
            sdf_dim:           Vec3<usize>,
            ids_dim:           Vec3<usize>,
            min_step_size:     Option<f32>,
            min_dist:          Option<f32>,
            min_dist_global:   Option<f32>,
            max_dist:          Option<f32>,
            min_steps:         Option<usize>,
            max_steps:         Option<usize>,
            normal_polls:      Option<usize>,
            normal_smoothness: Option<f32>
        ) -> Self {
            let sdf_texel_size = 1.0 / sdf_dim.maxcw() as f32;
            Self {
                sdf:                   OwnedSdf::new(sdf_dim, sdf_texel_size - 1e-6), // subtract a small value, to force to lookup all sdfs
                ids:                   vec![SCENE_ID_MULTIPLE; ids_dim.0 * ids_dim.1 * ids_dim.2].into_boxed_slice(),
                ids_dim,
                instances:             vec![],
                sdfs:                  vec![],
                materials:             vec![],
                cfg_min_step_size:     min_step_size.unwrap_or(1e-6),
                cfg_min_dist:          min_dist.unwrap_or(1e-6),
                cfg_min_dist_global:   min_dist_global.unwrap_or(sdf_texel_size),
                cfg_max_dist:          max_dist.unwrap_or(1.0),
                cfg_min_steps:         min_steps.unwrap_or(4),
                cfg_max_steps:         max_steps.unwrap_or(128),
                cfg_normal_polls:      normal_polls.unwrap_or(1),
                cfg_normal_smoothness: normal_smoothness.unwrap_or(1.0),
                metrics_trace_ns:      AtomicUsize::new(0),
                metrics_update_ns:     AtomicUsize::new(0)
            }
        }

        pub fn update_texel(&mut self, pos: Vec3<f32>) {
            let (id, dist) = self.lookup_all(pos * 2.0 - Vec3::from(1.0));
            self.sdf.data[get_index3(pos, self.sdf.dim).expect("`pos` is out of bounds")] = dist;
            self.ids[get_index3(pos, self.ids_dim).expect("`pos` is out of bounds")] = id as _;
        }

        pub fn update(&mut self, offset: Vec3<usize>, extent: Vec3<usize>) {
            let t = std::time::Instant::now();
            for x in offset.0..extent.0 {
                for y in offset.1..extent.1 {
                    for z in offset.2..extent.2 {
                        self.update_texel(Vec3(x, y, z).cast::<f32>() / self.sdf.dim.cast::<f32>());
                    }
                }
            }
            self.metrics_update_ns.fetch_add(t.elapsed().as_nanos() as _, Ordering::Relaxed);
        }

        pub fn update_all(&mut self) {
            self.update(Vec3::from(0), self.sdf.dim);
        }

        fn lookup_sdf(&self, id: usize, pos: Vec3<f32>) -> f32 {
            let inst = self.instances[id];
            self.sdfs[inst.sdf].lookup(inst.trans.transform_pos(pos)) /* * (1.0 / inst.trans.get_scale().0)*/
        }

        fn lookup_all(&self, pos: Vec3<f32>) -> (usize, f32) {
            self.instances.iter()
                .enumerate()
                .filter(|(_, inst)| inst.sdf != core::usize::MAX)
                .map(|(i, _)| (i, self.lookup_sdf(i, pos)))
                .reduce(|(i0, d0), (i1, d1)| if d1.abs() < d0.abs() {
                    (i1, d1) } else { (i0, d0) })
                .unwrap_or((SCENE_ID_MULTIPLE as usize, 1.0))
        }

        fn lookup_ids(&self, pos: Vec3<f32>) -> usize {
            get_index3((pos + Vec3::from(1.0)) / 2.0, self.ids_dim)
                .map(|i| self.ids[i])
                .unwrap_or(SCENE_ID_MULTIPLE) as usize
        }
    }

    impl<T: Hitable + Send + Sync> Scene for SdfScene<T> {
        type Hitable = T;

        fn trace(&self, (mut pos, dir): (Vec3<f32>, Vec3<f32>)) -> Option<(Vec3<f32>, Vec3<f32>, &Self::Hitable)> {
            let t = std::time::Instant::now();
            for step in 0..self.cfg_max_steps {
                let mut dist = self.sdf.lookup(pos);

                if dist <= self.cfg_min_dist_global {
                    let mut id = self.lookup_ids(pos);

                    dist = if id == SCENE_ID_MULTIPLE as usize {
                        let v = self.lookup_all(pos);
                        id = v.0;
                        v.1
                    } else {
                        self.lookup_sdf(id, pos)
                    };

                    if dist <= self.cfg_min_dist && step >= self.cfg_min_steps {
                        pos += dir * dist;
                        let (material, smoothness) = &self.materials[self.instances[id].mat];
                        let normal = (!self.instances[id].trans).transform_dir(calc_normal(|v| self.lookup_sdf(id, v), pos,
                                                                                           *smoothness * self.cfg_normal_smoothness));
                        self.metrics_trace_ns.fetch_add(t.elapsed().as_nanos() as _, Ordering::Relaxed);
                        return Some((pos, normal, material));
                    }
                } else if dist > self.cfg_max_dist {
                    self.metrics_trace_ns.fetch_add(t.elapsed().as_nanos() as _, Ordering::Relaxed);
                    return None;
                }

                pos += dir * dist.max(self.cfg_min_step_size);
            }

            self.metrics_trace_ns.fetch_add(t.elapsed().as_nanos() as _, Ordering::Relaxed);
            None
        }

        fn sample(&self, pos: Vec3<f32>, _nor: Vec3<f32>, _out: Vec3<f32>) -> Vec3<f32> {
            (self.instances[rand::thread_rng().gen_range(0..self.instances.len())]
                .trans.get_translation() - pos).normalize(1.0)
        }
    }

    pub struct BasicOutput {
        pub data:   Vec<Vec3<un8>>,
        pub extent: Vec2<usize>,
        pub clear:  Vec3<f32>
    }

    impl BasicOutput {
        pub fn new(extent: Vec2<usize>, clear: Vec3<f32>) -> Self {
            Self { data: vec![Default::default(); extent.0 * extent.1], extent, clear }
        }

        pub fn write_ppm(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
            writeln!(writer, "P6\n{} {}\n255", self.extent.0, self.extent.1)?;
            self.data.iter().try_for_each(|v| writer.write_all(&[(v.0).0, (v.1).0, (v.2).0]))
        }
    }

    impl Output for BasicOutput {
        fn store(&mut self, uv: Vec2<usize>, color: Option<Vec3<f32>>) {
            self[uv] = color.unwrap_or(self.clear).map(|v| un8::from(v.sqrt()))
        }
    }

    impl core::ops::Index<Vec2<usize>> for BasicOutput {
        type Output = Vec3<un8>;

        fn index(&self, index: Vec2<usize>) -> &Self::Output {
            &self.data[index.0 + index.1 * self.extent.0]
        }
    }

    impl core::ops::IndexMut<Vec2<usize>> for BasicOutput {
        fn index_mut(&mut self, index: Vec2<usize>) -> &mut Self::Output {
            &mut self.data[index.0 + index.1 * self.extent.0]
        }
    }
}

pub mod misc {
    use super::*;

    pub fn mix(x: f32, y: f32, a: f32) -> f32 {
        x * (1f32 - a) + y * a
    }

    pub trait Sign {
        fn sign(self) -> i32;
    }

    impl Sign for f32 {
        fn sign(self) -> i32 {
            match self {
                v if v < 0f32 => -1,
                v if v > 0f32 => 1,
                _ => 0
            }
        }
    }

    impl Sign for f64 {
        fn sign(self) -> i32 {
            match self {
                v if v < 0f64 => -1,
                v if v > 0f64 => 1,
                _ => 0
            }
        }
    }

    pub trait Swizzle: Copy {
        fn xx(self) -> Vec2<Self> {
            Vec2(self, self)
        }

        fn xxx(self) -> Vec3<Self> {
            Vec3(self, self, self)
        }

        fn xxxx(self) -> Vec4<Self> {
            Vec4(self, self, self, self)
        }
    }

    impl<T: Copy> Swizzle for T {}

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_mix() {
            assert_eq!(0.5, mix(0.0, 1.0, 0.5));
            assert_eq!(0.0, mix(-1.0, 1.0, 0.5));
        }

        #[test]
        fn sign() {
            assert_eq!(0, 0.0.sign());
            assert_eq!(-1, (-1.5).sign());
            assert_eq!(1, 1.5.sign());
        }
    }
}

