use std::{sync::atomic::AtomicU64, time::Instant};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use fixed_map::{Key, Map};
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Key, EnumIter, Debug)]
pub enum PerfParts {
    UniqueMoves,
    UniqueMovesDouble,
    UniqueMovesNormalDice,

    UniqueOneDie,
    OneDie,

    // one die:
    Bar,
    Removals,
    Calc,
    GenBoards,
}

pub static PERF_MAP: Lazy<Map<PerfParts, AtomicU64>> = Lazy::new(|| {
    let mut m = Map::new();
    for v in PerfParts::iter() {
        m.insert(v, AtomicU64::new(0));
    }
    m
});

#[cfg(feature = "time")]
#[inline(always)]
pub fn time<T>(key: PerfParts, f: impl FnOnce() -> T) -> T {
    let now = Instant::now();

    let result = f();

    let elapsed = now.elapsed().as_nanos() as u64;

    (*PERF_MAP)
        .get(key)
        .unwrap()
        .fetch_add(elapsed, std::sync::atomic::Ordering::Relaxed);

    result
}

#[cfg(not(feature = "time"))]
#[inline(always)]
pub fn time<T>(_key: PerfParts, f: impl FnOnce() -> T) -> T {
    f()
}
