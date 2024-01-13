use std::{array::IntoIter, iter::Chain, ops::Add};

pub trait Symbol {
    type Env;
    type Output;
    fn zap(self, env: Self::Env) -> (Self::Output, Self::Env);
}

pub struct Symbols<S, I, C>(C)
where
    S: Symbol,
    C: IntoIterator<Item = S, IntoIter = I>;

impl<S, I1, C1> Symbols<S, I1, C1>
where
    S: Symbol + Clone,
    <S as Symbol>::Env: Default,
    I1: Iterator<Item = S>,
    C1: IntoIterator<Item = S, IntoIter = I1>,
{
    pub fn zap<
        I2: Iterator<Item = S::Output>,
        C2: IntoIterator<Item = S::Output, IntoIter = I2>
            + Default
            + From<Chain<IntoIter<S::Output, 1>, I2>>
            + Add<<S as Symbol>::Output, Output = C2>,
    >(
        self,
    ) -> C2 {
        self.0
            .into_iter()
            .fold((C2::default(), S::Env::default()), move |(acc, env), s| {
                let (o, e) = s.zap(env);
                (acc + o, e)
            })
            .0
    }
}

impl<S, I, C> From<C> for Symbols<S, I, C>
where
    S: Symbol,
    C: IntoIterator<Item = S, IntoIter = I>,
{
    fn from(value: C) -> Self {
        Symbols(value)
    }
}
