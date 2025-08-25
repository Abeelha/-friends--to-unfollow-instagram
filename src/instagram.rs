use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct StringListData {
    pub href: String,
    pub value: String,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FollowerEntry {
    pub title: String,
    pub media_list_data: Vec<serde_json::Value>,
    pub string_list_data: Vec<StringListData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FollowingData {
    pub relationships_following: Vec<FollowerEntry>,
}

pub fn analyze_followers(followers_path: &str, following_path: &str) -> Result<Vec<String>> {
    let followers_data = fs::read_to_string(followers_path)
        .with_context(|| format!("Failed to read followers file: {}", followers_path))?;
    
    let following_data = fs::read_to_string(following_path)
        .with_context(|| format!("Failed to read following file: {}", following_path))?;

    let followers: Vec<FollowerEntry> = serde_json::from_str(&followers_data)
        .with_context(|| "Failed to parse followers JSON")?;
    
    let following: FollowingData = serde_json::from_str(&following_data)
        .with_context(|| "Failed to parse following JSON")?;

    let follower_usernames: HashSet<String> = followers
        .iter()
        .flat_map(|entry| &entry.string_list_data)
        .map(|data| data.value.clone())
        .collect();

    let following_usernames: HashSet<String> = following
        .relationships_following
        .iter()
        .flat_map(|entry| &entry.string_list_data)
        .map(|data| data.value.clone())
        .collect();

    // Print some stats for debugging
    println!("📊 Analysis Stats:");
    println!("   • Following: {} accounts", following_usernames.len());
    println!("   • Followers: {} accounts", follower_usernames.len());
    println!();

    let mut non_mutual_follows: Vec<String> = following_usernames
        .difference(&follower_usernames)
        .cloned()
        .collect();

    non_mutual_follows.sort();

    Ok(non_mutual_follows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_analyze_followers() {
        let followers_json = r#"[
            {
                "title": "",
                "media_list_data": [],
                "string_list_data": [
                    {
                        "href": "https://www.instagram.com/mutual_user",
                        "value": "mutual_user",
                        "timestamp": 1234567890
                    }
                ]
            }
        ]"#;

        let following_json = r#"{
            "relationships_following": [
                {
                    "title": "",
                    "media_list_data": [],
                    "string_list_data": [
                        {
                            "href": "https://www.instagram.com/mutual_user",
                            "value": "mutual_user",
                            "timestamp": 1234567890
                        }
                    ]
                },
                {
                    "title": "",
                    "media_list_data": [],
                    "string_list_data": [
                        {
                            "href": "https://www.instagram.com/non_mutual_user",
                            "value": "non_mutual_user",
                            "timestamp": 1234567890
                        }
                    ]
                }
            ]
        }"#;

        let followers_file = NamedTempFile::new().unwrap();
        let following_file = NamedTempFile::new().unwrap();

        fs::write(&followers_file, followers_json).unwrap();
        fs::write(&following_file, following_json).unwrap();

        let result = analyze_followers(
            followers_file.path().to_str().unwrap(),
            following_file.path().to_str().unwrap(),
        ).unwrap();

        assert_eq!(result, vec!["non_mutual_user"]);
    }
}