use std::error::Error;
use std::collections::HashMap;


pub struct Comm {
    pub host: String,
    pub namespace: String,
    pub password: String
}

impl Comm {
    pub fn default() -> Comm {
        Comm {
            host: "0.0.0.0".to_string(),
            namespace: "".to_string(),
            password: "".to_string()
        }
    }

    pub fn new() -> Comm {
        return Comm::default();
    }

    pub fn get(&self, _key:String) -> String {
        let mut query_params = HashMap::new();
        query_params.insert(String::from("action"), String::from("get"));
        query_params.insert(String::from("key"), String::from(_key));
        return self.make_request(query_params).unwrap()
    }

    pub fn set(&self, _key: String, value:String) -> bool {
        let mut query_params = HashMap::new();
        query_params.insert(String::from("action"), String::from("set"));
        query_params.insert(String::from("key"), String::from(_key));
        query_params.insert(String::from("value"), String::from(value));
        return self.make_request(query_params).unwrap()
    }

    fn make_request(&self, qp:HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        let resp = reqwest::get("https://httpbin.org/ip")
        .query(qp)?.text()?;
        return resp;
    }
}