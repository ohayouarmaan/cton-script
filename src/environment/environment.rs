use std::collections::HashMap;

use crate::ast::ast::Literal;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Literal>,
    pub parent: Option<Box<Environment>>
}

impl Environment {
    pub fn new(env: Option<Box<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: env
        }
    }

    pub fn get_value(&self, name: String) -> Result<Literal, String> {
        match self.values.get(&name) {
            Some(val) => {
                return Ok(val.clone());
            }
            _ => {
                let par = &self.parent;
                if let Some(par) = par {
                    if let Ok(val) = par.get_value(name.clone()) {
                        return Ok(val);
                    }
                }
                let mut err_string = format!("Error while getting the value of variable '{}'", name);
                return Err(err_string);
            }
        }
    }

    pub fn assign(&mut self, name: String, value: Literal) {
        let par = &mut self.parent;
        if let Some(par) = par {
            if let Ok(_) = par.get_value(name.clone()) {
                par.assign(name.clone(), value.clone());
            }
        }
        self.values.insert(name, value);
    }
}