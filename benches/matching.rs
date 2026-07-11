//! Benchmarks the current SimpleMatch implementation (backed by the internal, allocation-minimal
//! ci_engine module) against the older allocate-then-compare formula it replaced
//! (`self.to_lowercase().strip_non_alphanum().starts_with(&pattern.to_lowercase())`, etc.),
//! reimplemented inline here so this bench doesn't depend on an older published version of
//! this same crate. Regex is deliberately not part of this comparison: this crate excludes
//! regex as a dependency, even for benchmarking, so any regex comparison is run separately
//! outside the repo.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple_string_patterns::SimpleMatch;

// Deterministic xorshift PRNG so benchmark inputs are reproducible without a `rand` dependency.
struct Xorshift(u64);
impl Xorshift {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn range(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

const NOISE_WORDS: &[&str] = &[
    "connection", "established", "user", "request", "processed", "timeout", "retry",
    "system", "memory", "usage", "cpu", "load", "average", "shutdown", "started",
    "completed", "elapsed", "bytes", "sent", "received", "session", "token", "cache",
    "hit", "miss", "queue", "worker", "thread", "pool", "handler", "route", "dispatch",
    "config", "loaded", "database", "query", "index", "table", "column", "row", "commit",
    "rollback", "lock", "release", "socket", "bind", "listen", "accept", "close",
];

fn rand_words(rng: &mut Xorshift, n: usize) -> String {
    (0..n)
        .map(|_| NOISE_WORDS[rng.range(NOISE_WORDS.len())])
        .collect::<Vec<_>>()
        .join(" ")
}

fn rand_prefix(rng: &mut Xorshift) -> String {
    match rng.range(4) {
        0 => format!("[2024-06-{:02}T12:{:02}:00Z] ", 1 + rng.range(28), rng.range(60)),
        1 => "[INFO] ".to_string(),
        2 => "".to_string(),
        _ => format!("{:05} ", rng.range(99999)),
    }
}

/// 0 = neither "float" nor "error", 1 = "error" only, 2 = "float" prefix only, 3 = both
fn gen_line(rng: &mut Xorshift, category: u8) -> String {
    let prefix = rand_prefix(rng);
    match category {
        0 => format!("{}{}", prefix, rand_words(rng, 6)),
        1 => format!("{}an error occurred while processing: {}", prefix, rand_words(rng, 4)),
        2 => format!("{}floating point conversion succeeded {}", prefix, rand_words(rng, 4)),
        _ => format!("{}floating point error detected at line {}", prefix, rng.range(9999)),
    }
}

/// Realistic-ish log-line distribution: most lines match neither term.
fn synthetic_lines(n: usize) -> Vec<String> {
    let mut rng = Xorshift(0x9E3779B97F4A7C15);
    (0..n)
        .map(|_| {
            let roll = rng.range(100);
            let category = if roll < 80 {
                0
            } else if roll < 92 {
                1
            } else if roll < 97 {
                2
            } else {
                3
            };
            gen_line(&mut rng, category)
        })
        .collect()
}

/// The allocate-then-compare baseline this crate's SimpleMatch impl used to use directly:
/// builds a full lower-cased (and, for the alphanumeric variant, punctuation-stripped) copy
/// of the whole subject on every call, regardless of how early the answer is knowable.
mod old {
    fn strip_non_alphanum(s: &str) -> String {
        s.chars().filter(|c| c.is_alphanumeric()).collect()
    }

    pub fn starts_with_ci_alphanum(s: &str, pattern: &str) -> bool {
        strip_non_alphanum(&s.to_lowercase()).starts_with(&pattern.to_lowercase())
    }

    pub fn contains_ci(s: &str, pattern: &str) -> bool {
        s.to_lowercase().contains(&pattern.to_lowercase())
    }
}

fn bench_starts_with_ci_alphanum(c: &mut Criterion) {
    let lines = synthetic_lines(2_000);
    let mut group = c.benchmark_group("starts_with_ci_alphanum");
    group.bench_function("old_alloc", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| old::starts_with_ci_alphanum(black_box(l), black_box("float")))
                .count()
        })
    });
    group.bench_function("ci_engine", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| black_box(l.as_str()).starts_with_ci_alphanum(black_box("float")))
                .count()
        })
    });
    group.finish();
}

fn bench_contains_ci(c: &mut Criterion) {
    let lines = synthetic_lines(2_000);
    let mut group = c.benchmark_group("contains_ci");
    group.bench_function("old_alloc", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| old::contains_ci(black_box(l), black_box("error")))
                .count()
        })
    });
    group.bench_function("ci_engine", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| black_box(l.as_str()).contains_ci(black_box("error")))
                .count()
        })
    });
    group.finish();
}

/// The realistic compound case: "starts with float (ignoring punctuation) AND contains error".
fn bench_compound_filter(c: &mut Criterion) {
    let lines = synthetic_lines(2_000);
    let mut group = c.benchmark_group("compound_starts_and_contains");
    group.bench_function("old_alloc", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| {
                    old::starts_with_ci_alphanum(black_box(l), "float")
                        && old::contains_ci(black_box(l), "error")
                })
                .count()
        })
    });
    group.bench_function("ci_engine", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| {
                    black_box(l.as_str()).starts_with_ci_alphanum("float")
                        && black_box(l.as_str()).contains_ci("error")
                })
                .count()
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_starts_with_ci_alphanum,
    bench_contains_ci,
    bench_compound_filter
);
criterion_main!(benches);
