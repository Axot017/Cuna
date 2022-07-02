use std::future::Future;

use crate::model::profile_creation_data::ProfileCreationData;

pub trait CreateProfile<'a, E>: FnOnce(E, &'a ProfileCreationData) -> Self::OutputFuture {
    type OutputFuture: Future<Output = <Self as CreateProfile<'a, E>>::Output>;

    type Output;
}
