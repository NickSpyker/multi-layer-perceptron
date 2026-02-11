/*
 * Copyright 2026 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_add(c: &mut Criterion) {
    c.bench_function("add u64", |b| {
        b.iter(|| mlp::add(black_box(123_456_789_u64), black_box(987_654_321_u64)));
    });
}

criterion_group!(benches, bench_add);
criterion_main!(benches);
