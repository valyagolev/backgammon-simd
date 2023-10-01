backgammon-simd
===============

<a href="https://crates.io/crates/backgammon-simd"><img alt="Crate Info" src="https://img.shields.io/crates/v/backgammon-simd.svg"/></a>
<a href="https://docs.rs/backgammon-simd/"><img alt="API Docs" src="https://img.shields.io/docsrs/backgammon-simd"/></a>


Type-safe valid move generator for backgammon using SIMD instructions. Useful for e.g. ML stuff.

Is it actually **very fast**? Probably not. Currently takes 7µs per board on my slow box (~140k/s). Let me know who to compete with. But it's nice to use and doesn't feel like a complete waste, performance-wise.

I only tested it using nightly, it wants `#![feature(portable_simd)]`.

Probably a lot of room for improvement ;-)

