//! Error management
//!
//! Depending on a compilation flag, the content of the `Context` enum
//! can change. In the default case, it will only have one variant:
//! `Context::Code(I, ErrorKind<E=u32>)` (with `I` and `E` configurable).
//! It contains an error code and the input position that triggered it.
//!

use internal::{Err, IResult};

pub trait ParseError<I>: Sized {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self;

  fn append(input: I, kind: ErrorKind, other: Self) -> Self;

  fn from_char(input: I, _: char) -> Self {
    Self::from_error_kind(input, ErrorKind::Char)
  }

  fn or(self, other: Self) -> Self {
    other
  }

  fn add_context(_input: I, _ctx: &'static str, other: Self) -> Self {
    other
  }
}

impl<I> ParseError<I> for (I, ErrorKind) {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    (input, kind)
  }

  fn append(_: I, _: ErrorKind, other: Self) -> Self {
    other
  }
}

impl<I> ParseError<I> for () {
  fn from_error_kind(_: I, _: ErrorKind) -> Self { }

  fn append(_: I, _: ErrorKind, _: Self) -> Self { }
}

pub fn make_error<I, E: ParseError<I>>(input: I, kind: ErrorKind) -> E {
  E::from_error_kind(input, kind)
}

pub fn append_error<I, E: ParseError<I>>(input: I, kind: ErrorKind, other: E) -> E {
  E::append(input, kind, other)
}

#[cfg(feature = "alloc")]
#[derive(Clone,Debug,PartialEq)]
pub struct VerboseError<I> {
  pub errors: ::lib::std::vec::Vec<(I, VerboseErrorKind)>,
}

#[cfg(feature = "alloc")]
#[derive(Clone,Debug,PartialEq)]
pub enum VerboseErrorKind {
  Context(&'static str),
  Char(char),
  Nom(ErrorKind),
}

#[cfg(feature = "alloc")]
impl<I> ParseError<I> for VerboseError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    VerboseError {
      errors: vec![(input, VerboseErrorKind::Nom(kind))]
    }
  }

  fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
    other.errors.push((input, VerboseErrorKind::Nom(kind)));
    other
  }

  fn from_char(input: I, c: char) -> Self {
    VerboseError {
      errors: vec![(input, VerboseErrorKind::Char(c))]
    }
  }

  fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
    other.errors.push((input, VerboseErrorKind::Context(ctx)));
    other
  }
}

#[cfg(feature = "alloc")]
pub fn context<I: Clone, E: ParseError<I>, F, O>(context: &'static str, f: F) -> impl FnOnce(I) -> IResult<I, O, E>
where
  F: Fn(I) -> IResult<I, O, E> {

    move |i: I| {
      match f(i.clone()) {
        Ok(o) => Ok(o),
        Err(Err::Incomplete(i)) => Err(Err::Incomplete(i)),
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
          Err(Err::Failure(E::add_context(i, context, e)))
        }
      }
    }

}


/// indicates which parser returned an error
#[cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
#[allow(deprecated)]
pub enum ErrorKind {
  //Custom(E),
  Tag,
  MapRes,
  MapOpt,
  Alt,
  IsNot,
  IsA,
  SeparatedList,
  SeparatedNonEmptyList,
  Many0,
  Many1,
  ManyTill,
  Count,
  TakeUntilAndConsume,
  TakeUntil,
  TakeUntilEitherAndConsume,
  TakeUntilEither,
  LengthValue,
  TagClosure,
  Alpha,
  Digit,
  HexDigit,
  OctDigit,
  AlphaNumeric,
  Space,
  MultiSpace,
  LengthValueFn,
  Eof,
  ExprOpt,
  ExprRes,
  CondReduce,
  Switch,
  TagBits,
  OneOf,
  NoneOf,
  Char,
  CrLf,
  RegexpMatch,
  RegexpMatches,
  RegexpFind,
  RegexpCapture,
  RegexpCaptures,
  TakeWhile1,
  Complete,
  Fix,
  Escaped,
  EscapedTransform,
  #[deprecated(since = "4.0.0", note = "Please use `Tag` instead")]
  TagStr,
  #[deprecated(since = "4.0.0", note = "Please use `IsNot` instead")]
  IsNotStr,
  #[deprecated(since = "4.0.0", note = "Please use `IsA` instead")]
  IsAStr,
  #[deprecated(since = "4.0.0", note = "Please use `TakeWhile1` instead")]
  TakeWhile1Str,
  NonEmpty,
  ManyMN,
  #[deprecated(since = "4.0.0", note = "Please use `TakeUntilAndConsume` instead")]
  TakeUntilAndConsumeStr,
  #[deprecated(since = "4.0.0", note = "Please use `TakeUntil` instead")]
  TakeUntilStr,
  Not,
  Permutation,
  Verify,
  TakeTill1,
  TakeUntilAndConsume1,
  TakeWhileMN,
  ParseTo,
  TooLarge,
  Many0Count,
  Many1Count,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(deprecated)]
pub fn error_to_u32(e: &ErrorKind) -> u32 {
  match *e {
    //ErrorKind::Custom(_)                 => 0,
    ErrorKind::Tag                       => 1,
    ErrorKind::MapRes                    => 2,
    ErrorKind::MapOpt                    => 3,
    ErrorKind::Alt                       => 4,
    ErrorKind::IsNot                     => 5,
    ErrorKind::IsA                       => 6,
    ErrorKind::SeparatedList             => 7,
    ErrorKind::SeparatedNonEmptyList     => 8,
    ErrorKind::Many1                     => 9,
    ErrorKind::Count                     => 10,
    ErrorKind::TakeUntilAndConsume       => 11,
    ErrorKind::TakeUntil                 => 12,
    ErrorKind::TakeUntilEitherAndConsume => 13,
    ErrorKind::TakeUntilEither           => 14,
    ErrorKind::LengthValue               => 15,
    ErrorKind::TagClosure                => 16,
    ErrorKind::Alpha                     => 17,
    ErrorKind::Digit                     => 18,
    ErrorKind::AlphaNumeric              => 19,
    ErrorKind::Space                     => 20,
    ErrorKind::MultiSpace                => 21,
    ErrorKind::LengthValueFn             => 22,
    ErrorKind::Eof                       => 23,
    ErrorKind::ExprOpt                   => 24,
    ErrorKind::ExprRes                   => 25,
    ErrorKind::CondReduce                => 26,
    ErrorKind::Switch                    => 27,
    ErrorKind::TagBits                   => 28,
    ErrorKind::OneOf                     => 29,
    ErrorKind::NoneOf                    => 30,
    ErrorKind::Char                      => 40,
    ErrorKind::CrLf                      => 41,
    ErrorKind::RegexpMatch               => 42,
    ErrorKind::RegexpMatches             => 43,
    ErrorKind::RegexpFind                => 44,
    ErrorKind::RegexpCapture             => 45,
    ErrorKind::RegexpCaptures            => 46,
    ErrorKind::TakeWhile1                => 47,
    ErrorKind::Complete                  => 48,
    ErrorKind::Fix                       => 49,
    ErrorKind::Escaped                   => 50,
    ErrorKind::EscapedTransform          => 51,
    ErrorKind::TagStr                    => 52,
    ErrorKind::IsNotStr                  => 53,
    ErrorKind::IsAStr                    => 54,
    ErrorKind::TakeWhile1Str             => 55,
    ErrorKind::NonEmpty                  => 56,
    ErrorKind::ManyMN                    => 57,
    ErrorKind::TakeUntilAndConsumeStr    => 58,
    ErrorKind::HexDigit                  => 59,
    ErrorKind::TakeUntilStr              => 60,
    ErrorKind::OctDigit                  => 61,
    ErrorKind::Many0                     => 62,
    ErrorKind::Not                       => 63,
    ErrorKind::Permutation               => 64,
    ErrorKind::ManyTill                  => 65,
    ErrorKind::Verify                    => 66,
    ErrorKind::TakeTill1                 => 67,
    ErrorKind::TakeUntilAndConsume1      => 68,
    ErrorKind::TakeWhileMN               => 69,
    ErrorKind::ParseTo                   => 70,
    ErrorKind::TooLarge                  => 71,
    ErrorKind::Many0Count                => 72,
    ErrorKind::Many1Count                => 73,
  }
}

impl ErrorKind {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  #[allow(deprecated)]
  pub fn description(&self) -> &str {
    match *self {
      //ErrorKind::Custom(_)                 => "Custom error",
      ErrorKind::Tag                       => "Tag",
      ErrorKind::MapRes                    => "Map on Result",
      ErrorKind::MapOpt                    => "Map on Option",
      ErrorKind::Alt                       => "Alternative",
      ErrorKind::IsNot                     => "IsNot",
      ErrorKind::IsA                       => "IsA",
      ErrorKind::SeparatedList             => "Separated list",
      ErrorKind::SeparatedNonEmptyList     => "Separated non empty list",
      ErrorKind::Many0                     => "Many0",
      ErrorKind::Many1                     => "Many1",
      ErrorKind::Count                     => "Count",
      ErrorKind::TakeUntilAndConsume       => "Take until and consume",
      ErrorKind::TakeUntil                 => "Take until",
      ErrorKind::TakeUntilEitherAndConsume => "Take until either and consume",
      ErrorKind::TakeUntilEither           => "Take until either",
      ErrorKind::LengthValue               => "Length followed by value",
      ErrorKind::TagClosure                => "Tag closure",
      ErrorKind::Alpha                     => "Alphabetic",
      ErrorKind::Digit                     => "Digit",
      ErrorKind::AlphaNumeric              => "AlphaNumeric",
      ErrorKind::Space                     => "Space",
      ErrorKind::MultiSpace                => "Multiple spaces",
      ErrorKind::LengthValueFn             => "LengthValueFn",
      ErrorKind::Eof                       => "End of file",
      ErrorKind::ExprOpt                   => "Evaluate Option",
      ErrorKind::ExprRes                   => "Evaluate Result",
      ErrorKind::CondReduce                => "Condition reduce",
      ErrorKind::Switch                    => "Switch",
      ErrorKind::TagBits                   => "Tag on bitstream",
      ErrorKind::OneOf                     => "OneOf",
      ErrorKind::NoneOf                    => "NoneOf",
      ErrorKind::Char                      => "Char",
      ErrorKind::CrLf                      => "CrLf",
      ErrorKind::RegexpMatch               => "RegexpMatch",
      ErrorKind::RegexpMatches             => "RegexpMatches",
      ErrorKind::RegexpFind                => "RegexpFind",
      ErrorKind::RegexpCapture             => "RegexpCapture",
      ErrorKind::RegexpCaptures            => "RegexpCaptures",
      ErrorKind::TakeWhile1                => "TakeWhile1",
      ErrorKind::Complete                  => "Complete",
      ErrorKind::Fix                       => "Fix",
      ErrorKind::Escaped                   => "Escaped",
      ErrorKind::EscapedTransform          => "EscapedTransform",
      ErrorKind::TagStr                    => "Tag on strings",
      ErrorKind::IsNotStr                  => "IsNot on strings",
      ErrorKind::IsAStr                    => "IsA on strings",
      ErrorKind::TakeWhile1Str             => "TakeWhile1 on strings",
      ErrorKind::NonEmpty                  => "NonEmpty",
      ErrorKind::ManyMN                    => "Many(m, n)",
      ErrorKind::TakeUntilAndConsumeStr    => "Take until and consume on strings",
      ErrorKind::HexDigit                  => "Hexadecimal Digit",
      ErrorKind::TakeUntilStr              => "Take until on strings",
      ErrorKind::OctDigit                  => "Octal digit",
      ErrorKind::Not                       => "Negation",
      ErrorKind::Permutation               => "Permutation",
      ErrorKind::ManyTill                  => "ManyTill",
      ErrorKind::Verify                    => "predicate verification",
      ErrorKind::TakeTill1                 => "TakeTill1",
      ErrorKind::TakeUntilAndConsume1      => "Take at least 1 until and consume",
      ErrorKind::TakeWhileMN               => "TakeWhileMN",
      ErrorKind::ParseTo                   => "Parse string to the specified type",
      ErrorKind::TooLarge                  => "Needed data size is too large",
      ErrorKind::Many0Count                => "Count occurrence of >=0 patterns",
      ErrorKind::Many1Count                => "Count occurrence of >=1 patterns",
    }
  }
}

/// creates a parse error from a `nom::ErrorKind`
/// and the position in the input
#[allow(unused_variables)]
#[macro_export(local_inner_macros)]
macro_rules! error_position(
  ($input:expr, $code:expr) => ({
    $crate::error::make_error($input, $code)
  });
);

/// creates a parse error from a `nom::ErrorKind`,
/// the position in the input and the next error in
/// the parsing tree.
#[allow(unused_variables)]
#[macro_export(local_inner_macros)]
macro_rules! error_node_position(
  ($input:expr, $code:expr, $next:expr) => ({
    $crate::error::append_error($input, $code, $next)
  });
);

/*

#[cfg(feature = "std")]
use $crate::lib::std::any::Any;
#[cfg(feature = "std")]
use $crate::lib::std::{error,fmt};
#[cfg(feature = "std")]
impl<E: fmt::Debug+Any> error::Error for Err<E> {
  fn description(&self) -> &str {
    self.description()
  }
}

#[cfg(feature = "std")]
impl<E: fmt::Debug> fmt::Display for Err<E> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}
*/

//FIXME: error rewrite
/// translate parser result from IResult<I,O,u32> to IResult<I,O,E> with a custom type
///
/// ```
/// # //FIXME
/// # #[macro_use] extern crate nom;
/// # use nom::IResult;
/// # use std::convert::From;
/// # use nom::Err;
/// # use nom::error::ErrorKind;
/// # fn main() {
/// #    /*
/// #    // will add a Custom(42) error to the error chain
/// #    named!(err_test, add_return_error!(ErrorKind::Custom(42u32), tag!("abcd")));
/// #
/// #    #[derive(Debug,Clone,PartialEq)]
/// #    pub struct ErrorStr(String);
/// #
/// #    // Convert to IResult<&[u8], &[u8], ErrorStr>
/// #    impl From<u32> for ErrorStr {
/// #      fn from(i: u32) -> Self {
/// #        ErrorStr(format!("custom error code: {}", i))
/// #      }
/// #    }
/// #
/// #    named!(parser<&[u8], &[u8], ErrorStr>,
/// #        fix_error!(ErrorStr, err_test)
/// #      );
/// #
/// #    let a = &b"efghblah"[..];
/// #    assert_eq!(parser(a), Err(Err::Error(Context::Code(a, ErrorKind::Custom(ErrorStr("custom error code: 42".to_string()))))));
/// # */
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! fix_error (
  ($i:expr, $t:ty, $submac:ident!( $($args:tt)* )) => (
    {
      use $crate::lib::std::result::Result::*;
      use $crate::Err;

      match $submac!($i, $($args)*) {
        Ok((i,o)) => Ok((i,o)),
        Err(e) => {
          let e2 = match e {
            Err::Error(err) => {
              /*
              let Context::Code(i, code) = err;
              let code2: ErrorKind<$t> = ErrorKind::convert(code);
              Err::Error(Context::Code(i, code2))
              */
              Err::Error(err.into())
            },
            Err::Failure(err) => {
              /*
              let Context::Code(i, code) = err;
              let code2: ErrorKind<$t> = ErrorKind::convert(code);
              Err::Failure(Context::Code(i, code2))
              */
              Err::Failure(err.into())
            },
            Err::Incomplete(e) => Err::Incomplete(e),
          };
          Err(e2)
        }
      }
    }
  );
  ($i:expr, $t:ty, $f:expr) => (
    fix_error!($i, $t, call!($f));
  );
);

/// `flat_map!(R -> IResult<R,S>, S -> IResult<S,T>) => R -> IResult<R, T>`
///
/// combines a parser R -> IResult<R,S> and
/// a parser S -> IResult<S,T> to return another
/// parser R -> IResult<R,T>
#[macro_export(local_inner_macros)]
macro_rules! flat_map(
  ($i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    flat_map!(__impl $i, $submac!($($args)*), $submac2!($($args2)*));
  );
  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    flat_map!(__impl $i, $submac!($($args)*), call!($g));
  );
  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    flat_map!(__impl $i, call!($f), $submac!($($args)*));
  );
  ($i:expr, $f:expr, $g:expr) => (
    flat_map!(__impl $i, call!($f), call!($g));
  );
  (__impl $i:expr, $submac:ident!( $($args:tt)* ), $submac2:ident!( $($args2:tt)* )) => (
    {
      use $crate::lib::std::result::Result::*;

      ($submac!($i, $($args)*)).and_then(|(i,o)| {
        match $submac2!(o, $($args2)*) {
          Err(e)      => Err(e.into()),
          Ok((_, o2)) => Ok((i, o2))
        }
      })
    }
  );
);
