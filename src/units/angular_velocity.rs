use super::angle::{Angle, Radians, Degrees};

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;

use num_traits::float::Float;
use num_traits::{Zero};

use uom::{Conversion};
use uom::si::{SI};
use uom::si::time::{Time};

pub trait FromAngularVelocity<T>
    where T: AngularVelocity {

    fn from_angular_velocity(from: T) -> Self;
}

pub trait AngularVelocity where Self: Clone + FromAngularVelocity<Self> + PartialEq + PartialOrd + Zero {
    type Scalar: Float;
    type Angle: Angle<Scalar = Self::Scalar>;

    fn new(v: Self::Scalar) -> Self;

    fn scalar(&self) -> Self::Scalar;

    fn abs(&self) -> Self {
        Self::new(self.scalar().abs())
    }

    fn period() -> Self::Scalar {
        Self::Angle::period()
    }
}

#[derive(Debug,Clone,Eq,Ord,PartialOrd,PartialEq,Hash)]
pub struct DegreesPerSecond<S: Float> {
    value: S
}

impl<S: Float> AngularVelocity for DegreesPerSecond<S> {
    type Scalar = S;
    type Angle = Degrees<S>;

    fn new(v: S) -> Self {
        DegreesPerSecond{
            value: v
        }
    }

    fn scalar(&self) -> S {
        self.value
    }
}

impl <S, T, A> FromAngularVelocity<A> for DegreesPerSecond<S>
    where S: Float + From<T>,
          T: Float,
          A: AngularVelocity<Scalar = T> {
    fn from_angular_velocity(other: A) -> Self {
        let ratio = Self::period() / <S as From<T>>::from(A::period());

        Self::new(<S as From<T>>::from(other.scalar()) * ratio)
    }
}

impl <S: Float> Zero for DegreesPerSecond<S> {
    fn zero() -> Self {
        Self::new(S::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl <S, A> Add<A> for DegreesPerSecond<S>
    where S: Float,
          A: AngularVelocity<Scalar = S> + FromAngularVelocity<Self>{
    type Output = Self;

    fn add(self, rhs: A) -> Self {
        Self::new(self.value + Self::from_angular_velocity(rhs).value)
    }
}

impl <S, A> Sub<A> for DegreesPerSecond<S>
    where S: Float,
          A: AngularVelocity<Scalar = S> + FromAngularVelocity<Self>{
    type Output = Self;

    fn sub(self, rhs: A) -> Self {
        Self::new(self.value - Self::from_angular_velocity(rhs).value)
    }
}

impl <S: Float + Conversion<S>> Mul<Time<SI<S>, S>> for DegreesPerSecond<S> {
    type Output = Degrees<S>;

    fn mul(self, rhs: Time<SI<S>, S>) -> Degrees<S> {
        Degrees::new(self.value * rhs.value)
    }
}

impl Mul<f32> for DegreesPerSecond<f32> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.value * rhs)
    }
}

impl Mul<f64> for DegreesPerSecond<f64> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.value * rhs)
    }
}

impl <S: Float> Div<S> for DegreesPerSecond<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self {
        Self::new(self.value / rhs)
    }
}

impl <S: Float> Neg for DegreesPerSecond<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

#[derive(Debug,Clone,Eq,Ord,PartialOrd,PartialEq,Hash)]
pub struct RadiansPerSecond<S: Float> {
    value: S
}

impl<S: Float> AngularVelocity for RadiansPerSecond<S> {
    type Scalar = S;
    type Angle = Radians<S>;

    fn new(v: S) -> Self {
        RadiansPerSecond{
            value: v
        }
    }

    fn scalar(&self) -> S {
        self.value
    }
}

impl <S, T, A> FromAngularVelocity<A> for RadiansPerSecond<S>
    where S: Float + From<T>,
          T: Float,
          A: AngularVelocity<Scalar = T> {
    fn from_angular_velocity(other: A) -> Self {
        let ratio = Self::period() / <S as From<T>>::from(A::period());

        Self::new(<S as From<T>>::from(other.scalar()) * ratio)
    }
}

impl <S: Float> Zero for RadiansPerSecond<S> {
    fn zero() -> Self {
        Self::new(S::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl <S, A> Add<A> for RadiansPerSecond<S>
    where S: Float,
          A: AngularVelocity<Scalar = S> + FromAngularVelocity<Self>{
    type Output = Self;

    fn add(self, rhs: A) -> Self {
        Self::new(self.value + Self::from_angular_velocity(rhs).value)
    }
}

impl <S, A> Sub<A> for RadiansPerSecond<S>
    where S: Float,
          A: AngularVelocity<Scalar = S> + FromAngularVelocity<Self>{
    type Output = Self;

    fn sub(self, rhs: A) -> Self {
        Self::new(self.value - Self::from_angular_velocity(rhs).value)
    }
}

impl <S: Float + Conversion<S>> Mul<Time<SI<S>, S>> for RadiansPerSecond<S> {
    type Output = Radians<S>;

    fn mul(self, rhs: Time<SI<S>, S>) -> Radians<S> {
        Radians::new(self.value * rhs.value)
    }
}

impl Mul<f32> for RadiansPerSecond<f32> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.value * rhs)
    }
}

impl Mul<f64> for RadiansPerSecond<f64> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.value * rhs)
    }
}

impl <S: Float> Div<S> for RadiansPerSecond<S> {
    type Output = Self;

    fn div(self, rhs: S) -> Self {
        Self::new(self.value / rhs)
    }
}

impl <S: Float> Neg for RadiansPerSecond<S> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.value)
    }
}

impl <S: Float> From<RadiansPerSecond<S>> for DegreesPerSecond<S> {
    fn from(other: RadiansPerSecond<S>) -> Self {
        Self::from_angular_velocity(other)
    }
}

impl <S: Float> From<DegreesPerSecond<S>> for RadiansPerSecond<S> {
    fn from(other: DegreesPerSecond<S>) -> Self {
        Self::from_angular_velocity(other)
    }
}

impl From<DegreesPerSecond<f32>> for DegreesPerSecond<f64> {
    fn from(other: DegreesPerSecond<f32>) -> Self {
        Self::new(other.value as f64)
    }
}

impl From<DegreesPerSecond<f64>> for DegreesPerSecond<f32> {
    fn from(other: DegreesPerSecond<f64>) -> Self {
        Self::new(other.value as f32)
    }
}

impl From<RadiansPerSecond<f32>> for RadiansPerSecond<f64> {
    fn from(other: RadiansPerSecond<f32>) -> Self {
        Self::new(other.value as f64)
    }
}

impl From<RadiansPerSecond<f64>> for RadiansPerSecond<f32> {
    fn from(other: RadiansPerSecond<f64>) -> Self {
        Self::new(other.value as f32)
    }
}

impl<T: fmt::Display + Float> fmt::Display for DegreesPerSecond<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} deg/s", self.value)
    }
}

impl<T: fmt::Display + Float> fmt::Display for RadiansPerSecond<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} rad/s", self.value)
    }
}