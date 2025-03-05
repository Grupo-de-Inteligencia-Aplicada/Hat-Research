pub trait TryAdd: Sized {
    fn try_add(self, rhs: Self) -> anyhow::Result<Self>;
}

pub trait TrySub: Sized {
    fn try_sub(self, rhs: Self) -> anyhow::Result<Self>;
}

pub trait TryMul: Sized {
    fn try_mul(self, rhs: Self) -> anyhow::Result<Self>;
}

pub trait TryDiv: Sized {
    fn try_div(self, rhs: Self) -> anyhow::Result<Self>;
}
