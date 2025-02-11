pub fn random() -> u32 {
    #[cfg(target_arch = "wasm32")]
    return (web_sys::js_sys::Math::random() * u32::MAX as f64) as u32;

    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unreachable_code)]
    return todo!();
}
