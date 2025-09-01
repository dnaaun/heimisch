#[cfg(all(feature = "ssr", not(feature = "hydrate")))]
pub mod monotonic_time_ssr;
#[cfg(all(feature = "ssr", not(feature = "hydrate")))]
pub use monotonic_time_ssr as monotonic_time;

#[cfg(feature = "hydrate")]
pub mod monotonic_time_hydrate;
#[cfg(feature = "hydrate")]
pub use monotonic_time_hydrate as monotonic_time;

pub mod db;
mod optimistic_change_map;
mod optimistic_changes;
