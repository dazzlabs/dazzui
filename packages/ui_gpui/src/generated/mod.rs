//! What `packages/ds` wrote.
//!
//! The two files beside this one are generated and committed, so the crate
//! builds and ships on its own — there is no dependency edge to the design
//! system and so no build order to get wrong. **Never hand-edit them**: a
//! value a component needs belongs in `packages/ds`, and anything written
//! here is overwritten on the next `pnpm tokens`.
//!
//! This index is the exception. Rust needs a module to reach them by, and
//! `packages/ds` writes files rather than directories, so it is written by
//! hand and holds nothing but the two declarations.

pub mod colors;
pub mod theme_variables;
