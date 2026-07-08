#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicUiTrustLevel {
    FirstParty,
    GeneratedFixture,
    Sandbox,
    Untrusted,
}

impl DynamicUiTrustLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::GeneratedFixture => "generated_fixture",
            Self::Sandbox => "sandbox",
            Self::Untrusted => "untrusted",
        }
    }

    pub const fn is_trusted(self) -> bool {
        matches!(
            self,
            Self::FirstParty | Self::GeneratedFixture | Self::Sandbox
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynamicUiTrustMetadata {
    pub level: DynamicUiTrustLevel,
    pub owner_pr: &'static str,
}

impl DynamicUiTrustMetadata {
    pub const fn first_party(owner_pr: &'static str) -> Self {
        Self {
            level: DynamicUiTrustLevel::FirstParty,
            owner_pr,
        }
    }

    pub const fn generated_fixture(owner_pr: &'static str) -> Self {
        Self {
            level: DynamicUiTrustLevel::GeneratedFixture,
            owner_pr,
        }
    }

    pub const fn sandbox(owner_pr: &'static str) -> Self {
        Self {
            level: DynamicUiTrustLevel::Sandbox,
            owner_pr,
        }
    }

    pub const fn untrusted(owner_pr: &'static str) -> Self {
        Self {
            level: DynamicUiTrustLevel::Untrusted,
            owner_pr,
        }
    }
}
