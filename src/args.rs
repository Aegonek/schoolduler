use std::{error::Error, fs};

use clap::Parser;

use crate::{algen::params::Params, school::*, log::{log, LogHandle}};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, value_parser)]
    pub params: Option<String>,

    #[clap(long, value_parser)]
    #[cfg(feature = "debug")]
    pub requirements: Option<String>,
    #[cfg(not(feature = "debug"))]
    pub requirements: String,
}

impl Args {
    pub fn params(&self, logger: &mut LogHandle) -> Result<Params, Box<dyn Error>> {
        match &self.params {
            Some(path) => {
                log!(logger, "Loading algorithm parameters from file {}...", path);
                let raw = String::from_utf8(fs::read(path)?)?;
                log!(logger, "{raw}");
                let de = serde_json::from_str(&raw)?;
                Ok(de)
            }
            None => {
                log!(logger, "Using default parameters for algorithm...");
                let params = Params::default();
                log!(logger, "{}", serde_json::to_string(&params)?);
                Ok(params)
            },
        }
    }
    
    pub fn requirements(&self, logger: &mut LogHandle) -> Result<Requirements, Box<dyn Error>> {
        #[cfg(feature = "debug")]
        match &self.requirements {
            Some(path) => {
                log!(logger, "Loading requirements from file {}...", path);
                let raw = String::from_utf8(fs::read(path)?)?;
                let de = serde_json::from_str(&raw)?;
                Ok(de)
            }
            None => {
                log!(logger, "Debug mode: using example requirements.");
                const RAW: &'static str = include_str!(r"..\debug\courses.json");
                let de = serde_json::from_str(&RAW)?;
                Ok(de)
            }
        }

        #[cfg(not(feature = "debug"))]
        {
            log!(logger, "Loading requirements from file {}...", &self.requirements);
            let raw = String::from_utf8(fs::read(&self.requirements)?)?;
            let de = serde_json::from_str(&raw)?;
            Ok(de)
        }
    }
}
