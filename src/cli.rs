// cli.rs
//
// Copyright (c) 2020 All The Music, LLC
//
// This work is licensed under the Creative Commons Attribution 4.0 International License.
// To view a copy of this license, visit http://creativecommons.org/licenses/by/4.0/ or send
// a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.

use std::str::FromStr;

/*************************
***** Utility Macros *****
*************************/

macro_rules! impl_into {
    ($struct:ty, $field:ident, $target:ty) => {
        impl std::convert::Into<$target> for $struct {
            fn into(self) -> $target {
                self.$field
            }
        }
    }
}

/**********************
***** Error Types *****
**********************/

/// Error type for parsing integers arguments from `&str`
#[derive(Debug, thiserror::Error)]
pub enum ParseNumberArgError {
    #[error(transparent)]
    NotInteger(#[from] std::num::ParseIntError),
    #[error("{arg_name} must be greater than 0")]
    LessThanZero { arg_name: String },
    #[error("{arg_name} must be between {min} and {max}, found {input}")]
    OutOfRange { arg_name: String, min: String, max: String, input: String },
}

/********************
***** BatchSize *****
********************/

fn try_batch_from_str(arg: &str) -> Result<u32, ParseNumberArgError> {
    let batch_size = arg.parse::<u32>()?;
    if batch_size == 0 {
        return Err(ParseNumberArgError::LessThanZero { arg_name: "Batch size".to_string() });
    }
    Ok(batch_size)
}

#[derive(Debug, structopt::StructOpt)]
pub struct BatchSize {
    #[structopt(
        short="s",
        long,
        default_value="25",
        help="Number of melodies per batch",
        parse(try_from_str = try_batch_from_str))]
    pub batch_size: u32,
}

impl_into! { BatchSize, batch_size, u32 }

/***********************
***** MelodyLength *****
***********************/

fn try_length_from_str(arg: &str) -> Result<u32, ParseNumberArgError> {
    let length = arg.parse::<u32>()?;
    if length == 0 {
        return Err(ParseNumberArgError::LessThanZero { arg_name: "Length".to_string() });
    }
    Ok(length)
}

#[derive(Debug, structopt::StructOpt)]
pub struct MelodyLengthArg {
    #[structopt(
        help="Length of melodies (pitch sequences) to generate",
        parse(try_from_str=try_length_from_str))]
    pub melody_length: u32,
}

impl_into! { MelodyLengthArg, melody_length, u32 }

/**************************
***** NoteSet/NoteVec *****
**************************/

#[derive(Debug, structopt::StructOpt)]
pub struct NoteSetArg {
    #[structopt(
        value_name="notes",
        help=concat!("Comma-separated set of NOTE:OCTAVE pairs ",
                     "(i.e., 'C:4,D:4,E:4,F:4,G:4,A:4,B:4,C:5')"),
        parse(try_from_str = libatm::MIDINoteSet::from_str))]
    pub note_set: libatm::MIDINoteSet,
}

impl_into! { NoteSetArg, note_set, libatm::MIDINoteSet }

#[derive(Debug, structopt::StructOpt)]
pub struct NoteVecArg {
    #[structopt(
        value_name="notes",
        help=concat!("Comma-separated sequence of NOTE:OCTAVE pairs ",
                     "(i.e., 'C:4,D:4,E:4,F:4,G:4,A:4,B:4,C:5')"),
        parse(try_from_str = libatm::MIDINoteVec::from_str))]
    pub note_vec: libatm::MIDINoteVec,
}

impl_into! { NoteVecArg, note_vec, libatm::MIDINoteVec }

/************************
***** PartitionArgs *****
************************/

fn try_maxf_from_str(arg: &str) -> Result<f32, ParseNumberArgError> {
    let max_files = arg.parse::<u32>()?;
    if max_files <= 0 || max_files > 4096 {
        return Err(ParseNumberArgError::OutOfRange {
            arg_name: "Max files per directory".to_string(),
            min: "0".to_string(),
            max: "4096".to_string(),
            input: arg.to_string(),
        });
    }
    Ok(max_files as f32)
}

fn try_pdepth_from_str(arg: &str) -> Result<u32, ParseNumberArgError> {
    let partition_depth = arg.parse::<u32>()?;
    if partition_depth == 0 || partition_depth > 4 {
        return Err(ParseNumberArgError::OutOfRange {
            arg_name: "Partition depth".to_string(),
            min: "0".to_string(),
            max: "4".to_string(),
            input: arg.to_string(),
        });
    }
    Ok(partition_depth)
}

#[derive(Debug, structopt::StructOpt)]
pub struct PartitionArgs {
    #[structopt(
        short,
        long,
        default_value="4096",
        help="Maximum number of files per directory",
        parse(try_from_str=try_maxf_from_str))]
    pub max_files: f32,
    #[structopt(
        short="p",
        long = "partitions",
        help = concat!("Partition depth to use for output directory structure.  For ",
                     "example, if set to 2 the ouput directory structure would look ",
                     "like <root>/<branch>/<hash>.mid"),
        parse(try_from_str=try_pdepth_from_str))]
    pub partition_depth: Option<u32>, 
}

/*****************
***** Target *****
*****************/

#[derive(Debug, structopt::StructOpt)]
pub struct TargetArg {
    #[structopt(
        help="File output path (directory/directories must exist)",
        parse(from_str))]
    pub target: std::path::PathBuf,
}

impl_into! { TargetArg, target, std::path::PathBuf }

/******************************
***** CLI Directive Trait *****
******************************/

/// Trait to implement command line directive. Typical implementation
/// will parse the user-provided command line arguments (if any) and run
/// a command or set of commands.
pub trait CliDirective {
    fn run(self);
}

/**************
***** CLI *****
**************/

#[derive(structopt::StructOpt)]
#[structopt(
    about = concat!("Tools for generating and working with MIDI files. ",
                    "This app was created as part of an effort to generate ",
                    "by brute-force billions of melodies, and is tailored for that use case."),
    author = "All The Music, LLC",
    version = env!("CARGO_PKG_VERSION"),
    setting=structopt::clap::AppSettings::ArgRequiredElseHelp)]
pub enum Cli {
    Gen(crate::directives::GenDirective),
}

impl CliDirective for Cli {
    fn run(self) {
        match self {
            Self::Gen(d) => d.run(),
        }
    }
}
