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
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_docs_in_private_items,
    clippy::missing_panics_doc,
    missing_docs
)]

#[test]
fn add_two_numbers() {
    assert_eq!(mlp::add(2, 3), 5);
}

#[test]
fn add_with_zero() {
    assert_eq!(mlp::add(0, 100), 100);
    assert_eq!(mlp::add(100, 0), 100);
}

#[test]
fn add_large_values() {
    assert_eq!(mlp::add(u64::MAX, 0), u64::MAX);
}
