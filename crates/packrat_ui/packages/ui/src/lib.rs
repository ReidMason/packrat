//! This crate contains all shared UI for the workspace.
mod tailwind;
pub use tailwind::TailwindConfig;

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

#[cfg(feature = "demo_echo")]
mod echo;
#[cfg(feature = "demo_echo")]
pub use echo::Echo;
