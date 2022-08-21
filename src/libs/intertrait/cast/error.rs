#[derive(thiserror::Error, Debug)]
pub enum CastError
{
    #[error("Failed to cast from trait {from} to trait {to}")]
    CastFailed
    {
        from: &'static str,
        to: &'static str,
    },
}
