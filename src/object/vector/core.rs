use std::fmt::Debug;
use std::fmt::Display;

use crate::error::Error;
use crate::lang::EvalResult;
use crate::object::Obj;

use super::coercion::CoercibleInto;
use super::rep::Rep;
use super::subset::Subset;
use super::types::*;

#[derive(Default, Clone, PartialEq, Eq)]
pub enum OptionNA<T> {
    #[default]
    NA,
    Some(T),
}

impl<T> PartialOrd for OptionNA<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (OptionNA::Some(l), OptionNA::Some(r)) => l.partial_cmp(r),
            _ => None,
        }
    }
}

impl<T> OptionNA<T> {
    pub fn map<F, U>(self, f: F) -> OptionNA<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OptionNA::Some(x) => OptionNA::Some(f(x)),
            OptionNA::NA => OptionNA::NA,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Vector {
    Numeric(Rep<Numeric>),
    Integer(Rep<Integer>),
    Logical(Rep<Logical>),
    Character(Rep<Character>),
    // Complex(Complex),
    // Raw(Raw),
}

impl Vector {
    pub fn get(&self, index: usize) -> Option<Vector> {
        use Vector::*;
        match self {
            Numeric(x) => x.get(index).map(Numeric),
            Integer(x) => x.get(index).map(Integer),
            Logical(x) => x.get(index).map(Logical),
            Character(x) => x.get(index).map(Character),
        }
    }

    pub fn try_get(&self, index: Obj) -> EvalResult {
        let err =
            Error::Other("Vector index cannot be coerced into a valid indexing type.".to_string());
        match (self, index.as_vector()?) {
            (Vector::Numeric(v), Obj::Vector(i)) => {
                Ok(Obj::Vector(Vector::from(v.subset(i.try_into()?))))
            }
            (Vector::Integer(v), Obj::Vector(i)) => {
                Ok(Obj::Vector(Vector::from(v.subset(i.try_into()?))))
            }
            (Vector::Logical(v), Obj::Vector(i)) => {
                Ok(Obj::Vector(Vector::from(v.subset(i.try_into()?))))
            }
            (Vector::Character(v), Obj::Vector(i)) => {
                Ok(Obj::Vector(Vector::from(v.subset(i.try_into()?))))
            }
            _ => Err(err.into()),
        }
    }

    pub fn subset(&self, subset: Subset) -> Self {
        match self {
            Vector::Numeric(x) => x.subset(subset).into(),
            Vector::Integer(x) => x.subset(subset).into(),
            Vector::Logical(x) => x.subset(subset).into(),
            Vector::Character(x) => x.subset(subset).into(),
        }
    }

    pub fn assign(&mut self, other: Obj) -> EvalResult {
        let err =
            Error::Other("Cannot assign to a vector from a different type".to_string()).into();
        match (self, other) {
            (Vector::Numeric(l), Obj::Vector(Vector::Numeric(r))) => {
                Ok(Obj::Vector(Vector::from(l.assign(r))))
            }
            (Vector::Integer(l), Obj::Vector(Vector::Integer(r))) => {
                Ok(Obj::Vector(Vector::from(l.assign(r))))
            }
            (Vector::Logical(l), Obj::Vector(Vector::Logical(r))) => {
                Ok(Obj::Vector(Vector::from(l.assign(r))))
            }
            (Vector::Character(l), Obj::Vector(Vector::Character(r))) => {
                Ok(Obj::Vector(Vector::from(l.assign(r))))
            }
            _ => Err(err),
        }
    }

    pub fn materialize(self) -> Self {
        match self {
            Vector::Numeric(x) => Vector::from(x.materialize()),
            Vector::Integer(x) => Vector::from(x.materialize()),
            Vector::Logical(x) => Vector::from(x.materialize()),
            Vector::Character(x) => Vector::from(x.materialize()),
        }
    }

    pub fn vec_coerce<T, U>(v: &[OptionNA<T>]) -> Vec<OptionNA<U>>
    where
        T: CoercibleInto<U> + Clone,
    {
        use OptionNA::*;
        v.iter()
            .map(|i| match i {
                Some(x) => Some(CoercibleInto::<U>::coerce_into((*x).clone())),
                NA => NA,
            })
            .collect()
    }

    pub fn vec_parse<U>(v: &[OptionNA<String>]) -> (bool, Vec<OptionNA<U>>)
    where
        U: std::str::FromStr,
    {
        use OptionNA::*;
        let mut any_new_nas = false;
        let result = v
            .iter()
            .map(|i| match i {
                Some(s) => match s.parse::<U>() {
                    Ok(value) => Some(value),
                    Err(_) => {
                        any_new_nas = true;
                        NA
                    }
                },
                NA => NA,
            })
            .collect();

        (any_new_nas, result)
    }

    pub fn as_integer(self) -> Vector {
        use Vector::*;
        match self {
            Numeric(v) => Integer(v.as_integer()),
            Integer(_) => self,
            Logical(v) => Integer(v.as_integer()),
            Character(v) => Integer(v.as_integer()),
        }
    }

    pub fn as_numeric(self) -> Vector {
        use Vector::*;
        match self {
            Numeric(_) => self,
            Integer(v) => Numeric(v.as_numeric()),
            Logical(v) => Numeric(v.as_numeric()),
            Character(v) => Numeric(v.as_numeric()),
        }
    }

    pub fn as_logical(self) -> Vector {
        use Vector::*;
        match self {
            Numeric(v) => Logical(v.as_logical()),
            Integer(v) => Logical(v.as_logical()),
            Logical(_) => self,
            Character(v) => Logical(v.as_logical()),
        }
    }

    pub fn as_character(self) -> Vector {
        use Vector::*;
        match self {
            Numeric(v) => Character(v.as_character()),
            Integer(v) => Character(v.as_character()),
            Logical(v) => Character(v.as_character()),
            Character(_) => self,
        }
    }

    pub fn len(&self) -> usize {
        use Vector::*;
        match self {
            Numeric(v) => v.len(),
            Integer(v) => v.len(),
            Logical(v) => v.len(),
            Character(v) => v.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl TryInto<bool> for Vector {
    type Error = ();
    fn try_into(self) -> Result<bool, Self::Error> {
        use Vector::*;
        match self {
            Numeric(i) => i.try_into(),
            Integer(i) => i.try_into(),
            Logical(i) => i.try_into(),
            Character(i) => i.try_into(),
        }
    }
}

impl From<Rep<Numeric>> for Vector {
    fn from(x: Rep<Numeric>) -> Self {
        Vector::Numeric(x)
    }
}

impl From<Rep<Integer>> for Vector {
    fn from(x: Rep<Integer>) -> Self {
        Vector::Integer(x)
    }
}

impl From<Rep<Logical>> for Vector {
    fn from(x: Rep<Logical>) -> Self {
        Vector::Logical(x)
    }
}

impl From<Rep<Character>> for Vector {
    fn from(x: Rep<Character>) -> Self {
        Vector::Character(x)
    }
}

impl From<Vec<f64>> for Vector {
    fn from(x: Vec<f64>) -> Self {
        Vector::Numeric(x.into())
    }
}

impl From<Vec<OptionNA<f64>>> for Vector {
    fn from(x: Vec<OptionNA<f64>>) -> Self {
        Vector::Numeric(x.into())
    }
}

impl From<Vec<i32>> for Vector {
    fn from(x: Vec<i32>) -> Self {
        Vector::Integer(x.into())
    }
}

impl From<Vec<OptionNA<i32>>> for Vector {
    fn from(x: Vec<OptionNA<i32>>) -> Self {
        Vector::Integer(x.into())
    }
}

impl From<Vec<bool>> for Vector {
    fn from(x: Vec<bool>) -> Self {
        Vector::Logical(x.into())
    }
}

impl From<bool> for Vector {
    fn from(x: bool) -> Self {
        Vector::Logical(vec![x].into())
    }
}

impl From<Vec<OptionNA<bool>>> for Vector {
    fn from(x: Vec<OptionNA<bool>>) -> Self {
        Vector::Logical(x.into())
    }
}

impl From<Vec<String>> for Vector {
    fn from(x: Vec<String>) -> Self {
        Vector::Character(x.into())
    }
}

impl From<Vector> for String {
    fn from(val: Vector) -> Self {
        match val.as_character() {
            Vector::Character(v) => match v.inner().clone().borrow().first() {
                Some(OptionNA::Some(s)) => s.clone(),
                Some(OptionNA::NA) => "NA".to_string(),
                None => "".to_string(),
            },
            _ => unreachable!(),
        }
    }
}

impl From<Vector> for Vec<String> {
    fn from(val: Vector) -> Self {
        match val.as_character() {
            Vector::Character(v) => v
                .inner()
                .clone()
                .borrow()
                .iter()
                .map(|x| format!("{}", x))
                .collect(),
            _ => unreachable!(),
        }
    }
}

impl From<Vec<OptionNA<String>>> for Vector {
    fn from(x: Vec<OptionNA<String>>) -> Self {
        Vector::Character(x.into())
    }
}

pub trait DefaultDebug {}
impl DefaultDebug for bool {}
impl DefaultDebug for i32 {}
impl DefaultDebug for f64 {}

impl<T> Debug for OptionNA<T>
where
    T: DefaultDebug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionNA::Some(x) => write!(f, "{}", x),
            OptionNA::NA => write!(f, "NA"),
        }
    }
}

impl Debug for OptionNA<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionNA::Some(x) => write!(f, "\"{}\"", x),
            OptionNA::NA => write!(f, "NA"),
        }
    }
}

impl<T> Display for OptionNA<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptionNA::Some(x) => write!(f, "{}", x),
            OptionNA::NA => write!(f, "NA"),
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::Numeric(x) => std::fmt::Display::fmt(&x, f),
            Vector::Integer(x) => std::fmt::Display::fmt(&x, f),
            Vector::Logical(x) => std::fmt::Display::fmt(&x, f),
            Vector::Character(x) => std::fmt::Display::fmt(&x, f),
        }
    }
}

impl<T, U, O> std::ops::Add<OptionNA<U>> for OptionNA<T>
where
    T: std::ops::Add<U, Output = O>,
{
    type Output = OptionNA<O>;
    fn add(self, rhs: OptionNA<U>) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            _ => NA,
        }
    }
}

impl<T> std::ops::Sub for OptionNA<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = OptionNA<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l - r),
            _ => NA,
        }
    }
}

impl<T> std::ops::Neg for OptionNA<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = OptionNA<T>;
    fn neg(self) -> Self::Output {
        use OptionNA::*;
        match self {
            Some(x) => Some(x.neg()),
            _ => NA,
        }
    }
}

impl<T> std::ops::Mul for OptionNA<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = OptionNA<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l * r),
            _ => NA,
        }
    }
}

impl<T> std::ops::Div for OptionNA<T>
where
    T: std::ops::Div<Output = T>,
{
    type Output = OptionNA<T>;
    fn div(self, rhs: Self) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l / r),
            _ => NA,
        }
    }
}

pub trait Pow<Rhs> {
    type Output;
    /// raise self to the rhs power
    fn power(self, rhs: Rhs) -> Self::Output;
}

impl Pow<i32> for i32 {
    type Output = i32;
    fn power(self, rhs: Self) -> Self::Output {
        i32::pow(self, rhs as u32)
    }
}

impl Pow<f64> for i32 {
    type Output = f64;
    fn power(self, rhs: f64) -> Self::Output {
        f64::powf(self as f64, rhs)
    }
}

impl<T> Pow<T> for f64
where
    f64: From<T>,
{
    type Output = f64;
    fn power(self, rhs: T) -> Self::Output {
        f64::powf(self, rhs.into())
    }
}

impl<T, U, O> Pow<OptionNA<U>> for OptionNA<T>
where
    T: Pow<U, Output = O>,
{
    type Output = OptionNA<O>;
    fn power(self, rhs: OptionNA<U>) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(T::power(l, r)),
            _ => NA,
        }
    }
}

impl<T> std::ops::Rem for OptionNA<T>
where
    T: std::ops::Rem,
{
    type Output = OptionNA<<T as std::ops::Rem>::Output>;
    fn rem(self, rhs: Self) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.rem(r)),
            _ => NA,
        }
    }
}

impl std::ops::BitOr<Logical> for Logical {
    type Output = Logical;
    fn bitor(self, rhs: Logical) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l || r),
            _ => NA,
        }
    }
}

impl std::ops::BitAnd<Logical> for Logical {
    type Output = Logical;
    fn bitand(self, rhs: Logical) -> Self::Output {
        use OptionNA::*;
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l && r),
            _ => NA,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        use Vector::*;
        match self {
            Numeric(x) => Numeric(x.neg()),
            Integer(x) => Integer(x.neg()),
            Logical(x) => Integer(x.neg()),
            _ => todo!(),
        }
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => (l + r).into(),
            (Numeric(l), Integer(r)) => (l + r).into(),
            (Numeric(l), Logical(r)) => (l + r).into(),
            (Integer(l), Numeric(r)) => (l + r).into(),
            (Integer(l), Integer(r)) => (l + r).into(),
            (Integer(l), Logical(r)) => (l + r).into(),
            (Logical(l), Numeric(r)) => (l + r).into(),
            (Logical(l), Integer(r)) => (l + r).into(),
            (Logical(l), Logical(r)) => (l + r).into(),
            _ => todo!(),
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => (l - r).into(),
            (Numeric(l), Integer(r)) => (l - r).into(),
            (Numeric(l), Logical(r)) => (l - r).into(),
            (Integer(l), Numeric(r)) => (l - r).into(),
            (Integer(l), Integer(r)) => (l - r).into(),
            (Integer(l), Logical(r)) => (l - r).into(),
            (Logical(l), Numeric(r)) => (l - r).into(),
            (Logical(l), Integer(r)) => (l - r).into(),
            (Logical(l), Logical(r)) => (l - r).into(),
            _ => todo!(),
        }
    }
}

impl std::ops::Mul for Vector {
    type Output = Vector;
    fn mul(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => (l * r).into(),
            (Numeric(l), Integer(r)) => (l * r).into(),
            (Numeric(l), Logical(r)) => (l * r).into(),
            (Integer(l), Numeric(r)) => (l * r).into(),
            (Integer(l), Integer(r)) => (l * r).into(),
            (Integer(l), Logical(r)) => (l * r).into(),
            (Logical(l), Numeric(r)) => (l * r).into(),
            (Logical(l), Integer(r)) => (l * r).into(),
            (Logical(l), Logical(r)) => (l * r).into(),
            _ => todo!(),
        }
    }
}

impl std::ops::Div for Vector {
    type Output = Vector;
    fn div(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => (l / r).into(),
            (Numeric(l), Integer(r)) => (l / r).into(),
            (Numeric(l), Logical(r)) => (l / r).into(),
            (Integer(l), Numeric(r)) => (l / r).into(),
            (Integer(l), Integer(r)) => (l / r).into(),
            (Integer(l), Logical(r)) => (l / r).into(),
            (Logical(l), Numeric(r)) => (l / r).into(),
            (Logical(l), Integer(r)) => (l / r).into(),
            (Logical(l), Logical(r)) => (l / r).into(),
            _ => todo!(),
        }
    }
}

impl Pow<Vector> for Vector {
    type Output = Vector;
    fn power(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.power(r).into(),
            (Numeric(l), Integer(r)) => l.power(r).into(),
            (Numeric(l), Logical(r)) => l.power(r).into(),
            (Integer(l), Numeric(r)) => l.power(r).into(),
            (Integer(l), Integer(r)) => l.power(r).into(),
            (Integer(l), Logical(r)) => l.power(r).into(),
            (Logical(l), Numeric(r)) => l.power(r).into(),
            (Logical(l), Integer(r)) => l.power(r).into(),
            (Logical(l), Logical(r)) => l.power(r).into(),
            _ => todo!(),
        }
    }
}

pub trait VecPartialCmp<Rhs> {
    type Output;
    fn vec_gt(self, rhs: Rhs) -> Self::Output;
    fn vec_gte(self, rhs: Rhs) -> Self::Output;
    fn vec_lt(self, rhs: Rhs) -> Self::Output;
    fn vec_lte(self, rhs: Rhs) -> Self::Output;
    fn vec_eq(self, rhs: Rhs) -> Self::Output;
    fn vec_neq(self, rhs: Rhs) -> Self::Output;
}

impl VecPartialCmp<Vector> for Vector {
    type Output = Vector;
    fn vec_gt(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.vec_gt(r).into(),
            (Numeric(l), Integer(r)) => l.vec_gt(r).into(),
            (Numeric(l), Logical(r)) => l.vec_gt(r).into(),
            (Numeric(l), Character(r)) => l.vec_gt(r).into(),
            (Integer(l), Numeric(r)) => l.vec_gt(r).into(),
            (Integer(l), Integer(r)) => l.vec_gt(r).into(),
            (Integer(l), Logical(r)) => l.vec_gt(r).into(),
            (Integer(l), Character(r)) => l.vec_gt(r).into(),
            (Logical(l), Numeric(r)) => l.vec_gt(r).into(),
            (Logical(l), Integer(r)) => l.vec_gt(r).into(),
            (Logical(l), Logical(r)) => l.vec_gt(r).into(),
            (Logical(l), Character(r)) => l.vec_gt(r).into(),
            (Character(l), Numeric(r)) => l.vec_gt(r).into(),
            (Character(l), Integer(r)) => l.vec_gt(r).into(),
            (Character(l), Logical(r)) => l.vec_gt(r).into(),
            (Character(l), Character(r)) => l.vec_gt(r).into(),
        }
    }

    fn vec_gte(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.vec_gte(r).into(),
            (Numeric(l), Integer(r)) => l.vec_gte(r).into(),
            (Numeric(l), Logical(r)) => l.vec_gte(r).into(),
            (Numeric(l), Character(r)) => l.vec_gte(r).into(),
            (Integer(l), Numeric(r)) => l.vec_gte(r).into(),
            (Integer(l), Integer(r)) => l.vec_gte(r).into(),
            (Integer(l), Logical(r)) => l.vec_gte(r).into(),
            (Integer(l), Character(r)) => l.vec_gte(r).into(),
            (Logical(l), Numeric(r)) => l.vec_gte(r).into(),
            (Logical(l), Integer(r)) => l.vec_gte(r).into(),
            (Logical(l), Logical(r)) => l.vec_gte(r).into(),
            (Logical(l), Character(r)) => l.vec_gte(r).into(),
            (Character(l), Numeric(r)) => l.vec_gte(r).into(),
            (Character(l), Integer(r)) => l.vec_gte(r).into(),
            (Character(l), Logical(r)) => l.vec_gte(r).into(),
            (Character(l), Character(r)) => l.vec_gte(r).into(),
        }
    }

    fn vec_lt(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.vec_lt(r).into(),
            (Numeric(l), Integer(r)) => l.vec_lt(r).into(),
            (Numeric(l), Logical(r)) => l.vec_lt(r).into(),
            (Numeric(l), Character(r)) => l.vec_lt(r).into(),
            (Integer(l), Numeric(r)) => l.vec_lt(r).into(),
            (Integer(l), Integer(r)) => l.vec_lt(r).into(),
            (Integer(l), Logical(r)) => l.vec_lt(r).into(),
            (Integer(l), Character(r)) => l.vec_lt(r).into(),
            (Logical(l), Numeric(r)) => l.vec_lt(r).into(),
            (Logical(l), Integer(r)) => l.vec_lt(r).into(),
            (Logical(l), Logical(r)) => l.vec_lt(r).into(),
            (Logical(l), Character(r)) => l.vec_lt(r).into(),
            (Character(l), Numeric(r)) => l.vec_lt(r).into(),
            (Character(l), Integer(r)) => l.vec_lt(r).into(),
            (Character(l), Logical(r)) => l.vec_lt(r).into(),
            (Character(l), Character(r)) => l.vec_lt(r).into(),
        }
    }

    fn vec_lte(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.vec_lte(r).into(),
            (Numeric(l), Integer(r)) => l.vec_lte(r).into(),
            (Numeric(l), Logical(r)) => l.vec_lte(r).into(),
            (Numeric(l), Character(r)) => l.vec_lte(r).into(),
            (Integer(l), Numeric(r)) => l.vec_lte(r).into(),
            (Integer(l), Integer(r)) => l.vec_lte(r).into(),
            (Integer(l), Logical(r)) => l.vec_lte(r).into(),
            (Integer(l), Character(r)) => l.vec_lte(r).into(),
            (Logical(l), Numeric(r)) => l.vec_lte(r).into(),
            (Logical(l), Integer(r)) => l.vec_lte(r).into(),
            (Logical(l), Logical(r)) => l.vec_lte(r).into(),
            (Logical(l), Character(r)) => l.vec_lte(r).into(),
            (Character(l), Numeric(r)) => l.vec_lte(r).into(),
            (Character(l), Integer(r)) => l.vec_lte(r).into(),
            (Character(l), Logical(r)) => l.vec_lte(r).into(),
            (Character(l), Character(r)) => l.vec_lte(r).into(),
        }
    }

    fn vec_eq(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.vec_eq(r).into(),
            (Numeric(l), Integer(r)) => l.vec_eq(r).into(),
            (Numeric(l), Logical(r)) => l.vec_eq(r).into(),
            (Numeric(l), Character(r)) => l.vec_eq(r).into(),
            (Integer(l), Numeric(r)) => l.vec_eq(r).into(),
            (Integer(l), Integer(r)) => l.vec_eq(r).into(),
            (Integer(l), Logical(r)) => l.vec_eq(r).into(),
            (Integer(l), Character(r)) => l.vec_eq(r).into(),
            (Logical(l), Numeric(r)) => l.vec_eq(r).into(),
            (Logical(l), Integer(r)) => l.vec_eq(r).into(),
            (Logical(l), Logical(r)) => l.vec_eq(r).into(),
            (Logical(l), Character(r)) => l.vec_eq(r).into(),
            (Character(l), Numeric(r)) => l.vec_eq(r).into(),
            (Character(l), Integer(r)) => l.vec_eq(r).into(),
            (Character(l), Logical(r)) => l.vec_eq(r).into(),
            (Character(l), Character(r)) => l.vec_eq(r).into(),
        }
    }

    fn vec_neq(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.vec_neq(r).into(),
            (Numeric(l), Integer(r)) => l.vec_neq(r).into(),
            (Numeric(l), Logical(r)) => l.vec_neq(r).into(),
            (Numeric(l), Character(r)) => l.vec_neq(r).into(),
            (Integer(l), Numeric(r)) => l.vec_neq(r).into(),
            (Integer(l), Integer(r)) => l.vec_neq(r).into(),
            (Integer(l), Logical(r)) => l.vec_neq(r).into(),
            (Integer(l), Character(r)) => l.vec_neq(r).into(),
            (Logical(l), Numeric(r)) => l.vec_neq(r).into(),
            (Logical(l), Integer(r)) => l.vec_neq(r).into(),
            (Logical(l), Logical(r)) => l.vec_neq(r).into(),
            (Logical(l), Character(r)) => l.vec_neq(r).into(),
            (Character(l), Numeric(r)) => l.vec_neq(r).into(),
            (Character(l), Integer(r)) => l.vec_neq(r).into(),
            (Character(l), Logical(r)) => l.vec_neq(r).into(),
            (Character(l), Character(r)) => l.vec_neq(r).into(),
        }
    }
}

impl std::ops::Rem for Vector {
    type Output = Vector;
    fn rem(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.rem(r).into(),
            (Numeric(l), Integer(r)) => l.rem(r).into(),
            (Numeric(l), Logical(r)) => l.rem(r).into(),
            (Integer(l), Numeric(r)) => l.rem(r).into(),
            (Integer(l), Integer(r)) => l.rem(r).into(),
            (Integer(l), Logical(r)) => l.rem(r).into(),
            (Logical(l), Numeric(r)) => l.rem(r).into(),
            (Logical(l), Integer(r)) => l.rem(r).into(),
            (Logical(l), Logical(r)) => l.rem(r).into(),
            _ => todo!(),
        }
    }
}

impl std::ops::BitOr for Vector {
    type Output = Vector;
    fn bitor(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.bitor(r).into(),
            (Numeric(l), Integer(r)) => l.bitor(r).into(),
            (Numeric(l), Logical(r)) => l.bitor(r).into(),
            (Numeric(l), Character(r)) => l.bitor(r).into(),
            (Integer(l), Numeric(r)) => l.bitor(r).into(),
            (Integer(l), Integer(r)) => l.bitor(r).into(),
            (Integer(l), Logical(r)) => l.bitor(r).into(),
            (Integer(l), Character(r)) => l.bitor(r).into(),
            (Logical(l), Numeric(r)) => l.bitor(r).into(),
            (Logical(l), Integer(r)) => l.bitor(r).into(),
            (Logical(l), Logical(r)) => l.bitor(r).into(),
            (Logical(l), Character(r)) => l.bitor(r).into(),
            (Character(l), Numeric(r)) => l.bitor(r).into(),
            (Character(l), Integer(r)) => l.bitor(r).into(),
            (Character(l), Logical(r)) => l.bitor(r).into(),
            (Character(l), Character(r)) => l.bitor(r).into(),
        }
    }
}

impl std::ops::BitAnd for Vector {
    type Output = Vector;
    fn bitand(self, rhs: Self) -> Self::Output {
        use Vector::*;
        match (self, rhs) {
            (Numeric(l), Numeric(r)) => l.bitand(r).into(),
            (Numeric(l), Integer(r)) => l.bitand(r).into(),
            (Numeric(l), Logical(r)) => l.bitand(r).into(),
            (Numeric(l), Character(r)) => l.bitand(r).into(),
            (Integer(l), Numeric(r)) => l.bitand(r).into(),
            (Integer(l), Integer(r)) => l.bitand(r).into(),
            (Integer(l), Logical(r)) => l.bitand(r).into(),
            (Integer(l), Character(r)) => l.bitand(r).into(),
            (Logical(l), Numeric(r)) => l.bitand(r).into(),
            (Logical(l), Integer(r)) => l.bitand(r).into(),
            (Logical(l), Logical(r)) => l.bitand(r).into(),
            (Logical(l), Character(r)) => l.bitand(r).into(),
            (Character(l), Numeric(r)) => l.bitand(r).into(),
            (Character(l), Integer(r)) => l.bitand(r).into(),
            (Character(l), Logical(r)) => l.bitand(r).into(),
            (Character(l), Character(r)) => l.bitand(r).into(),
        }
    }
}
