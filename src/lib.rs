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

pub trait Extend<E> {
    type Output;
    fn extend(self, element: E) -> Self::Output;
}

impl<E, T> Extend<E> for T
where
    T: IntoIterator<Item = E> + FromIterator<E>,
    E: Clone,
{
    type Output = T;
    fn extend(self, element: E) -> Self::Output {
        self.into_iter().chain([element].iter().cloned()).collect()
    }
}

impl<S, C, I> Symbols<S, C, I>
where
    S: Symbol + Clone,
    <S as Symbol>::Env: Default,
    C: IntoIterator<Item = S, IntoIter = I>,
    I: Iterator<Item = S>,
{
    pub fn zap<O1, O2, I2>(self) -> O1
    where
        <S as Symbol>::Output: Clone,
        O1: FromIterator<<S as Symbol>::Output>,
        O2: Default
            + Extend<Option<<S as Symbol>::Output>, Output = O2>
            + IntoIterator<Item = Option<<S as Symbol>::Output>, IntoIter = I2>,
        I2: Iterator<Item = Option<<S as Symbol>::Output>>,
    {
        self.0
            .into_iter()
            .fold((O2::default(), S::Env::default()), move |(acc, env), s| {
                let (o, e) = s.zap(env);
                (acc.extend(o), e)
            })
            .0
            .into_iter()
            .flatten()
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
