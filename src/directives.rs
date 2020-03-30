// directives.rs
//
// Copyright (c) 2020 All The Music, LLC
//
// This work is licensed under the Creative Commons Attribution 4.0 International License.
// To view a copy of this license, visit http://creativecommons.org/licenses/by/4.0/ or send
// a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

pub mod gen;
mod gen_single;
mod gen_tar;
mod gen_tar_gz;
mod gen_batch;

pub use gen::GenDirective;
pub use gen_single::GenSingleDirective;
pub use gen_tar::GenTarDirective;
pub use gen_tar_gz::GenTarGzDirective;
pub use gen_batch::GenBatchDirective;
