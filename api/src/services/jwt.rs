use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::args::AppArgs;

#[derive(Serialize, Deserialize)]
pub struct TokenPayload<C> {
    pub iat: u64,
    pub exp: u64,
    #[serde(flatten)]
    pub claims: C,
}

#[derive(Clone)]
pub struct JwtService {
    key: Hmac<Sha256>,
}

impl JwtService {
    fn timestamp() -> Result<u64> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
    }

    pub fn new(args: &AppArgs) -> Result<Self> {
        let secret: Vec<u8> = match &args.jwt_secret {
            Some(s) => s.as_bytes().to_vec(),
            None => {
                let mut key = vec![0u8; 32];
                rand::rngs::OsRng.try_fill_bytes(&mut key)?;
                key
            }
        };

        Ok(Self {
            key: Hmac::new_from_slice(&secret)?,
        })
    }

    pub fn encode<C: Serialize>(&self, claims: C, expires_in_s: u64) -> Result<String> {
        let now = Self::timestamp()?;
        let token = TokenPayload {
            iat: now,
            exp: now + expires_in_s,
            claims,
        };
        Ok(token.sign_with_key(&self.key)?)
    }

    pub fn decode<C>(&self, value: &str) -> Result<TokenPayload<C>>
    where
        C: for<'de> Deserialize<'de>,
    {
        let token: TokenPayload<C> = value.verify_with_key(&self.key)?;
        if token.exp < Self::timestamp()? {
            return Err(anyhow!("Expired"));
        }
        Ok(token)
    }
}
