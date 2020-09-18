/// This Source Code Form is subject to the terms of the Mozilla Public
/// License, v. 2.0. If a copy of the MPL was not distributed with this
/// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[test]
fn hello_eq() {
    assert_eq!("Hello, World", "Hello, World");
}

#[test]
fn hello_neq() {
    assert_ne!("Hello, World", "hello, world");
}
