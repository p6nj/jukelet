pub trait Symbol {
    type Env;
    type Output;
    fn zap(self, env: Self::Env) -> (Option<Self::Output>, Self::Env);
}

pub struct Symbols<S>(Vec<S>)
where
    S: Symbol;

impl<S> Symbols<S>
where
    S: Symbol + Clone,
    <S as Symbol>::Env: Default,
{
    pub fn zap(self) -> Vec<S::Output>
    where
        <S as Symbol>::Output: Clone,
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

impl<S> From<Vec<S>> for Symbols<S>
where
    S: Symbol,
{
    fn from(value: Vec<S>) -> Self {
        Symbols(value)
    }
}
