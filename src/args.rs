use std::{error::Error, fs};

use clap::Parser;

use crate::{school::*, log::{log, Logger}};

// TODO: App description and usage.
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, value_parser)]
    #[cfg(feature = "debug")]
    pub requirements: Option<String>,
    #[cfg(not(feature = "debug"))]
    pub requirements: String,
}

impl Args { 
    pub fn requirements(&self, logger: &mut Logger) -> Result<Requirements, Box<dyn Error>> {
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
