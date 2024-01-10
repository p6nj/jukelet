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

// TODO: turn that into a derive macro
// https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
