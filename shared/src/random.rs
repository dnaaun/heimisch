
pub fn random() -> u32 {
    #[cfg(feature = "hydrate")]
    return (web_sys::js_sys::Math::random() * u32::MAX as f64) as u32;

    #[cfg(not(feature = "hydrate"))]
    return todo!();
}
