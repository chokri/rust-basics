use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub token: String,
}

pub struct SocialGraph {
    users: HashMap<u64, User>,
    followers: HashMap<u64, Vec<u64>>,
}

impl SocialGraph {
    pub fn new() -> Self {
        SocialGraph {
            users: HashMap::new(),
            followers: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user.clone());
        self.followers.entry(user.id).or_insert(vec![]);
    }

    pub fn add_follower(&mut self, user_id: u64, follower_id: u64) {
        if let Some(followers) = self.followers.get_mut(&user_id) {
            if !followers.contains(&follower_id) {
                followers.push(follower_id);
            }
        } else {
            self.followers.insert(user_id, vec![follower_id]);
        }
    }

    pub fn display_graph(&self) {
        for (user_id, followers) in &self.followers {
            if let Some(user) = self.users.get(user_id) {
                println!("User: {} ({})", user.username, user.email);
                println!("Followers:");
                for follower_id in followers {
                    if let Some(follower) = self.users.get(follower_id) {
                        println!("- {} ({})", follower.username, follower.email);
                    }
                }
                println!();
            }
        }
    }
}

