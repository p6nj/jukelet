use std::{array::IntoIter, iter::Chain};

trait Symbol {
    fn zap<E, O>(self, env: &mut E) -> O;
}

struct Symbols<S, I, C>(C)
where
    S: Symbol,
    C: IntoIterator<Item = S, IntoIter = I>;

impl<S, I1, C1> Symbols<S, I1, C1>
where
    S: Symbol + Clone,
    I1: Iterator<Item = S>,
    C1: IntoIterator<Item = S, IntoIter = I1>,
{
    pub fn zap<
        E: Default,
        O,
        I2: Iterator<Item = O>,
        C2: IntoIterator<Item = O, IntoIter = I2> + Default + From<Chain<IntoIter<O, 1>, I2>>,
    >(
        self,
    ) -> C2 {
        self.zapr(C2::default(), &mut E::default())
    }
    fn zapr<
        E,
        O,
        I2: Iterator<Item = O>,
        C2: IntoIterator<Item = O, IntoIter = I2> + From<Chain<IntoIter<O, 1>, I2>>,
    >(
        self,
        out: C2,
        env: &mut E,
    ) -> C2 {
        let e = self.0.into_iter().next().clone();
        if let Some(e) = e {
            [e.zap(env)].into_iter().chain(self.zapr(out, env)).into()
        } else {
            out
        }
    }
}
