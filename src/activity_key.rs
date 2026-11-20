use crate::Activity;
#[cfg(any(test, feature = "testactivities"))]
use fnv::FnvBuildHasher;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};
#[cfg(any(test, feature = "testactivities"))]
use uuid::Uuid;

#[derive(Clone, Debug)]
#[cfg(any(test, feature = "testactivities"))]
/// An ActivityKey provides a map String -> Activity, which compiles to a usize in release mode.
/// There are automated tests in place to ensure an Activity is never interacting with an ActivityKey it does not belong to.
/// ActivityKeys are assumed to reflect the model: each Activity in the ActivityKey should appear in the object, and
/// each Activity in an object should appear in its ActivityKey.
pub struct ActivityKey {
    pub name2activity: HashMap<String, Activity, FnvBuildHasher>,
    pub activity2name: Vec<String>,
    pub next_index: usize,
    uuid: Uuid,
}

/// An ActivityKey provides a map String -> Activity, which compiles to a usize in release mode.
/// There are automated tests in place to ensure an Activity is never interacting with an ActivityKey it does not belong to.
/// ActivityKeys are assumed to reflect the model: each Activity in the ActivityKey should appear in the object, and
/// each Activity in an object should appear in its ActivityKey.
#[derive(Clone, Debug, Default)]
#[cfg(not(any(test, feature = "testactivities")))]
pub struct ActivityKey {
    pub name2activity: HashMap<String, Activity>,
    pub activity2name: Vec<String>,
    pub next_index: usize,
}

#[cfg(any(test, feature = "testactivities"))]
impl Default for ActivityKey {
    fn default() -> Self {
        Self {
            name2activity: HashMap::default(),
            activity2name: vec![],
            next_index: 0,
            uuid: Uuid::new_v4(),
        }
    }
}

impl<'a> ActivityKey {
    #[deprecated(note = "Use default() instead.")]
    #[cfg(any(test, feature = "testactivities"))]
    pub fn new() -> Self {
        Self {
            name2activity: HashMap::default(),
            activity2name: vec![],
            next_index: 0,
            uuid: Uuid::new_v4(),
        }
    }

    #[deprecated(note = "Use default() instead.")]
    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn new() -> Self {
        Self {
            name2activity: HashMap::new(),
            activity2name: vec![],
            next_index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.name2activity.len()
    }

    pub fn is_empty(&self) -> bool {
        self.name2activity.is_empty()
    }

    #[deprecated(note = "Use len() instead.")]
    pub fn get_number_of_activities(&self) -> usize {
        self.name2activity.len()
    }

    #[deprecated(note = "Use activities() instead.")]
    pub fn get_activities(&self) -> Vec<&Activity> {
        self.name2activity.values().collect()
    }

    pub fn activities(&self) -> Vec<&Activity> {
        self.name2activity.values().collect()
    }

    #[cfg(any(test, feature = "testactivities"))]
    pub fn process_trace(&mut self, trace: &Vec<String>) -> Vec<Activity> {
        let mut result = vec![];
        for activity in trace {
            match self.name2activity.get(activity) {
                Some(index) => result.push(*index),
                None => {
                    let index = Activity {
                        id: self.next_index,
                        activity_key_uuid: self.uuid,
                    };
                    result.push(index);
                    self.activity2name.push(activity.clone());
                    self.name2activity.insert(activity.clone(), index);
                    self.next_index += 1;
                }
            }
        }
        result
    }

    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn process_trace(&mut self, trace: &Vec<String>) -> Vec<Activity> {
        let mut result = vec![];
        for activity in trace {
            match self.name2activity.get(activity) {
                Some(index) => result.push(*index),
                None => {
                    let index = Activity {
                        id: self.next_index,
                    };
                    result.push(index);
                    self.activity2name.push(activity.clone());
                    self.name2activity.insert(activity.clone(), index);
                    self.next_index += 1;
                }
            }
        }
        result
    }

    #[cfg(any(test, feature = "testactivities"))]
    pub fn process_trace_ref(&mut self, trace: &Vec<&str>) -> Vec<Activity> {
        let mut result = vec![];
        for label in trace {
            match self.name2activity.get(*label) {
                Some(index) => result.push(*index),
                None => {
                    let index = Activity {
                        id: self.next_index,
                        activity_key_uuid: self.uuid,
                    };
                    result.push(index);
                    self.activity2name.push(label.to_string());
                    self.name2activity.insert(label.to_string(), index);
                    self.next_index += 1;
                }
            }
        }
        result
    }

    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn process_trace_ref(&mut self, trace: &Vec<&str>) -> Vec<Activity> {
        let mut result = vec![];
        for label in trace {
            match self.name2activity.get(*label) {
                Some(index) => result.push(*index),
                None => {
                    let index = Activity {
                        id: self.next_index,
                    };
                    result.push(index);
                    self.activity2name.push(label.to_string());
                    self.name2activity.insert(label.to_string(), index);
                    self.next_index += 1;
                }
            }
        }
        result
    }

    #[deprecated(note = "Use activity_to_label() instead.")]
    #[cfg(any(test, feature = "testactivities"))]
    pub fn get_activity_label(&self, activity: &Activity) -> &str {
        assert!(
            self.uuid == activity.activity_key_uuid,
            "cannot get activity label of activity of different activity key"
        );
        &self.activity2name[activity.id]
    }

    #[cfg(any(test, feature = "testactivities"))]
    pub fn activity_to_label(&self, activity: &Activity) -> &str {
        assert!(
            self.uuid == activity.activity_key_uuid,
            "cannot get activity label of activity of different activity key"
        );
        &self.activity2name[activity.id]
    }

    #[deprecated(note = "Use activity_to_label() instead.")]
    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn get_activity_label(&self, activity: &Activity) -> &str {
        &self.activity2name[activity.id]
    }

    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn activity_to_label(&self, activity: &Activity) -> &str {
        &self.activity2name[activity.id]
    }

    #[cfg(any(test, feature = "testactivities"))]
    pub fn process_activity(&mut self, activity: &str) -> Activity {
        match self.name2activity.get(activity) {
            Some(index) => *index,
            None => {
                let result = Activity {
                    id: self.next_index,
                    activity_key_uuid: self.uuid,
                };
                self.activity2name.push(activity.to_string());
                self.name2activity.insert(activity.to_string(), result);
                self.next_index += 1;
                result
            }
        }
    }

    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn process_activity(&mut self, activity: &str) -> Activity {
        match self.name2activity.get(activity) {
            Some(index) => *index,
            None => {
                let result = Activity {
                    id: self.next_index,
                };
                self.activity2name.push(activity.to_string());
                self.name2activity.insert(activity.to_string(), result);
                self.next_index += 1;
                result
            }
        }
    }

    #[deprecated(note = "Use try_process_activity() instead.")]
    pub fn process_activity_attempt(&self, activity: &str) -> Option<Activity> {
        self.name2activity.get(activity).copied()
    }

    pub fn try_process_activity(&self, activity: &str) -> Option<Activity> {
        self.name2activity.get(activity).copied()
    }

    #[deprecated(note = "Use id_to_activity() instead.")]
    #[cfg(any(test, feature = "testactivities"))]
    pub fn get_activity_by_id(&self, activity_id: usize) -> Activity {
        Activity {
            id: activity_id,
            activity_key_uuid: self.uuid,
        }
    }

    #[cfg(any(test, feature = "testactivities"))]
    pub fn id_to_activity(&self, activity_id: usize) -> Activity {
        Activity {
            id: activity_id,
            activity_key_uuid: self.uuid,
        }
    }

    #[deprecated(note = "Use id_to_activity() instead.")]
    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn get_activity_by_id(&self, activity_id: usize) -> Activity {
        Activity { id: activity_id }
    }

    #[cfg(not(any(test, feature = "testactivities")))]
    pub fn id_to_activity(&self, activity_id: usize) -> Activity {
        Activity { id: activity_id }
    }

    #[deprecated(note = "Use activity_to_id() instead.")]
    pub fn get_id_from_activity(&self, activity: impl Borrow<Activity>) -> usize {
        activity.borrow().id
    }

    pub fn activity_to_id(&self, activity: impl Borrow<Activity>) -> usize {
        activity.borrow().id
    }

    pub fn deprocess_trace(&self, trace: &[Activity]) -> Vec<&str> {
        trace
            .iter()
            .map(|activity| self.activity_to_label(activity))
            .collect()
    }

    pub fn deprocess_set(&'a self, set: &HashSet<Vec<Activity>>) -> HashSet<Vec<&'a str>> {
        set.iter()
            .map(|trace| self.deprocess_trace(trace))
            .collect()
    }

    pub fn deprocess_activity(&self, activity: &Activity) -> &str {
        self.activity_to_label(activity)
    }

    #[cfg(any(test, feature = "testactivities"))]
    pub fn assert_activity_is_of_key(&self, activity: &Activity) {
        assert!(
            self.uuid == activity.activity_key_uuid,
            "activity does not belong to a different activity key"
        );
    }
}

impl Display for ActivityKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, label) in self.activity2name.iter().enumerate() {
            write!(f, "ac{}: {}, ", i, label)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::ActivityKey;

    #[test]
    #[should_panic(expected = "cannot get activity label of activity of different activity key")]
    fn activity_key_process() {
        let key1 = ActivityKey::default();
        let mut key2 = ActivityKey::default();
        let a2 = key2.process_activity("a");
        key1.deprocess_activity(&a2);
    }

    #[test]
    #[should_panic(expected = "cannot compare activities of different activity keys")]
    fn activity_key_equal() {
        let mut key1 = ActivityKey::default();
        let mut key2 = ActivityKey::default();
        let a1 = key1.process_activity("a");
        let a2 = key2.process_activity("a");
        let _ = a1 == a2;
    }

    #[test]
    fn activity_key() {
        let mut activity_key = ActivityKey::default();
        let a = activity_key.process_activity("a");
        let b = activity_key.process_activity("b");

        assert!(a < b);
        assert!(a < 1);
        let _ = a.eq(&0);
        let _ = format!("{:?}", a);
        assert!(a <= b);
        assert!(a.cmp(&b).is_lt());

        let trace = activity_key.process_trace_ref(&vec!["a", "b", "c"]);
        let mut set = HashSet::new();
        set.insert(trace);
        activity_key.deprocess_set(&set);
        activity_key.to_string();
    }

    #[test]
    #[should_panic]
    fn activity_key_ord() {
        let mut activity_key1 = ActivityKey::default();
        let mut activity_key2 = ActivityKey::default();
        let a1 = activity_key1.process_activity("a");
        let a2 = activity_key2.process_activity("a");
        let _ = a1.cmp(&a2);
    }

    #[test]
    #[should_panic]
    fn activity_key_partial_ord() {
        let mut activity_key1 = ActivityKey::default();
        let mut activity_key2 = ActivityKey::default();
        let a1 = activity_key1.process_activity("a");
        let a2 = activity_key2.process_activity("a");
        let _ = a1.partial_cmp(&a2);
    }
}
