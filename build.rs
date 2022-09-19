use std::{
    env,
    error::Error,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use serde_yaml::value::TaggedValue;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("config.yaml").unwrap());
    let cfg = serde_yaml::from_reader::<_, serde_yaml::Value>(reader).unwrap();
    let mutation_probability = {
        let tagged = cfg
            .get("mutation-probability")
            .and_then(|x| x.as_tagged())
            .ok_or(parse_error!())?;
        parse_probability(&tagged).ok_or(parse_error!())?
    };
    let crossover_probability = {
        let tagged = cfg
            .get("crossover-probability")
            .and_then(|x| x.as_tagged())
            .ok_or(parse_error!())?;
        parse_probability(&tagged).ok_or(parse_error!())?
    };
    let population_size = cfg
        .get("population-size")
        .and_then(|x| x.as_u64())
        .ok_or(parse_error!())?;
    let children_per_parent = cfg
        .get("children-per-parent")
        .and_then(|x| x.as_u64())
        .ok_or(parse_error!())?;

    let cfg_src = format!(
        r#"
crate::algen::config::Config {{
    population_size: {population_size},
    mutation_probability: {mutation_probability},
    crossover_probability: {crossover_probability},
    children_per_parent: {children_per_parent}
}}
"#
    );
    let config_dest = Path::new(&env::var_os("OUT_DIR").unwrap()).join("config.generated.rs");
    fs::write(&config_dest, cfg_src).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

trait YamlValueExt {
    fn as_tagged(&self) -> Option<Box<serde_yaml::value::TaggedValue>>;
}

impl YamlValueExt for serde_yaml::Value {
    fn as_tagged(&self) -> Option<Box<serde_yaml::value::TaggedValue>> {
        match self {
            serde_yaml::Value::Tagged(tagged) => Some(tagged.clone()),
            _ => None,
        }
    }
}

fn parse_probability(value: &TaggedValue) -> Option<String> {
    match value {
        TaggedValue { tag, value } if tag == "Percent" => {
            Some(format!("Percent({})", value.as_u64()?))
        }
        TaggedValue { tag, value } if tag == "Promile" => {
            Some(format!("Promile({})", value.as_u64()?))
        }
        _ => None,
    }
}

macro_rules! parse_error {
    () => {
        <Box<dyn Error>>::from(format!("Couldn't parse mutation probability. Line: {}, column: {}", line!(), column!()))
    };
}

pub(self) use parse_error;