//! This crate contains all shared UI for the workspace.
mod tailwind;
pub use tailwind::{TailwindConfig, ThemeToggle};

mod hero;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;
