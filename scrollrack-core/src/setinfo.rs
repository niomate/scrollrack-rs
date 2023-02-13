use scryfall::set::SetType;

#[derive(Debug, Clone)]
pub struct SetInfo {
    set_name: String,
    virtual_set: bool,
    set_type: Option<SetType>,
}

impl PartialEq for SetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.set_name == other.set_name
    }
}

impl Eq for SetInfo {}

impl std::hash::Hash for SetInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.set_name.hash(state)
    }
}

impl SetInfo {
    pub fn new<P>(set_name: P) -> Self
    where
        P: ToString,
    {
        SetInfo {
            set_name: set_name.to_string(),
            virtual_set: false,
            set_type: None,
        }
    }

    pub fn with_set_type<P: ToString>(set_name: P, set_type: SetType) -> Self {
        SetInfo {
            set_name: set_name.to_string(),
            virtual_set: false,
            set_type: Some(set_type),
        }
    }

    pub fn create_virtual_set<P>(set_name: P) -> Self
    where
        P: ToString,
    {
        SetInfo {
            set_name: set_name.to_string(),
            virtual_set: true,
            set_type: None,
        }
    }

    /// Get a reference to the set info's set name.
    #[must_use]
    pub fn set_name(&self) -> &str {
        self.set_name.as_ref()
    }

    #[must_use]
    pub fn set_type(&self) -> Option<SetType> {
        self.set_type
    }

    #[must_use]
    pub fn virtual_set(&self) -> bool {
        self.virtual_set
    }
}

#[cfg(test)]
mod tests {
    use super::SetInfo;

    #[test]
    fn test_set_info_constructor() {
        let c = SetInfo::new("Kaladesh");
        assert_eq!(c.set_name(), "Kaladesh");
    }
}
