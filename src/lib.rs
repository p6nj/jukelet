trait Symbol<E, O> {
    fn zap(self, env: E) -> (O, E);
}

struct Symbols<E, O, S: Symbol<E, O>, I, C: IntoIterator<Item = S, IntoIter = I>>(C);

impl<
        E,
        O,
        S: Symbol<E, O>,
        I1,
        C1: IntoIterator<Item = S, IntoIter = I1>,
        I2,
        C2: IntoIterator<Item = O, IntoIter = I2>,
    > Symbols<E, O, S, I1, C1>
{
    pub fn zap(self) -> C2 {}
    fn zapr(self, out: O, env: E) -> (O, E) {
        let i = self.0.into_iter();
        if let Ok(e) = i.next() {
            let next = e.zap(env);
            next.0.into_iter().chain(self.zapr(out, next.1))
        } else {
            (out, env)
        }
    }
}
