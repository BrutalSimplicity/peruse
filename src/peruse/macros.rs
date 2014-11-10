
use parsers::*;

//coercion fn needed for unboxed closures
//without this all kinds of weird errors show up
pub fn _p<'a, I, O>(l: Box<Parser<'a, I, O> + 'a>) -> Box<Parser<'a, I, O> + 'a> {
  l
}

#[macro_export]
macro_rules! or {
  ($a: expr, $b: expr) => {
    _p(box OrParser{
      a: box |&:| $a ,
      b: box |&:| $b ,
    }) 
 };
  ($a: expr, $b: expr $(, $c: expr)* ) => {
    _p(box OrParser{
      a: box |&:| $a,
      b: box |&:| or!($b, $($c),*),
    }) 
  };
}

#[macro_export]
pub macro_rules! seq {
  ($a: expr, $b: expr ) => {
    box DualParser{
      first: $a,
      second: $b,
    } 
 };
  ($a: expr, $b: expr $(, $c: expr)* ) => {
    box DualParser{
      first: $a,
      second: seq!($b, $($c),* ),
    } 
  };
}

#[macro_export]
pub macro_rules! map {
  ($a: expr, $b: expr) => {
    box MapParser{
      parser: $a,
      mapper: box $b
    }
  }
}

#[macro_export]
pub macro_rules! repsep {
  ($rep: expr, $sep: expr, $min: expr) => {
    box RepSepParser{
      rep: $rep,
      sep: $sep,
      min_reps: $min,
    }
  }
}

#[macro_export]
pub macro_rules! rep {
  ($rep: expr) => {
    box RepParser{
      parser: $rep,
    }
  }
}