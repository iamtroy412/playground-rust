//! Higher-level biological structures.
//!
//! We always simulate a sample of all chemical interactions at the cellular
//! level, but simulating everything that way is just too computationally
//! expensive.  Therefore we keep higher-level data structures representing
//! each fern's roots, leaves, and so on.  When we simulate physics (light, air
//! currents, gravity) we always use these structures as shorthand for the
//! millions of cells they typically represent. On a more morbid note, these
//! structures stick around when stuff dies, so that dead fronds have weight,
//! cast shadows, and so on.

// in plant_structures/mod.rs
pub mod roots;
pub mod stems;
pub mod leaves;
