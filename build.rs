use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use serde_yaml::value::TaggedValue;

fn main() {
    let reader = BufReader::new(File::open("config.yaml").unwrap());
    let cfg = serde_yaml::from_reader::<_, serde_yaml::Value>(reader).unwrap();
    let cfg_src = format!(
        r#"
crate::algen::config::Config {{
    population_size: {},
    mutation_probability: {},
    crossover_probability: {},
    children_per_parent: {}
}}
"#,
        cfg.get("population-size").and_then(|x| x.as_u64()).unwrap(),
        as_probability(
            &cfg.get("mutation-probability")
                .and_then(|x| x.as_tagged())
                .unwrap()
        )
        .unwrap(),
        as_probability(
            &cfg.get("crossover-probability")
                .and_then(|x| x.as_tagged())
                .unwrap()
        )
        .unwrap(),
        cfg.get("children-per-parent")
            .and_then(|x| x.as_u64())
            .unwrap()
    );
    let config_dest = Path::new(&env::var_os("OUT_DIR").unwrap()).join("config.generated.rs");
    fs::write(&config_dest, cfg_src).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
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

fn as_probability(value: &TaggedValue) -> Option<String> {
    match value {
        TaggedValue { tag, value } if tag == "Percent" => {
            Some(format!("Percent({})", value.as_u64().unwrap()))
        }
        TaggedValue { tag, value } if tag == "Promile" => {
            Some(format!("Promile({})", value.as_u64().unwrap()))
        }
        _ => None,
    }
}
