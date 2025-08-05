use fleet_net_common::error::FleetNetError;
use semver::Version as Semver;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    /// The current version of the protocol.
    current: Option<Semver>,
    /// List of compatible versions.
    supported_versions: Vec<Semver>,
}

impl Version {
    /// Creates a new Version instance with the current version and an empty list of compatible versions.
    pub fn new(supported: &[Semver]) -> Self {
        Self {
            current: None,
            supported_versions: Vec::from(supported),
        }
    }

    pub fn current(&self) -> Option<Semver> {
        self.current.clone()
    }

    /// Negotiates the version with the client.
    /// Returns the negotiated version if compatible, or an error message if not.
    pub fn negotiate(&mut self, client_versions: &Vec<Semver>) -> Result<Semver, FleetNetError> {
        // Handle no client versions provided
        if client_versions.is_empty() {
            return Err(FleetNetError::NetworkError(
                "No client versions provided for negotiation".into(),
            ));
        }

        // Handle no supported versions
        if self.supported_versions.is_empty() {
            return Err(FleetNetError::NetworkError(
                "No supported versions available for negotiation".into(),
            ));
        }

        let mut compatible_versions: Vec<&Semver> = Vec::new();
        for client_version in client_versions {
            if self.supported_versions.contains(client_version) {
                compatible_versions.push(client_version);
            }
        }

        // Select the highest compatible version
        if let Some(best_version) = compatible_versions.into_iter().max() {
            self.current = Some(best_version.clone());
            Ok(best_version.clone())
        } else {
            Err(FleetNetError::NetworkError(Cow::Owned(format!(
                "No compatible versions. Client: [{}], Supported: [{}]",
                client_versions
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                self.supported_versions
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_version() -> Version {
        Version::new(&[
            Semver::parse("1.0.0").unwrap(),
            Semver::parse("1.1.0").unwrap(),
            Semver::parse("2.0.0").unwrap(),
        ])
    }

    #[test]
    fn test_version_negotiation() {
        let mut version = create_test_version();
        let client_version = vec![
            Semver::parse("1.1.0").unwrap(),
            Semver::parse("1.0.0").unwrap(),
        ];

        let negotiation_result = version.negotiate(&client_version);
        assert!(negotiation_result.is_ok());
        assert_eq!(negotiation_result.unwrap(), Semver::parse("1.1.0").unwrap());
    }

    #[test]
    fn test_version_negotiation_failure() {
        let mut version = create_test_version();

        let client_version = vec![
            Semver::parse("3.0.0").unwrap(),
            Semver::parse("3.1.0").unwrap(),
        ];
        let negotiation_result = version.negotiate(&client_version);
        assert!(negotiation_result.is_err());
        assert!(matches!(
            negotiation_result.unwrap_err(),
            FleetNetError::NetworkError(_)
        ));
    }

    #[test]
    fn test_version_no_client_versions() {
        let mut version = create_test_version();
        let client_version: Vec<Semver> = vec![];

        let negotiation_result = version.negotiate(&client_version);
        assert!(negotiation_result.is_err());
        assert!(matches!(
            negotiation_result.unwrap_err(),
            FleetNetError::NetworkError(_)
        ));
    }

    #[test]
    fn test_version_no_supported_versions() {
        let mut version = Version::new(&[]);
        let client_version = vec![Semver::parse("1.0.0").unwrap()];
        let negotiation_result = version.negotiate(&client_version);
        assert!(negotiation_result.is_err());
        assert!(matches!(
            negotiation_result.unwrap_err(),
            FleetNetError::NetworkError(_)
        ));
    }

    #[test]
    fn test_version_current() {
        let mut version = create_test_version();
        let client_version = vec![Semver::parse("1.0.0").unwrap()];
        let negotiation_result = version.negotiate(&client_version);
        assert!(negotiation_result.is_ok());
        assert_eq!(version.current(), Some(Semver::parse("1.0.0").unwrap()));
    }
}
