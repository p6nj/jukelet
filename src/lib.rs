trait Symbol<E, O> {
    fn zap(self, env: &mut E) -> O;
}

struct Symbols<E, O, S: Symbol<E, O>, I, C: IntoIterator<Item = S, IntoIter = I>>(C);

impl<E, O, S, I1, C1, I2, C2> Symbols<E, O, S, I1, C1>
where
    S: Symbol<E, O>,
    I1: Iterator<Item = S>,
    C1: IntoIterator<Item = S, IntoIter = I1>,
    I2: Iterator<Item = O>,
    C2: IntoIterator<Item = O, IntoIter = I2>,
{
    pub fn zap(self) -> C2 {}
    fn zapr(self, out: C2, env: &mut E) -> C2 {
        let i = self.0.into_iter();
        if let Ok(e) = i.next() {
            let next = e.zap(&mut env);
            next.0.into_iter().chain(self.zapr(out, env))
        } else {
            out
        }
    }
}
