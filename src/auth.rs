#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Auth {
    Basic { shop_id: String, secret_key: String },
    OAuth { token: String },
}

impl Auth {
    pub fn basic(shop_id: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self::Basic {
            shop_id: shop_id.into(),
            secret_key: secret_key.into(),
        }
    }

    pub fn oauth(token: impl Into<String>) -> Self {
        Self::OAuth {
            token: token.into(),
        }
    }

    pub fn is_oauth(&self) -> bool {
        matches!(self, Self::OAuth { .. })
    }
}
