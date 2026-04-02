/*
Implement a time-based key-value data structure that supports:

Storing multiple values for the same key at specified time stamps
Retrieving the key's value at a specified timestamp
Implement the TimeMap class:

TimeMap() Initializes the object.
void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
String get(String key, int timestamp) Returns the most recent value of key if set was previously called on it and the most recent timestamp for that key prev_timestamp is less than or equal to the given timestamp (prev_timestamp <= timestamp). If there are no values, it returns "".
Note: For all calls to set, the timestamps are in strictly increasing order.

Example 1:

Input:
["TimeMap", "set", ["alice", "happy", 1], "get", ["alice", 1], "get", ["alice", 2], "set", ["alice", "sad", 3], "get", ["alice", 3]]

Output:
[null, null, "happy", "happy", null, "sad"]

Explanation:
TimeMap timeMap = new TimeMap();
timeMap.set("alice", "happy", 1);  // store the key "alice" and value "happy" along with timestamp = 1.
timeMap.get("alice", 1);           // return "happy"
timeMap.get("alice", 2);           // return "happy", there is no value stored for timestamp 2, thus we return the value at timestamp 1.
timeMap.set("alice", "sad", 3);    // store the key "alice" and value "sad" along with timestamp = 3.
timeMap.get("alice", 3);           // return "sad"
*/

use std::collections::HashMap;

struct TimeMap {
    // key -> timestamps
    keys_to_timestamps: HashMap<String, Vec<i32>>,
    // (key, timestamp) -> value
    key_timestamps_to_values: HashMap<(String, i32), String>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            keys_to_timestamps: HashMap::default(),
            key_timestamps_to_values: HashMap::default(),
        }
    }

    /// timestamp guaranteed to be monotonically increasing
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        // TODO: handle duplicates/erroneous inputs

        self.keys_to_timestamps
            .entry(key.clone())
            .and_modify(|e| e.push(timestamp))
            .or_insert(vec![timestamp]);
        self.key_timestamps_to_values
            .insert((key, timestamp), value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(timestamps) = self.keys_to_timestamps.get(&key) {
            return match timestamps.binary_search(&timestamp) {
                Ok(existing_timestamp_idx) => self
                    .key_timestamps_to_values
                    .get(&(key, timestamps[existing_timestamp_idx]))
                    .unwrap()
                    .clone(),
                Err(next_idx) => {
                    let first_timestamp = *timestamps.first().unwrap();
                    if timestamp < first_timestamp {
                        return "".to_owned();
                    }

                    self.key_timestamps_to_values
                        .get(&(key, timestamps[next_idx - 1]))
                        .unwrap()
                        .clone()
                }
            };
        } else {
            return "".to_owned();
        }
    }
}
