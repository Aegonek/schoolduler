use std::{error::Error, fmt::Display, fs, path::PathBuf};

use clap::Parser;

use crate::{algen::params::Params, domain::*};

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
    pub fn params(&self) -> Result<Params, Box<dyn Error>> {
        match &self.params {
            Some(path) => {
                let raw = String::from_utf8(fs::read(path)?)?;
                let de = serde_json::from_str(&raw)?;
                Ok(de)
            }
            None => Ok(Params::default()),
        }
    }
    
    pub fn requirements(&self) -> Result<Requirements, Box<dyn Error>> {
        #[cfg(feature = "debug")]
        match &self.requirements {
            Some(path) => {
                let raw = String::from_utf8(fs::read(path)?)?;
                let de = serde_json::from_str(&raw)?;
                Ok(de)
            }
            None => {
                const RAW: &'static str = include_str!(r"..\input\example-courses.json");
                let de = serde_json::from_str(&RAW)?;
                Ok(de)
            }
        }

        #[cfg(not(feature = "debug"))]
        {
            let raw = String::from_utf8(fs::read(&self.requirements)?)?;
            let de = serde_json::from_str(&raw)?;
            Ok(de)
        }
    }
}
