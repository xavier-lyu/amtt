use anyhow::Result;
use jwt_simple::prelude::*;

/// Represents a JWT signing key using ES256.
pub struct SigningKey {
    key: ES256KeyPair,
}

/// Represents a JWT verifying key using ES256.
pub struct VerifyingKey {
    key: ES256PublicKey,
}

impl SigningKey {
    /// Creates a new signing key from a PEM string and key ID.
    pub fn try_new(pem: &str, key_id: &str) -> Result<Self> {
        let key = ES256KeyPair::from_pem(pem)?.with_key_id(key_id);
        Ok(Self { key })
    }

    /// Signs a JWT with the given issuer and expiration (in seconds).
    pub fn sign(&self, iss: &str, expiration: u64) -> Result<String> {
        let claims = Claims::create(Duration::from_secs(expiration)).with_issuer(iss);
        self.key.sign(claims)
    }
}

impl VerifyingKey {
    /// Creates a new verifying key from a PEM string and key ID.
    pub fn try_new(pem: &str, key_id: &str) -> Result<Self> {
        let key = ES256PublicKey::from_pem(pem)?.with_key_id(key_id);
        Ok(Self { key })
    }

    /// Verifies a JWT token for the given issuer.
    pub fn verify(&self, token: &str, iss: &str) -> Result<bool> {
        self.verify_with_time_tolerance(token, iss, None)
    }

    fn verify_with_time_tolerance(
        &self,
        token: &str,
        iss: &str,
        time_rolerance: Option<Duration>,
    ) -> Result<bool> {
        let opts = VerificationOptions {
            required_key_id: self.key.key_id().clone(),
            allowed_issuers: Some(HashSet::from_strings(&[iss])),
            time_tolerance: time_rolerance,
            ..Default::default()
        };
        Ok(self
            .key
            .verify_token::<NoCustomClaims>(token, Some(opts))
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration as StdDuration;

    const JWT_EXPIRATION: u64 = 60 * 60 * 24 * 7;
    const JWT_ISS: &str = "JWT_ISS";
    const JWT_KEY_ID: &str = "JWT_KEY_ID";

    const SIGNING_KEY: &str = include_str!("../../fixtures/private.pem");
    const VERIFYING_KEY: &str = include_str!("../../fixtures/public.pem");

    #[test]
    fn jwt_sign_verify_should_work() -> Result<()> {
        let sk = SigningKey::try_new(SIGNING_KEY, JWT_KEY_ID)?;
        let vk = VerifyingKey::try_new(VERIFYING_KEY, JWT_KEY_ID)?;
        let token = sk.sign(JWT_ISS, JWT_EXPIRATION)?;
        let is_valid = vk.verify(&token, JWT_ISS)?;
        assert!(is_valid);
        Ok(())
    }

    #[test]
    fn jwt_sign_verify_key_id_not_match() -> Result<()> {
        const VERIFY_JWT_KEY_ID: &str = "VERIFY_JWT_KEY_ID";
        let sk = SigningKey::try_new(SIGNING_KEY, JWT_KEY_ID)?;
        let vk = VerifyingKey::try_new(VERIFYING_KEY, VERIFY_JWT_KEY_ID)?;
        let token = sk.sign(JWT_ISS, JWT_EXPIRATION)?;
        let is_valid = vk.verify(&token, JWT_ISS)?;
        assert!(!is_valid);
        Ok(())
    }

    #[test]
    fn jwt_sign_verify_issuer_not_match() -> Result<()> {
        let sk = SigningKey::try_new(SIGNING_KEY, JWT_KEY_ID)?;
        let vk = VerifyingKey::try_new(VERIFYING_KEY, JWT_KEY_ID)?;
        let token = sk.sign(JWT_ISS, JWT_EXPIRATION)?;
        let is_valid = vk.verify(&token, "WRONG_ISSUER")?;
        assert!(!is_valid);
        Ok(())
    }

    #[test]
    fn jwt_sign_verify_expired_token() -> Result<()> {
        let sk = SigningKey::try_new(SIGNING_KEY, JWT_KEY_ID)?;
        let vk = VerifyingKey::try_new(VERIFYING_KEY, JWT_KEY_ID)?;
        let token = sk.sign(JWT_ISS, 1)?;
        // Sleep longer to ensure token is expired
        sleep(StdDuration::from_secs(5));
        let is_valid =
            vk.verify_with_time_tolerance(&token, JWT_ISS, Some(Duration::from_secs(0)))?;
        assert!(!is_valid);
        Ok(())
    }

    #[test]
    fn jwt_sign_verify_invalid_token() -> Result<()> {
        let vk = VerifyingKey::try_new(VERIFYING_KEY, JWT_KEY_ID)?;
        let is_valid = vk.verify("invalid.token.value", JWT_ISS)?;
        assert!(!is_valid);
        Ok(())
    }
}
