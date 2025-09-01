pub fn random() -> u32 {
    #[cfg(feature = "hydrate")]
    return (web_sys::js_sys::Math::random() * u32::MAX as f64) as u32;

    #[cfg(feature = "ssr")]
    #[allow(unreachable_code)]
    return {
        use std::sync::atomic::{AtomicU32, Ordering};
        static COUNTER: AtomicU32 = AtomicU32::new(1);

        let x = COUNTER.fetch_add(1, Ordering::Relaxed);
        x.wrapping_mul(1103515245).wrapping_add(12345)
    };
}
