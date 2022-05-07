use simple_expr_parser::{parse::{parse, ParseError}, structures::EvaluationError};
use std::fmt::{self, Display};
use fraction::{Fraction, Sign};
use std::error::Error;


pub fn calc(input: &str) -> Result<Fraction, ExpressionError> {
    match parse(input) {
        Ok(expr) => Ok(expr.eval()?),
        Err(err) => Err((err, input.to_string()).into()),
    }
}


#[derive(Debug)]
pub enum ExpressionError {
    Parse(ParseError, String),
    Evaluation(EvaluationError),
}

impl Error for ExpressionError {}

impl From<(ParseError, String)> for ExpressionError {
    fn from((err, raw): (ParseError, String)) -> Self {
        Self::Parse(err, raw)
    }
}
impl From<EvaluationError> for ExpressionError {
    fn from(err: EvaluationError) -> Self {
        Self::Evaluation(err)
    }
}

impl Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err, raw) => match err {
                ParseError::ExceptedExpr(c, at) => {
                    let at = raw[..*at].chars().count();
                    if let Some(c) = c {
                        write!(f, "式が期待されますが、文字'{}'が見つかりました。({}文字目)", c, at)?;
                    } else {
                        write!(f, "式が期待されます。({}文字目)", at)?;
                    }
                },
                ParseError::ExceptedOp(c, at) => {
                    let at = raw[..*at].chars().count();
                    write!(f, "演算子や閉じ括弧が期待されますが、文字'{}'が見つかりました。({}文字目)", c, at)?;
                },
                ParseError::InvalidCloseParenthese(at) => {
                    let at = raw[..*at].chars().count();
                    write!(f, "対応する開き括弧がありません。({}文字目)", at)?;
                },
                ParseError::UncloseParentheses => {
                    write!(f, "括弧が閉じられていません。")?;
                },
                ParseError::Overflow(raw) => {
                    write!(f, r#""{}"は大きすぎて計算できません。"#, raw)?;
                },
            },
            Self::Evaluation(err) => match err {
                EvaluationError::ZeroDivision => write!(f, "途中計算にゼロ除算が発生しました。")?,
                EvaluationError::Overflow => write!(f, "途中計算に算術オーバーフローが発生しました。")?,
            },
        }
        Ok(())
    }
}



pub fn to_integer<T: FromFrac>(frac: &Fraction) -> Option<T> {
    T::from_frac(frac)
}

pub trait FromFrac where
    Self: Sized,
{
    fn from_frac(frac: &Fraction) -> Option<Self>;
}
impl FromFrac for u64 {
    fn from_frac(frac: &Fraction) -> Option<Self> {
        match frac {
            Fraction::Rational(sign, ratio) => match sign {
                Sign::Plus => if ratio.is_integer() {Some(ratio.to_integer())} else {None},
                Sign::Minus => None,
            },
            _ => None,
        }
    }
}
impl FromFrac for i64 {
    fn from_frac(frac: &Fraction) -> Option<Self> {
        match frac {
            Fraction::Rational(sign, ratio) => if ratio.is_integer() {
                let num: Result<i64, _> = ratio.to_integer().try_into();
                if let Ok(num) = num {
                    Some(match sign {
                        Sign::Plus => num,
                        Sign::Minus => num,
                    })
                } else {
                    if (ratio.to_integer(), *sign) == (i64::MAX as u64 +1, Sign::Minus) {
                        Some(i64::MIN)
                    } else {None}
                }
            } else {None},
            _ => None,
        }
    }
}

#[test]
fn test() {
    assert_eq!(to_integer::<u64>(&Fraction::from(-1)), None);
    assert_eq!(to_integer::<u64>(&Fraction::from(0)), Some(0));
    assert_eq!(to_integer::<u64>(&Fraction::from(u64::MAX)), Some(u64::MAX));

    assert_eq!(to_integer::<i64>(&Fraction::from(2u64.pow(63))), None);
    assert_eq!(to_integer::<i64>(&Fraction::from(i64::MAX)), Some(i64::MAX));
    assert_eq!(to_integer::<i64>(&Fraction::from(i64::MIN)), Some(i64::MIN));
}