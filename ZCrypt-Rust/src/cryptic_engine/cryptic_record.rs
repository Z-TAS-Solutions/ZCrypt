use super::aad_builder::AAD;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrypticRecord {
    pub schema_version: u16,
    pub user_id: String,
    pub template_id: String,
    pub template_type: String,
    pub template_ver: u16,

    pub template_nonce: [u8; 12],
    pub wrapped_dek: Vec<u8>,
    pub wrap_nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
}

pub struct CrypticRecordBuilder {
    inner: CrypticRecord,
}

impl CrypticRecord {
    pub fn new_from_aad(
        aad: AAD,
        template_nonce: [u8; 12],
        wrapped_dek: Vec<u8>,
        wrap_nonce: [u8; 12],
        ciphertext: Vec<u8>,
    ) -> Self {
        Self {
            schema_version: aad.schema_version,
            user_id: aad.user_id,
            template_id: aad.template_id,
            template_type: aad.template_type,
            template_ver: aad.template_ver,

            template_nonce,
            wrapped_dek,
            wrap_nonce,
            ciphertext,
        }
    }
}

impl From<AAD> for CrypticRecord {
    fn from(aad: AAD) -> Self {
        Self {
            schema_version: aad.schema_version,
            user_id: aad.user_id,
            template_id: aad.template_id,
            template_type: aad.template_type,
            template_ver: aad.template_ver,

            template_nonce: [0u8; 12],
            wrapped_dek: Vec::new(),
            wrap_nonce: [0u8; 12],
            ciphertext: Vec::new(),
        }
    }
}

impl CrypticRecordBuilder {
    pub fn new() -> Self {
        Self {
            inner: CrypticRecord {
                schema_version: 1,
                user_id: String::new(),
                template_id: String::new(),
                template_type: String::new(),
                template_ver: 1,

                template_nonce: [0u8; 12],
                wrapped_dek: Vec::new(),
                wrap_nonce: [0u8; 12],
                ciphertext: Vec::new(),
            },
        }
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.inner.user_id = user_id.into();
        self
    }

    pub fn template_id(mut self, template_id: impl Into<String>) -> Self {
        self.inner.template_id = template_id.into();
        self
    }

    pub fn template_type(mut self, template_type: impl Into<String>) -> Self {
        self.inner.template_type = template_type.into();
        self
    }

    pub fn template_ver(mut self, template_ver: u16) -> Self {
        self.inner.template_ver = template_ver;
        self
    }

    pub fn template_nonce(mut self, template_nonce: [u8; 12]) -> Self {
        self.inner.template_nonce = template_nonce;
        self
    }

    pub fn wrapped_dek(mut self, dek: Vec<u8>) -> Self {
        self.inner.wrapped_dek = dek;
        self
    }

    pub fn wrap_nonce(mut self, wrap_nonce: [u8; 12]) -> Self {
        self.inner.wrap_nonce = wrap_nonce;
        self
    }

    pub fn ciphertext(mut self, data: Vec<u8>) -> Self {
        self.inner.ciphertext = data;
        self
    }

    pub fn build(self) -> CrypticRecord {
        self.inner
    }
}
