use ngtq::{NGCategoryTask, NGId, NGIdTask, NGTQError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    has_value: bool,
    id_token: String
}

impl NGId for Id {
    fn set_with_validation(input: String) -> Result<Self, NGTQError> {
        if input.len() != 10 {
            return Err(
                NGTQError::generate_error(
                    ngtq::NGTQErrorType::IdQueue(String::from("Task id have to be 10 chars length")), 
                    String::from("Failed to validate id for task")
                )
            )
        }

        Ok(Id { has_value: true, id_token: input })
    }

    fn get(&self) -> Option<String> {
        if self.has_value  {
            return Some(self.id_token.to_string())
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdTask {
    pub id: Option<Id>,
    pub payload: String,
}

impl NGIdTask for IdTask {
    fn get_payload(&self) -> String {
        self.payload.to_string()
    }
    
    fn get_id<Id>(&self) -> Option<String> {
        match &self.id {
            Some(id) => Some(id.id_token.to_string()),
            None => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTask {
    pub category: String,
    pub payload: String,
}

impl NGCategoryTask for CategoryTask {  
    fn get_category(&self) -> &str {
        &self.category
    }
    
    fn get_payload(&self) -> String {
        self.payload.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_with_validation_too_short_invalid() {
        let id = Id::set_with_validation(String::from("123"));
        match id {
            Ok(_) => {
                println!("Test failed - expected id to be invalid");
                assert!(false)
            },
            Err(_) => assert!(true)
        } 
    }


    #[test]
    fn set_with_validation_too_long_invalid() {
        let id = Id::set_with_validation(String::from("11223344556677889900"));
        match id {
            Ok(_) => {
                println!("Test failed - expected id to be invalid");
                assert!(false)
            },
            Err(_) => assert!(true)
        } 
    }


    #[test]
    fn set_with_validation_10_char_valid() {
        let id = Id::set_with_validation(String::from("1234567890"));
        match id {
            Ok(_) => assert!(true),
            Err(_) => {
                println!("Test failed - expected id to be valid!");
                assert!(false)
            }
        } 
    }

}