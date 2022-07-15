extern crate strict_yaml_rust;
use std::{fs, error::Error};

use strict_yaml_rust::{StrictYamlLoader, StrictYamlEmitter, StrictYaml};

pub struct NodeFirewalls {
    pub raw: String,
    pub raw_docs: Vec<String>,
    pub docs: Vec<StrictYaml>,
}

impl NodeFirewalls {
    pub fn new(filename: &String) -> Result<NodeFirewalls, Box<dyn Error>> {
        let raw: String = fs::read_to_string(filename)?;  
        let docs = StrictYamlLoader::load_from_str(&raw)?;

        let mut raw_docs = Vec::new();
        for doc in &docs {
            // Dump the YAML object
            let mut raw_doc = String::new();
            {
                let mut emitter = StrictYamlEmitter::new(&mut raw_doc);
                emitter.dump(doc)?;
                raw_docs.push(raw_doc);
            }
        }
        Ok(NodeFirewalls { raw, raw_docs, docs })
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.docs.is_empty() {
            return Err("Empty YAML supplied");
        }
    
        Ok(())        
    }
}
