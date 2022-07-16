extern crate strict_yaml_rust;
use std::{fs, error::Error, str::FromStr};

use strict_yaml_rust::{StrictYamlLoader, StrictYamlEmitter, StrictYaml};

pub struct NodeEndpoint {
    pub name: String,
    // pub labels: Vec<String>,
    // pub interfaces: Vec<String>,
}

pub struct IngressNodeFirewall {
    pub name: String,
}

pub enum Node {
    Endpoint(NodeEndpoint),
    IngressFirewall(IngressNodeFirewall),
}

fn get_node(doc: &StrictYaml, kind: &str) -> Option<Node> {
    let name = String::from_str(kind).unwrap(); 
    match kind {
        "NodeEndpoint" => Some(Node::Endpoint(NodeEndpoint{name})),
        "IngressNodeFirewall" => Some(Node::IngressFirewall(IngressNodeFirewall{name})),
        _ => None
    }
}


pub struct NodeFirewallDocs {
    pub raw: String,
    pub raw_docs: Vec<String>,
    pub docs: Vec<StrictYaml>,
}

impl NodeFirewallDocs {
    pub fn new(filename: &String) -> Result<NodeFirewallDocs, Box<dyn Error>> {
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
        Ok(NodeFirewallDocs { raw, raw_docs, docs })
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.docs.is_empty() {
            return Err("Empty YAML supplied");
        }

        for doc in &self.docs {
            if doc["kind"].is_badvalue() {
                return Err("Unexpected yaml doc: no kind");
            }
            match doc["kind"].as_str().unwrap() {
                "NodeEndpoint" => {
                    if !self.validate_node_endpoint(doc) {
                        return Err("Invalid NodeEndpoint");
                    }
                },
                "IngressNodeFirewall" => {
                    if !self.validate_ingress_node_firewall(doc) {
                        return Err("Invalid IngressNodeFirewall");
                    }
                },
                other_kind => {
                    eprintln!("Kind {} is not supported", other_kind);
                    return Err("Unexpected kind in yaml");
                },
            }
        }

        Ok(())        
    }

    fn validate_node_endpoint(&self, doc: &StrictYaml) -> bool {
        if doc["metadata"]["name"].is_badvalue() {
            eprintln!("node endpoint does not have name");
            return false
        }

        // let name = doc["name"].as_str().unwrap();
        // let name = String::from(name);
        // let node_endpoint = NodeEndpoint{name};
        // self.node_endpoints.push(node_endpoint);
        return true
    }

    fn validate_ingress_node_firewall(&self, doc: &StrictYaml) -> bool {
        return true
    }

    pub fn get_nodes(&self) -> Result<Vec<Node>, Box<dyn Error>> {
        let mut nodes: Vec<Node> = Vec::new();
    
        for doc in &self.docs {
            if doc["kind"].is_badvalue() {
                continue;
            }
            match get_node(doc, doc["kind"].as_str().unwrap()) {
                Some(node) => nodes.push(node),
                _ => ()
            };
        }
        Ok(nodes)
    }

}
