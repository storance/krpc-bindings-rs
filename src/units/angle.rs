use std::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::float::{Float};
use num_traits::{Zero, NumCast, cast};

use std::f64::consts::PI;
use std::convert::{From};
use std::fmt;

use uom::{Conversion};
use uom::si::{SI};
use uom::si::time::{Time};

use super::angular_velocity::{AngularVelocity, DegreesPerSecond, RadiansPerSecond};

pub trait FromAngle<T>
    where T: Angle {
    fn from_angle(from: T) -> Self;
}

pub trait Angle where Self: Clone + FromAngle<Self> + PartialEq + PartialOrd + Zero {
    type Scalar : Float + NumCast;

    fn new(v: Self::Scalar) -> Self;

    fn sin(self) -> Self::Scalar;

    fn cos(self) -> Self::Scalar;

    fn tan(self) -> Self::Scalar;

    fn asin(v: Self::Scalar) -> Self;

    fn acos(v: Self::Scalar) -> Self;

    fn atan(v: Self::Scalar) -> Self;

    fn atan2(v1: Self::Scalar, v2: Self::Scalar) -> Self;

    fn sinh(self) -> Self::Scalar;

    fn cosh(self) -> Self::Scalar;

    fn tanh(self) -> Self::Scalar;

    fn asinh(v: Self::Scalar) -> Self;

    fn acosh(v: Self::Scalar) -> Self;

    fn atanh(v: Self::Scalar) -> Self;

    fn abs(&self) -> Self;

    fn scalar(&self) -> Self::Scalar;

    fn normalize(self) -> Self {
        if !self.is_normalized() {
            let new_value = self.scalar() % Self::period();
            if new_value < Self::Scalar::zero() {
                Self::new(new_value + Self::period())
            } else {
                Self::new(new_value)
            }
        } else {
            self
        }
    }

    fn is_normalized(&self) -> bool {
        return self.scalar() >= Self::Scalar::zero() && self.scalar() < Self::period();
    }

    fn period() -> Self::Scalar;

    fn full_turn() -> Self {
        Self::new( Self::period())
    }

    fn quarter_turn() -> Self {
        Self::new( cast::<_, Self::Scalar>(0.25).unwrap() * Self::period())
    }

    fn half_turn() -> Self {
        Self::new(cast::<_, Self::Scalar>(0.5).unwrap() * Self::period())
    }
}

#[derive(Debug,Clone,Eq,Ord,PartialOrd,PartialEq,Hash)]
pub struct Degrees<S : Float> {
    value: S
}

impl<S: Float + NumCast> Angle for Degrees<S> {
    type Scalar = S;

    fn new(v: S) -> Self {
        Degrees{
            value: v
        }
    }

    fn sin(self) -> S {
        Radians::from_angle(self).sin()
    }

    fn cos(self) -> S {
        Radians::from_angle(self).cos()
    }

    fn tan(self) -> S {
        Radians::from_angle(self).tan()
    }

    fn asin(v: S) -> Self {
        Degrees::from_angle(Radians::asin(v))
    }

    fn acos(v: S) -> Self {
        Degrees::from_angle(Radians::acos(v))
    }

    fn atan(v: S) -> Self {
        Degrees::from_angle(Radians::atan(v))
    }

    fn atan2(v1: S, v2: S) -> Self {
        Degrees::from_angle(Radians::atan2(v1, v2))
    }

    fn sinh(self) -> S {
        Radians::from_angle(self).sinh()
    }

    fn cosh(self) -> S {
        Radians::from_angle(self).cosh()
    }

    fn tanh(self) -> S {
        Radians::from_angle(self).tanh()
    }

    fn asinh(v: S) -> Self {
        Degrees::from_angle(Radians::asinh(v))
    }

    fn acosh(v: S) -> Self {
        Degrees::from_angle(Radians::acosh(v))
    }

    fn atanh(v: S) -> Self {
        Degrees::from_angle(Radians::atanh(v))
    }

    fn abs(&self) -> Self {
        Degrees::new(self.value.abs())
    }

    fn scalar(&self) -> S {
        return self.value
    }

    fn period() -> S {
        return S::from(360.0).unwrap();
    }
}

impl <S, A> Add<A> for Degrees<S>
    where S: Float,
          A: Angle<Scalar = S> + FromAngle<Self>{
    type Output = Self;

    fn add(self, rhs: A) -> Self {
        Self::new(self.value + Self::from_angle(rhs).value)
    }
}

impl <S, A> Sub<A> for Degrees<S>
    where S: Float,
          A: Angle<Scalar = S> + FromAngle<Self>{
    type Output = Self;

    fn sub(self, rhs: A) -> Self {
        Self::new(self.value - Self::from_angle(rhs).value)
    }
}

impl <S: Float> Mul<S> for Degrees<S> {
    type Output = Degrees<S>;

    fn mul(self, rhs: S) -> Degrees<S> {
        Degrees::new(self.value * rhs)
    }
}

impl <S: Float> Div<S> for Degrees<S> {
    type Output = Degrees<S>;

    fn div(self, rhs: S) -> Degrees<S> {
        Degrees::new(self.value / rhs)
    }
}

impl <S: Float> Div<Degrees<S>> for Degrees<S> {
    type Output = S;

    fn div(self, rhs: Degrees<S>) -> S {
        self.value / rhs.value
    }
}

impl <S> Div<Time<SI<S>, S>> for Degrees<S>
    where S: Float + Conversion<S> {
    type Output = DegreesPerSecond<S>;

    fn div(self, rhs: Time<SI<S>, S>) -> DegreesPerSecond<S> {
        DegreesPerSecond::new(self.value / rhs.value)
    }
}

impl <S: Float> Neg for Degrees<S> {
    type Output = Degrees<S>;

    fn neg(self) -> Degrees<S> {
        Degrees{
            value: -self.value
        }
    }
}

impl <S: Float> Zero for Degrees<S> {
    fn zero() -> Self {
        Degrees::new(S::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl <S, T, A> FromAngle<A> for Degrees<S>
    where S: Float + From<T>,
          T: Float,
          A: Angle<Scalar = T> {
    fn from_angle(other: A) -> Self {
        let ratio = Self::period() / <S as From<T>>::from(A::period());

        Self::new(<S as From<T>>::from(other.scalar()) * ratio)
    }
}

#[derive(Debug,Clone,Eq,Ord,PartialOrd,PartialEq,Hash)]
pub struct Radians<S : Float> {
    value: S
}

impl<S: Float + NumCast> Angle for Radians<S> {
    type Scalar = S;

    fn new(v: S) -> Self {
        Radians{
            value: v
        }
    }

    fn sin(self) -> S {
        self.value.sin()
    }

    fn cos(self) -> S {
        self.value.cos()
    }

    fn tan(self) -> S {
        self.value.tan()
    }

    fn asin(v: S) -> Self {
        Radians::new(v.asin())
    }

    fn acos(v: S) -> Self {
        Radians::new(v.acos())
    }

    fn atan(v: S) -> Self {
        Radians::new(v.atan())
    }

    fn atan2(v1: S, v2: S) -> Self {
        Radians::new(v1.atan2(v2))
    }

    fn sinh(self) -> S {
        return self.value.sinh()
    }

    fn cosh(self) -> S {
        return self.value.cosh()
    }

    fn tanh(self) -> S {
        return self.value.tanh()
    }

    fn asinh(v: S) -> Self {
        Radians::new(v.asinh())
    }

    fn acosh(v: S) -> Self {
        Radians::new(v.acosh())
    }

    fn atanh(v: S) -> Self {
        Radians::new(v.tanh())
    }

    fn abs(&self) -> Self {
        Radians::new(self.value.abs())
    }

    fn scalar(&self) -> S {
        return self.value
    }

    fn period() -> S {
        return S::from(2.0 * PI).unwrap();
    }
}

impl <S, A> Add<A> for Radians<S>
    where S: Float,
          A: Angle<Scalar = S> + FromAngle<Self>{
    type Output = Self;

    fn add(self, rhs: A) -> Self {
        Self::new(self.value + Self::from_angle(rhs).value)
    }
}

impl <S, A> Sub<A> for Radians<S>
    where S: Float,
          A: Angle<Scalar = S> + FromAngle<Self>{
    type Output = Self;

    fn sub(self, rhs: A) -> Self {
        Self::new(self.value - Self::from_angle(rhs).value)
    }
}

impl <S: Float> Mul<S> for Radians<S> {
    type Output = Self;

    fn mul(self, rhs: S) -> Self {
        Self::new(self.value * rhs)
    }
}

impl <S: Float> Div<S> for Radians<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self {
        Self::new(self.value / rhs)
    }
}

impl <S: Float> Div<Radians<S>> for Radians<S> {
    type Output = S;

    fn div(self, rhs: Radians<S>) -> S {
        self.value / Self::from_angle(rhs).value
    }
}

impl <S> Div<Time<SI<S>, S>> for Radians<S>
    where S: Float + Conversion<S> {
    type Output = RadiansPerSecond<S>;

    fn div(self, rhs: Time<SI<S>, S>) -> RadiansPerSecond<S> {
        RadiansPerSecond::new(self.value / rhs.value)
    }
}

impl <S: Float> Neg for Radians<S> {
    type Output = Radians<S>;

    fn neg(self) -> Radians<S> {
        Radians::new(-self.value)
    }
}

impl <S: Float> Zero for Radians<S> {
    fn zero() -> Self {
        Radians::new(S::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl <S, T, A> FromAngle<A> for Radians<S>
    where S: Float + From<T>,
          T: Float,
          A: Angle<Scalar = T> {
    fn from_angle(other: A) -> Self {
        let ratio = Self::period() / <S as From<T>>::from(A::period());

        Self::new(<S as From<T>>::from(other.scalar()) * ratio)
    }
}

impl <S: Float> From<Radians<S>> for Degrees<S> {
    fn from(other: Radians<S>) -> Self {
        Self::from_angle(other)
    }
}

impl <S: Float> From<Degrees<S>> for Radians<S> {
    fn from(other: Degrees<S>) -> Self {
        Self::from_angle(other)
    }
}

impl From<Degrees<f32>> for Degrees<f64> {
    fn from(other: Degrees<f32>) -> Self {
        Degrees::new(other.value as f64)
    }
}

impl From<Degrees<f64>> for Degrees<f32> {
    fn from(other: Degrees<f64>) -> Self {
        Degrees::new(other.value as f32)
    }
}

impl From<Radians<f32>> for Radians<f64> {
    fn from(other: Radians<f32>) -> Self {
        Radians::new(other.value as f64)
    }
}

impl From<Radians<f64>> for Radians<f32> {
    fn from(other: Radians<f64>) -> Self {
        Radians::new(other.value as f32)
    }
}

impl<T: fmt::Display + Float> fmt::Display for Degrees<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} deg", self.value)
    }
}

impl<T: fmt::Display + Float> fmt::Display for Radians<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} rad", self.value)
    }
}