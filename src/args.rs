use std::{error::Error, fs, path::PathBuf};

use clap::Parser;

use crate::{algen::params::Params, domain::*, log::{log, Logger}};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, value_parser)]
    pub params: Option<PathBuf>,

    #[clap(long, value_parser)]
    #[cfg(feature = "debug")]
    pub requirements: Option<PathBuf>,
    #[cfg(not(feature = "debug"))]
    pub requirements: PathBuf,
}

impl Args {
    pub fn params(&self, logger: &mut Logger) -> Result<Params, Box<dyn Error>> {
        match &self.params {
            Some(path) => {
                log!(logger, "Loading algorithm parameters from file {}...", path.to_string_lossy())?;
                let raw = String::from_utf8(fs::read(path)?)?;
                log!(logger, "{raw}");
                let de = serde_json::from_str(&raw)?;
                Ok(de)
            }
            None => {
                log!(logger, "Using default parameters for algorithm...")?;
                let params = Params::default();
                log!(logger, "{}", serde_json::to_string(&params)?);
                Ok(params)
            },
        }
    }
    
    pub fn requirements(&self, logger: &mut Logger) -> Result<Requirements, Box<dyn Error>> {
        #[cfg(feature = "debug")]
        match &self.requirements {
            Some(path) => {
                log!(logger, "Loading requirements from file {}...", path.to_string_lossy())?;
                let raw = String::from_utf8(fs::read(path)?)?;
                let de = serde_json::from_str(&raw)?;
                Ok(de)
            }
            None => {
                log!(logger, "Debug mode: using example requirements.")?;
                const RAW: &'static str = include_str!(r"..\input\example-courses.json");
                let de = serde_json::from_str(&RAW)?;
                Ok(de)
            }
        }

        #[cfg(not(feature = "debug"))]
        {
            log!(logger, "Loading requirements from file {}...", &self.requirements.to_string_lossy())?;
            let raw = String::from_utf8(fs::read(&self.requirements)?)?;
            let de = serde_json::from_str(&raw)?;
            Ok(de)
        }
    }
}
