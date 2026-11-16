// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-drivers: mint drivers component model.
//!
//! Real implementation: drivers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Driver {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintDriversError {
    EmptyDriverName,
    DuplicateDriver(String),
    EmptyEventSubject,
}

impl fmt::Display for MintDriversError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDriverName => write!(f, "driver name must not be empty"),
            Self::DuplicateDriver(id) => write!(f, "duplicate driver {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintDriversError {}

pub struct MintDrivers {
    pub name: String,
    drivers: BTreeMap<String, Driver>,
    events: BTreeMap<String, Event>,
}

impl MintDrivers {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            drivers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_driver(&mut self, item: Driver) -> Result<(), MintDriversError> {
        if item.name.trim().is_empty() {
            return Err(MintDriversError::EmptyDriverName);
        }
        let id = item.id.clone();
        if self.drivers.contains_key(&id) {
            return Err(MintDriversError::DuplicateDriver(id.clone()));
        }
        self.drivers.insert(id, item);
        Ok(())
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }

    pub fn get_driver(&self, id: &str) -> Option<&Driver> {
        self.drivers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintDriversError> {
        if event.subject.trim().is_empty() {
            return Err(MintDriversError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_driver(id: &str, name: &str) -> Driver {
        Driver {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintDrivers::new("MintDrivers");
        assert_eq!(m.name, "MintDrivers");
        assert_eq!(m.driver_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_driver() {
        let mut m = MintDrivers::new("MintDrivers");
        m.add_driver(sample_driver("a1", "Alpha")).unwrap();
        assert_eq!(m.driver_count(), 1);
        assert!(m.get_driver("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintDrivers::new("MintDrivers");
        assert!(m.add_driver(sample_driver("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintDrivers::new("MintDrivers");
        m.add_driver(sample_driver("x", "One")).unwrap();
        assert!(m.add_driver(sample_driver("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintDrivers::new("MintDrivers");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
