use core::fmt::{Debug, Display};

use anyhow::{Error, Result, bail};

#[allow(dead_code)]
pub fn is_underlying<E>(e: &Error) -> bool
where
    E: Display + Debug + Send + Sync + 'static,
{
    e.downcast_ref::<E>().is_some()
}

#[allow(dead_code)]
pub fn downcast_ref<E>(e: &Error) -> Result<&E>
where
    E: Display + Debug + Send + Sync + 'static,
{
    match e.downcast_ref::<E>() {
        Some(x) => return Ok(x),
        None => bail!("downcast failed"),
    }
}
