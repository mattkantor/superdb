
pub struct Comm {
    pub host: String,
    pub namespace: String,
    pub password: String
}

impl Comm {
    pub fn default() -> Comm {
        Comm {
            host: String::from("0.0.0.0"),
            namespace: String::from(""),
            password: String::from(""),
        }
    }

    pub fn new() -> Comm {
        Comm {
            host: String::from("0.0.0.0"),
            namespace: String::from(""),
            password: String::from(""),
        }
    }

    pub fn get(&self, _key:String) -> String {
        let query_params = vec![
            ("action", "get"),
            ("key", &_key)];  // input encoding
        let result = self.make_request(query_params);
        return result;
    }

    pub fn set(&self, _key: String, value:String) -> String {
        let query_params = vec![("action","set"),("key",&_key),("value",&value)];
        let result = self.make_request(query_params);
        return result;
    }

    fn make_request(&self, qp:Vec<(&str, &str)>) -> String {
        let req_client = reqwest::blocking::Client::new();
        let response = req_client.get("https://httpbin.org/ip").query(&qp).send();
        let result = match response{
            Ok(data) => data.text(),
            Err(e) => panic!("Problem getting data: {:?}", e),
        };
        return result.unwrap()

    }

}