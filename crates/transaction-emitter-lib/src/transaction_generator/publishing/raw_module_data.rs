// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

// This file was generated. Do not modify!
//
// To update this code, run `cargo run` from `testsuite/module-publish` in aptos core.
// That test compiles the set of modules defined in
// `testsuite/simple/src/simple/sources/`
// and it writes the binaries here.
// The module name (prefixed with `MODULE_`) is a `Lazy` instance that returns the
// byte array of the module binary.
// This create should also provide a Rust file that allows proper manipulation of each
// module defined below.

use once_cell::sync::Lazy;

#[rustfmt::skip]
pub static PACKAGE_METADATA_SIMPLE: Lazy<Vec<u8>> = Lazy::new(|| {
	vec![
		13, 71, 101, 110, 101, 114, 105, 99, 77, 111, 100, 117, 108, 101, 1, 0, 0, 0,
		0, 0, 0, 0, 0, 64, 54, 50, 55, 65, 65, 66, 51, 68, 51, 55, 55, 70,
		53, 49, 67, 48, 49, 65, 54, 48, 66, 53, 51, 55, 66, 68, 53, 55, 56, 70,
		50, 57, 50, 65, 67, 70, 68, 50, 55, 67, 48, 55, 67, 54, 70, 69, 68, 54,
		67, 48, 54, 56, 54, 54, 67, 54, 57, 57, 50, 54, 67, 53, 70, 54, 132, 1,
		31, 139, 8, 0, 0, 0, 0, 0, 2, 255, 77, 139, 59, 14, 194, 48, 16, 68,
		251, 61, 133, 229, 30, 135, 11, 80, 208, 64, 197, 9, 162, 20, 43, 123, 64, 86,
		156, 93, 203, 134, 80, 32, 238, 142, 45, 1, 138, 102, 154, 249, 188, 49, 179, 159,
		249, 134, 137, 132, 23, 152, 131, 177, 103, 8, 74, 244, 23, 13, 143, 4, 75, 43,
		74, 141, 42, 125, 217, 187, 38, 75, 52, 6, 100, 72, 128, 248, 136, 58, 209, 49,
		223, 181, 158, 74, 195, 159, 90, 230, 118, 124, 153, 164, 158, 83, 71, 156, 27, 182,
		230, 126, 221, 45, 186, 98, 184, 254, 128, 111, 249, 207, 214, 188, 233, 3, 132, 221,
		66, 189, 150, 0, 0, 0, 1, 6, 83, 105, 109, 112, 108, 101, 0, 0, 0, 3,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 14, 65, 112, 116,
		111, 115, 70, 114, 97, 109, 101, 119, 111, 114, 107, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 1, 11, 65, 112, 116, 111, 115, 83, 116, 100, 108, 105,
		98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 10, 77, 111,
		118, 101, 83, 116, 100, 108, 105, 98, 0,
	]
});

#[rustfmt::skip]
pub static MODULE_SIMPLE: Lazy<Vec<u8>> = Lazy::new(|| {
	vec![
		161, 28, 235, 11, 6, 0, 0, 0, 10, 1, 0, 6, 2, 6, 16, 3, 22, 90,
		5, 112, 139, 1, 7, 251, 1, 245, 1, 8, 240, 3, 64, 6, 176, 4, 115, 10,
		163, 5, 22, 12, 185, 5, 134, 10, 13, 191, 15, 10, 0, 0, 1, 1, 1, 2,
		0, 3, 8, 0, 0, 4, 7, 0, 0, 5, 8, 0, 2, 14, 7, 0, 0, 6,
		0, 1, 0, 0, 7, 2, 3, 0, 0, 8, 4, 1, 0, 0, 9, 4, 1, 0,
		0, 10, 5, 1, 0, 0, 11, 4, 1, 0, 0, 12, 4, 1, 0, 0, 13, 5,
		1, 0, 0, 15, 6, 1, 0, 0, 16, 7, 1, 0, 0, 17, 7, 1, 0, 0,
		18, 4, 1, 0, 0, 19, 4, 1, 0, 0, 20, 5, 1, 0, 0, 21, 8, 1,
		0, 0, 22, 4, 1, 0, 1, 27, 4, 13, 0, 2, 28, 14, 15, 0, 2, 7,
		10, 2, 6, 10, 2, 0, 4, 6, 8, 2, 6, 8, 2, 6, 8, 0, 6, 8,
		0, 1, 6, 3, 1, 6, 12, 2, 6, 12, 3, 4, 6, 12, 3, 8, 3, 10,
		2, 2, 6, 12, 5, 2, 6, 12, 8, 3, 1, 3, 1, 2, 3, 6, 3, 6,
		3, 6, 3, 3, 3, 8, 2, 7, 8, 2, 1, 5, 1, 10, 2, 1, 8, 3,
		3, 10, 3, 10, 3, 3, 3, 8, 1, 7, 8, 2, 8, 2, 8, 1, 10, 2,
		7, 8, 2, 10, 2, 3, 3, 8, 2, 7, 8, 2, 9, 3, 7, 8, 2, 3,
		3, 3, 8, 2, 7, 8, 2, 6, 8, 2, 6, 8, 2, 2, 7, 8, 2, 8,
		2, 2, 8, 2, 7, 8, 2, 1, 7, 8, 0, 6, 83, 105, 109, 112, 108, 101,
		6, 115, 105, 103, 110, 101, 114, 6, 115, 116, 114, 105, 110, 103, 7, 67, 111, 117,
		110, 116, 101, 114, 4, 68, 97, 116, 97, 8, 82, 101, 115, 111, 117, 114, 99, 101,
		11, 97, 112, 112, 101, 110, 100, 95, 100, 97, 116, 97, 14, 99, 111, 112, 121, 95,
		112, 97, 115, 116, 97, 95, 114, 101, 102, 6, 100, 111, 117, 98, 108, 101, 11, 103,
		101, 116, 95, 99, 111, 117, 110, 116, 101, 114, 21, 103, 101, 116, 95, 102, 114, 111,
		109, 95, 114, 97, 110, 100, 111, 109, 95, 99, 111, 110, 115, 116, 4, 104, 97, 108,
		102, 11, 105, 110, 105, 116, 95, 109, 111, 100, 117, 108, 101, 5, 108, 111, 111, 112,
		121, 6, 83, 116, 114, 105, 110, 103, 14, 109, 97, 107, 101, 95, 111, 114, 95, 99,
		104, 97, 110, 103, 101, 8, 109, 97, 120, 105, 109, 105, 122, 101, 8, 109, 105, 110,
		105, 109, 105, 122, 101, 3, 110, 111, 112, 10, 114, 101, 115, 101, 116, 95, 100, 97,
		116, 97, 6, 115, 101, 116, 95, 105, 100, 8, 115, 101, 116, 95, 110, 97, 109, 101,
		4, 115, 116, 101, 112, 5, 99, 111, 117, 110, 116, 4, 100, 97, 116, 97, 2, 105,
		100, 4, 110, 97, 109, 101, 10, 97, 100, 100, 114, 101, 115, 115, 95, 111, 102, 4,
		117, 116, 102, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 171, 205,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 8, 1, 0,
		0, 0, 0, 0, 0, 0, 10, 2, 9, 8, 1, 35, 69, 103, 137, 171, 205, 239,
		10, 2, 6, 5, 104, 101, 108, 108, 111, 10, 3, 81, 10, 0, 0, 0, 0, 0,
		0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
		0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 5,
		0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
		0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0,
		0, 0, 0, 0, 2, 1, 23, 3, 1, 2, 1, 24, 10, 2, 2, 2, 3, 25,
		3, 26, 8, 3, 24, 8, 1, 0, 0, 0, 0, 9, 26, 10, 1, 65, 10, 12,
		2, 10, 2, 6, 0, 0, 0, 0, 0, 0, 0, 0, 36, 4, 21, 5, 8, 10,
		0, 10, 1, 10, 2, 6, 1, 0, 0, 0, 0, 0, 0, 0, 23, 66, 10, 20,
		68, 10, 11, 2, 6, 1, 0, 0, 0, 0, 0, 0, 0, 23, 12, 2, 5, 3,
		11, 1, 1, 11, 0, 1, 2, 1, 0, 0, 0, 11, 103, 10, 0, 16, 0, 12,
		5, 10, 1, 16, 0, 12, 6, 11, 5, 20, 10, 6, 20, 35, 4, 18, 11, 6,
		12, 5, 10, 2, 16, 1, 12, 6, 5, 26, 11, 6, 1, 10, 1, 16, 0, 12,
		5, 10, 3, 16, 1, 12, 6, 10, 6, 20, 10, 1, 16, 0, 20, 35, 4, 47,
		11, 5, 1, 11, 1, 1, 11, 0, 1, 11, 2, 1, 11, 6, 12, 5, 11, 3,
		16, 1, 12, 6, 5, 69, 11, 3, 1, 10, 5, 11, 0, 16, 0, 34, 4, 65,
		11, 6, 1, 11, 5, 1, 11, 2, 16, 1, 12, 5, 11, 1, 16, 0, 12, 6,
		5, 69, 11, 1, 1, 11, 2, 1, 10, 5, 20, 10, 6, 20, 35, 4, 82, 11,
		6, 1, 10, 5, 12, 6, 10, 5, 1, 5, 88, 11, 5, 1, 10, 6, 12, 5,
		10, 6, 1, 10, 5, 10, 6, 33, 4, 97, 11, 6, 1, 11, 5, 12, 4, 5,
		101, 11, 5, 1, 11, 6, 12, 4, 11, 4, 2, 2, 1, 4, 1, 2, 12, 44,
		10, 0, 17, 16, 41, 2, 32, 4, 16, 6, 0, 0, 0, 0, 0, 0, 0, 0,
		7, 2, 17, 17, 7, 1, 18, 1, 18, 2, 12, 2, 11, 0, 11, 2, 45, 2,
		5, 43, 11, 0, 17, 16, 42, 2, 12, 3, 10, 3, 16, 2, 16, 3, 65, 10,
		6, 2, 0, 0, 0, 0, 0, 0, 0, 24, 12, 1, 10, 3, 16, 2, 16, 3,
		65, 10, 10, 1, 35, 4, 41, 5, 35, 10, 3, 15, 2, 15, 3, 49, 255, 68,
		10, 5, 27, 11, 3, 1, 2, 3, 1, 4, 1, 0, 1, 7, 11, 0, 17, 16,
		43, 0, 16, 1, 20, 1, 2, 4, 1, 4, 0, 16, 25, 7, 3, 12, 2, 14,
		2, 65, 9, 12, 4, 10, 4, 6, 0, 0, 0, 0, 0, 0, 0, 0, 34, 4,
		24, 10, 1, 10, 4, 38, 4, 17, 11, 4, 6, 1, 0, 0, 0, 0, 0, 0,
		0, 23, 12, 1, 7, 3, 12, 3, 14, 3, 11, 1, 66, 9, 20, 1, 2, 5,
		1, 4, 1, 2, 12, 44, 10, 0, 17, 16, 41, 2, 32, 4, 16, 6, 0, 0,
		0, 0, 0, 0, 0, 0, 7, 2, 17, 17, 7, 1, 18, 1, 18, 2, 12, 2,
		11, 0, 11, 2, 45, 2, 5, 43, 11, 0, 17, 16, 42, 2, 12, 3, 10, 3,
		16, 2, 16, 3, 65, 10, 6, 2, 0, 0, 0, 0, 0, 0, 0, 26, 12, 1,
		10, 3, 16, 2, 16, 3, 65, 10, 10, 1, 36, 4, 41, 5, 35, 10, 3, 15,
		2, 15, 3, 69, 10, 1, 5, 27, 11, 3, 1, 2, 6, 0, 0, 0, 1, 5,
		11, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 45, 0, 2, 7, 1,
		4, 0, 1, 11, 10, 1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 36, 4, 10,
		5, 5, 11, 1, 6, 1, 0, 0, 0, 0, 0, 0, 0, 23, 12, 1, 5, 0,
		2, 8, 1, 4, 1, 2, 17, 34, 10, 0, 17, 16, 41, 2, 4, 22, 11, 0,
		17, 16, 42, 2, 12, 5, 11, 1, 10, 5, 15, 0, 21, 11, 2, 10, 5, 15,
		4, 21, 11, 3, 11, 5, 15, 2, 15, 3, 21, 5, 33, 11, 3, 18, 1, 12,
		4, 11, 1, 11, 2, 11, 4, 18, 2, 12, 6, 11, 0, 11, 6, 45, 2, 2,
		9, 1, 4, 1, 2, 18, 93, 10, 1, 41, 2, 4, 6, 11, 0, 1, 2, 10,
		0, 17, 16, 41, 2, 32, 4, 21, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7,
		2, 17, 17, 7, 1, 18, 1, 18, 2, 12, 8, 10, 0, 11, 8, 45, 2, 10,
		0, 17, 16, 43, 2, 16, 2, 16, 3, 65, 10, 12, 6, 10, 1, 43, 2, 16,
		2, 16, 3, 65, 10, 12, 7, 11, 6, 11, 7, 36, 4, 49, 11, 0, 17, 16,
		43, 2, 16, 2, 16, 3, 20, 11, 1, 42, 2, 12, 4, 12, 3, 5, 59, 11,
		1, 43, 2, 16, 2, 16, 3, 20, 11, 0, 17, 16, 42, 2, 12, 4, 12, 3,
		11, 3, 11, 4, 12, 9, 12, 5, 14, 5, 65, 10, 10, 9, 16, 2, 16, 3,
		65, 10, 36, 4, 75, 5, 72, 8, 12, 2, 5, 82, 10, 9, 16, 2, 16, 3,
		65, 10, 6, 16, 39, 0, 0, 0, 0, 0, 0, 35, 12, 2, 11, 2, 4, 90,
		10, 9, 15, 2, 15, 3, 14, 5, 17, 0, 5, 63, 11, 9, 1, 2, 10, 1,
		4, 1, 2, 19, 81, 10, 1, 41, 2, 4, 6, 11, 0, 1, 2, 10, 0, 17,
		16, 41, 2, 32, 4, 21, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 2, 17,
		17, 7, 1, 18, 1, 18, 2, 12, 7, 10, 0, 11, 7, 45, 2, 10, 0, 17,
		16, 43, 2, 12, 9, 10, 1, 43, 2, 12, 10, 11, 9, 16, 2, 16, 3, 65,
		10, 11, 10, 16, 2, 16, 3, 65, 10, 12, 5, 12, 4, 10, 4, 10, 5, 36,
		4, 51, 11, 5, 6, 2, 0, 0, 0, 0, 0, 0, 0, 26, 11, 0, 17, 16,
		42, 2, 12, 3, 12, 2, 5, 60, 11, 0, 1, 11, 4, 6, 2, 0, 0, 0,
		0, 0, 0, 0, 26, 11, 1, 42, 2, 12, 3, 12, 2, 11, 2, 11, 3, 12,
		8, 12, 6, 10, 8, 16, 2, 16, 3, 65, 10, 10, 6, 36, 4, 78, 5, 72,
		10, 8, 15, 2, 15, 3, 69, 10, 1, 5, 64, 11, 8, 1, 2, 11, 1, 4,
		0, 1, 1, 2, 12, 1, 4, 1, 2, 20, 34, 10, 0, 17, 16, 41, 2, 4,
		23, 11, 0, 17, 16, 42, 2, 12, 1, 6, 0, 0, 0, 0, 0, 0, 0, 0,
		10, 1, 15, 0, 21, 7, 2, 17, 17, 10, 1, 15, 4, 21, 7, 1, 11, 1,
		15, 2, 15, 3, 21, 5, 33, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 2,
		17, 17, 7, 1, 18, 1, 18, 2, 12, 2, 11, 0, 11, 2, 45, 2, 2, 13,
		1, 4, 1, 2, 21, 25, 10, 0, 17, 16, 41, 2, 32, 4, 16, 11, 1, 7,
		2, 17, 17, 7, 1, 18, 1, 18, 2, 12, 2, 11, 0, 11, 2, 45, 2, 5,
		24, 11, 0, 17, 16, 42, 2, 12, 3, 11, 1, 11, 3, 15, 0, 21, 2, 14,
		1, 4, 1, 2, 21, 24, 10, 0, 17, 16, 41, 2, 32, 4, 15, 6, 0, 0,
		0, 0, 0, 0, 0, 0, 11, 1, 7, 1, 18, 1, 18, 2, 12, 2, 11, 0,
		11, 2, 45, 2, 5, 23, 11, 0, 17, 16, 42, 2, 12, 3, 11, 1, 11, 3,
		15, 4, 21, 2, 15, 1, 4, 1, 0, 22, 13, 11, 0, 17, 16, 42, 0, 12,
		1, 10, 1, 16, 1, 20, 7, 0, 22, 11, 1, 15, 1, 21, 2, 2, 0, 0,
		0, 2, 2, 1, 0, 2, 1, 0,
	]
});
