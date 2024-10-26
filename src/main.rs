// Copyright 2024 Adam Burucs. MIT license.
use anyhow::{Context, Error, Result};
use hound::WavReader;
use std::{env, fs::File, io::BufReader};

fn to_rms(sample_value: f64, length: u32) -> f64 {
    (sample_value / length as f64).sqrt()
}

fn to_dbfs(rms: f64) -> f64 {
    let max_value: f64 = 32768.0; // Maximum value for i16 as float
    let rms_dbfs: f64 = 20.0 * rms.log10() - 20.0 * (max_value.log10());
    rms_dbfs
}

fn round_to_decimals(dbfs: f64) -> f64 {
    (dbfs * 100.0).round() / 100.0
}

fn calculate_square_sum(reader: &mut WavReader<BufReader<File>>) -> Result<f64, Error> {
    reader.samples::<i16>().try_fold(0.0, |sqr_sum, s| {
        let sample = s.context("Failed during reading samples")?;
        let sample_f64 = sample as f64;
        Ok(sqr_sum + sample_f64 * sample_f64)
    })
}

fn calc_file_rms(file: &String) -> Result<f64> {
    let mut reader =
        WavReader::open(file).with_context(|| format!("Failed to open file from {file}"))?;

    let sqr_sum = calculate_square_sum(&mut reader)?;
    let rms = to_rms(sqr_sum, reader.len());
    let dbfs = to_dbfs(rms);
    let rounded = round_to_decimals(dbfs);

    Ok(rounded)
}

// TODO
// use clap crate for CLI
fn main() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    println!("Loudness Addict {version}");
    println!();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Parameters");
        println!("{:?}", args);
        println!();

        for file in args.iter().skip(1) {
            let file_rms = calc_file_rms(file)?;
            println!("{file} Int. RMS: {file_rms}");
        }
    } else {
        println!("Please set the parameter list! eg. testsamples/beat.wav");
    }
    Ok(())
}
