trait Symbols {
    type Output;
    type Env;
    fn zap(self) -> Self::Output;
}

trait Symbol<S>
where
    S: Symbols,
{
    fn zap(self, env: S::Env, next: S) -> (S::Output, (S::Env, S));
}

impl<S, S1, I, O, E> Symbols for S
where
    S: IntoIterator<Item = S1, IntoIter = I>,
    S1: Symbol,
{
    type Output = O;
    type Env = E;
    fn zap(self) -> Self::Output {
        if let Ok(next) = self.into_iter().next() {
            next.zap()
        }
    }
}

// TODO: turn that into a derive macro
// https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
