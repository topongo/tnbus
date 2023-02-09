extern crate reqwest;

mod interface;
mod structures;
mod errors;

#[cfg(test)]
mod tests {
    use crate::interface::Api;

    #[test]
    fn test_new() {
        #[allow(unused_variables)]
        let interface = Api::new("false_secret".to_string());
    }
}
