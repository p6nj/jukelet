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
    <S as Symbol>::Env: Default,
    C: IntoIterator<Item = S, IntoIter = I>,
    I: Iterator<Item = S>,
{
    pub fn zap<O>(self) -> O
    where
        <S as Symbol>::Output: Clone,
        O: FromIterator<<S as Symbol>::Output>,
    {
        self.0
            .into_iter()
            .fold((Vec::new(), S::Env::default()), move |(acc, env), s| {
                let (o, e) = s.zap(env);
                ([acc, vec![o]].concat(), e)
            })
            .0
            .iter()
            .flatten()
            .cloned()
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
