use soroban_sdk::contracterror;

/// Account-related errors for the Cougr framework.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AccountError {
    Unauthorized = 20,
    SessionExpired = 21,
    InvalidSignature = 22,
    CapabilityNotSupported = 23,
    SessionLimitReached = 24,
    InvalidScope = 25,
}
