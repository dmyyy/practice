/*
Implement a simplified version of Twitter which allows users to post tweets, follow/unfollow each other, and view the 10 most recent tweets within their own news feed.

Users and tweets are uniquely identified by their IDs (integers).

Implement the following methods:

Twitter() Initializes the twitter object.
void postTweet(int userId, int tweetId) Publish a new tweet with ID tweetId by the user userId. You may assume that each tweetId is unique.
List<Integer> getNewsFeed(int userId) Fetches at most the 10 most recent tweet IDs in the user's news feed. Each item must be posted by users who the user is following or by the user themself. Tweets IDs should be ordered from most recent to least recent.
void follow(int followerId, int followeeId) The user with ID followerId follows the user with ID followeeId.
void unfollow(int followerId, int followeeId) The user with ID followerId unfollows the user with ID followeeId.
Example 1:

Input:
["Twitter", "postTweet", [1, 10], "postTweet", [2, 20], "getNewsFeed", [1], "getNewsFeed", [2], "follow", [1, 2], "getNewsFeed", [1], "getNewsFeed", [2], "unfollow", [1, 2], "getNewsFeed", [1]]

Output:
[null, null, null, [10], [20], null, [20, 10], [20], null, [10]]

Explanation:
Twitter twitter = new Twitter();
twitter.postTweet(1, 10); // User 1 posts a new tweet with id = 10.
twitter.postTweet(2, 20); // User 2 posts a new tweet with id = 20.
twitter.getNewsFeed(1);   // User 1's news feed should only contain their own tweets -> [10].
twitter.getNewsFeed(2);   // User 2's news feed should only contain their own tweets -> [20].
twitter.follow(1, 2);     // User 1 follows user 2.
twitter.getNewsFeed(1);   // User 1's news feed should contain both tweets from user 1 and user 2 -> [20, 10].
twitter.getNewsFeed(2);   // User 2's news feed should still only contain their own tweets -> [20].
twitter.unfollow(1, 2);   // User 1 follows user 2.
twitter.getNewsFeed(1);   // User 1's news feed should only contain their own tweets -> [10].
Constraints:

1 <= userId, followerId, followeeId <= 100
0 <= tweetId <= 1000
*/

use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Default, Debug)]
pub struct Twitter {
    time: usize,
    tweets: HashMap<i32, Vec<(usize, i32)>>,
    following: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        if let Some(tweets) = self.tweets.get_mut(&user_id) {
            tweets.push((self.time, tweet_id));
        } else {
            // user has never tweeted before!
            self.tweets.insert(user_id, vec![(self.time, tweet_id)]);
        }
    }

    // FIXME: news feed logic is kinda sus - solution is almost there but not quite

    /// Returns at most 10 of the most recent tweets in the users news feed.
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        // find all of the people the user is following
        let Some(following) = self.following.get(&user_id) else {
            return vec![];
        };

        let mut heap = BinaryHeap::new();
        for f in following {
            if let Some(tweets) = self.tweets.get(f) {
                tweets.iter().for_each(|t| heap.push(*t));
            }
        }

        let mut res = vec![];
        while !heap.is_empty() && res.len() < 10 {
            res.push(heap.pop().unwrap().1);
        }
        res
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followers) = self.following.get_mut(&follower_id) {
            followers.insert(followee_id);
        } else {
            // it's followers first time following someone!
            self.following
                .insert(follower_id, HashSet::from([follower_id, followee_id]));
        }
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followers) = self.following.get_mut(&follower_id) {
            followers.remove(&followee_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut twitter = Twitter::default();
        twitter.post_tweet(1, 10);
        twitter.post_tweet(2, 20);
        twitter.post_tweet(1, 10);
        twitter.post_tweet(2, 20);
        println!("{:?} user 1 news feed", twitter.get_news_feed(1));
        println!("{:?} user 2 news feed", twitter.get_news_feed(2));
        twitter.follow(1, 2);
        println!("{:?} user 1 news feed", twitter.get_news_feed(1));
        println!("{:?} user 2 news feed", twitter.get_news_feed(2));
        twitter.unfollow(1, 2);
        println!("{:?} user 1 news feed", twitter.get_news_feed(1));

        println!("{twitter:?}");
    }
}
