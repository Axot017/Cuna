use std::future::Future;

use crate::model::profile_creation_data::ProfileCreationData;

pub trait CreateProfile<'a, E>: FnOnce(E, &'a ProfileCreationData) -> Self::OutputFuture {
    type OutputFuture: Future<Output = <Self as CreateProfile<'a, E>>::Output>;

    type Output;
}

impl<'a, F, FUT, E> CreateProfile<'a, E> for F
where
    F: FnOnce(E, &'a ProfileCreationData) -> FUT,
    FUT: Future,
{
    type OutputFuture = FUT;

    type Output = FUT::Output;
}
