#![no_std]

pub trait Symbol {
    type Env;
    type Output;
    fn zap(self, env: Self::Env) -> (Option<Self::Output>, Self::Env);
}

pub struct Symbols<S, C, I>(C)
where
    C: IntoIterator<Item = S, IntoIter = I>,
    S: Symbol;

impl<S, C, I> Symbols<S, C, I>
where
    S: Symbol + Clone,
    C: IntoIterator<Item = S, IntoIter = I>,
    I: Iterator<Item = S>,
{
    pub fn zap<O, O1, I1>(self, acc: O1, env: S::Env) -> O
    where
        <S as Symbol>::Output: Clone,
        O1: IntoIterator<Item = <S as Symbol>::Output, IntoIter = I1>
            + Extend<<S as Symbol>::Output>,
        O: FromIterator<<S as Symbol>::Output>,
        I1: Iterator<Item = <S as Symbol>::Output>,
    {
        self.0
            .into_iter()
            .fold((acc, env), move |(mut acc, env), s| {
                let (o, e) = s.zap(env);
                acc.extend(o);
                (acc, e)
            })
            .0
            .into_iter()
            .collect()
    }
}

impl<S, C, I> From<C> for Symbols<S, C, I>
where
    S: Symbol,
    C: IntoIterator<Item = S, IntoIter = I>,
{
    fn from(value: C) -> Self {
        Symbols(value)
    }
}
