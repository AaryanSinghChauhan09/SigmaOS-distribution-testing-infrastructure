// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-distribution-testing-infrastructure: distribution testing infrastructure component model.
//!
//! Real implementation: infrastructures, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Infrastructure {
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
pub enum DistributionTestingInfrastructureError {
    EmptyInfrastructureName,
    DuplicateInfrastructure(String),
    EmptyEventSubject,
}

impl fmt::Display for DistributionTestingInfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInfrastructureName => write!(f, "infrastructure name must not be empty"),
            Self::DuplicateInfrastructure(id) => write!(f, "duplicate infrastructure {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DistributionTestingInfrastructureError {}

pub struct DistributionTestingInfrastructure {
    pub name: String,
    infrastructures: BTreeMap<String, Infrastructure>,
    events: BTreeMap<String, Event>,
}

impl DistributionTestingInfrastructure {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            infrastructures: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_infrastructure(&mut self, item: Infrastructure) -> Result<(), DistributionTestingInfrastructureError> {
        if item.name.trim().is_empty() {
            return Err(DistributionTestingInfrastructureError::EmptyInfrastructureName);
        }
        let id = item.id.clone();
        if self.infrastructures.contains_key(&id) {
            return Err(DistributionTestingInfrastructureError::DuplicateInfrastructure(id.clone()));
        }
        self.infrastructures.insert(id, item);
        Ok(())
    }

    pub fn infrastructure_count(&self) -> usize {
        self.infrastructures.len()
    }

    pub fn get_infrastructure(&self, id: &str) -> Option<&Infrastructure> {
        self.infrastructures.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DistributionTestingInfrastructureError> {
        if event.subject.trim().is_empty() {
            return Err(DistributionTestingInfrastructureError::EmptyEventSubject);
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

    fn sample_infrastructure(id: &str, name: &str) -> Infrastructure {
        Infrastructure {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DistributionTestingInfrastructure::new("DistributionTestingInfrastructure");
        assert_eq!(m.name, "DistributionTestingInfrastructure");
        assert_eq!(m.infrastructure_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_infrastructure() {
        let mut m = DistributionTestingInfrastructure::new("DistributionTestingInfrastructure");
        m.add_infrastructure(sample_infrastructure("a1", "Alpha")).unwrap();
        assert_eq!(m.infrastructure_count(), 1);
        assert!(m.get_infrastructure("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DistributionTestingInfrastructure::new("DistributionTestingInfrastructure");
        assert!(m.add_infrastructure(sample_infrastructure("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DistributionTestingInfrastructure::new("DistributionTestingInfrastructure");
        m.add_infrastructure(sample_infrastructure("x", "One")).unwrap();
        assert!(m.add_infrastructure(sample_infrastructure("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DistributionTestingInfrastructure::new("DistributionTestingInfrastructure");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
