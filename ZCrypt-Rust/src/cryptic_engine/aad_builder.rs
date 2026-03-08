use super::cryptic_record::CrypticRecord;

use bincode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct AAD {
    pub schema_version: u16,
    pub user_id: String,
    pub template_id: String,
    pub template_type: String,
    pub template_ver: u16,
}

impl AAD {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("AAD serialization faliure detected !")
    }
}

impl From<CrypticRecord> for AAD {
    fn from(record: CrypticRecord) -> Self {
        Self {
            schema_version: record.schema_version,
            user_id: record.user_id,
            template_id: record.template_id,
            template_type: record.template_type,
            template_ver: record.template_ver,
        }
    }
}

impl AAD {
    pub fn from_record(record: &CrypticRecord) -> Self {
        Self {
            schema_version: record.schema_version,
            user_id: record.user_id.clone(),
            template_id: record.template_id.clone(),
            template_type: record.template_type.clone(),
            template_ver: record.template_ver,
        }
    }
}

pub struct AADBuilder {
    schema_version: u16,
    user_id: String,
    template_id: String,
    template_type: String,
    template_ver: u16,
}

impl AADBuilder {
    pub fn new() -> Self {
        Self {
            schema_version: 1,
            user_id: String::new(),
            template_id: String::new(),
            template_type: String::new(),
            template_ver: 1,
        }
    }

    pub fn schema_version(mut self, v: u16) -> Self {
        self.schema_version = v;
        self
    }

    pub fn user_id(mut self, v: impl Into<String>) -> Self {
        self.user_id = v.into();
        self
    }

    pub fn template_id(mut self, v: impl Into<String>) -> Self {
        self.template_id = v.into();
        self
    }

    pub fn template_type(mut self, v: impl Into<String>) -> Self {
        self.template_type = v.into();
        self
    }

    pub fn template_ver(mut self, v: u16) -> Self {
        self.template_ver = v;
        self
    }

    pub fn build(self) -> AAD {
        AAD {
            schema_version: self.schema_version,
            user_id: self.user_id,
            template_id: self.template_id,
            template_type: self.template_type,
            template_ver: self.template_ver,
        }
    }
}
