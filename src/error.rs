use core::fmt::{Debug, Display};

use anyhow::Error;

#[allow(dead_code)]
pub fn is_underlying<E>(e: &Error) -> bool
where
    E: Display + Debug + Send + Sync + 'static,
{
    e.downcast_ref::<E>().is_some()
}
