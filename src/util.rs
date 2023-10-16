use crate::{error::ContractError, state::MAX_PATH_LEN};

pub fn split_path_str(path: &String) -> (String, Option<String>) {
    let mut cannonical_path = path.strip_prefix("/").unwrap_or(path);
    cannonical_path = cannonical_path.strip_suffix("/").unwrap_or(cannonical_path);

    let mut parts: Vec<&str> = cannonical_path.rsplitn(1, '/').collect();

    let res_name = parts.pop().map(String::from);
    let parent_path = parts.into_iter().rev().collect::<Vec<&str>>().join("/");

    if !parent_path.starts_with('/') {
        (format!("/{}", parent_path), res_name)
    } else {
        (parent_path, res_name)
    }
}

pub fn validate_path_string(path: &String) -> Result<(), ContractError> {
    if path.is_empty() || !path.starts_with("/") {
        return Err(ContractError::ValidationError {
            reason: Some(format!("path must begin with '/' but got '{}'", path)),
        });
    }
    if path.len() > MAX_PATH_LEN {
        return Err(ContractError::ValidationError {
            reason: Some(format!(
                "path '{}' too long. max length is {}",
                path, MAX_PATH_LEN
            )),
        });
    }
    Ok(())
}
