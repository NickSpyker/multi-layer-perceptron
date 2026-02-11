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

//! # multi-layer-perceptron
//!
//! A general-purpose Rust library implementing Multi-Layer Perceptrons for flexible neural network tasks.

/// Returns the sum of two unsigned 64-bit integers.
#[must_use]
pub const fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
#[allow(clippy::missing_panics_doc)]
mod tests {
    use super::*;

    #[test]
    fn add_two_small_numbers() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn add_with_zero() {
        assert_eq!(add(0, 42), 42);
        assert_eq!(add(42, 0), 42);
    }

    #[test]
    fn add_max_values() {
        assert_eq!(add(u64::MAX, 0), u64::MAX);
    }
}
