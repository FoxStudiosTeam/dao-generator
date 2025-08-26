


pub trait ErrOr<T> where Self: Sized {
    fn or_trace_warn<E>(self, msg: &'static str) -> anyhow::Result<T>;
    fn or_trace_err<E>(self, msg: &'static str) -> anyhow::Result<T>;
    fn or_err<E>(self, msg: &'static str) -> anyhow::Result<T>;
}

impl<T> ErrOr<T> for Result<T, String> {
    fn or_trace_warn<E>(self, msg: &'static str) -> anyhow::Result<T> { 
        match self {
            Ok(v) => Ok(v),
            Err(_) => {
                tracing::warn!("{}", msg);
                anyhow::bail!(msg)
            }
        }
    }
    fn or_trace_err<E>(self, msg: &'static str) -> anyhow::Result<T> { 
        match self {
            Ok(v) => Ok(v),
            Err(_) => {
                tracing::error!("{}", msg);
                anyhow::bail!(msg)
            }
        }
    }
    fn or_err<E>(self, msg: &'static str) -> anyhow::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => anyhow::bail!(msg)
        }
    }
}

impl<T> ErrOr<T> for Option<T> {
    fn or_trace_warn<E>(self, msg: &'static str) -> anyhow::Result<T> { 
        match self {
            Some(v) => Ok(v),
            None => {
                tracing::warn!("{}", msg);
                anyhow::bail!(msg)
            }
        }
    }
    fn or_trace_err<E>(self, msg: &'static str) -> anyhow::Result<T> { 
        match self {
            Some(v) => Ok(v),
            None => {
                tracing::error!("{}", msg);
                anyhow::bail!(msg)
            }
        }
    }
    fn or_err<E>(self, msg: &'static str) -> anyhow::Result<T> {
        match self {
            Some(v) => Ok(v),
            None => anyhow::bail!(msg)
        }
    }
}