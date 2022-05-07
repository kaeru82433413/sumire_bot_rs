use serenity::prelude::*;
use serenity::{
    model::{guild::Member, channel::Message},
    framework::standard::{Args, ArgError}
};
use std::str::FromStr;
use std::fmt::{self, Display, Debug};
use std::error::Error;
use crate::consts;
use fraction::Fraction;
use crate::utils::expression::{self, ExpressionError};



#[derive(Debug)]
pub struct ParseError<E: Debug>(pub String, pub E);

impl<E: Debug> Display for ParseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError({:?})", self.1)?;
        Ok(())
    }
}

impl<E: Debug> Error for ParseError<E> {}



#[derive(Debug)]
pub struct Eos;

impl Display for Eos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reached end of args")?;
        Ok(())
    }
}

impl Error for Eos {}



pub struct ArgsWrapper(pub Args);

impl ArgsWrapper {
    pub fn string(&mut self) -> Result<String, Eos> {
        match self.0.single() {
            Ok(raw) => Ok(raw),
            Err(error) => match error {
                ArgError::Eos => Err(Eos),
                _ => unreachable!(),
            }
        }
    }

    pub fn parse<T>(&mut self) -> Result<Result<T, Eos>, ParseError<T::Err>> where
        T: FromStr,
        T::Err: Debug,
    {
        let raw: String = match self.0.single() {
            Ok(raw) => raw,
            Err(error) => match error {
                ArgError::Eos => return Ok(Err(Eos)),
                _ => unreachable!(),
            }
        };
        match raw.parse() {
            Ok(value) => Ok(Ok(value)),
            Err(error) => Err(ParseError(raw, error)),
        }
    }

    pub fn expression(&mut self) -> Result<Result<Fraction, Eos>, ExpressionError> where {
        let raw = match self.string() {
            Ok(raw) => raw,
            Err(eos) => return Ok(Err(eos)),
        };
        
        expression::calc(&raw).map(|x| Ok(x))
    }

    pub async fn member(ctx: &Context, msg: &Message, query: &str) -> Result<Option<Member>, SerenityError> {
        let members = ctx.http.get_guild_members(consts::SUMIRE_GUILD.0, None, None).await?;

        let mut res = vec![];
        for member in members.iter() {
            todo!("{:?} {} {}", msg, query, member)
        }
        if res.len() == 1 {
            return Ok(Some(res.pop().unwrap()))
        }
        todo!()
    }
}