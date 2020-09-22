// Copyright (c) Microsoft. All rights reserved.

use crate::error::Error;

pub struct IdentityManager {
	locks: std::sync::Mutex<std::collections::BTreeMap<String, std::sync::Arc<std::sync::Mutex<()>>>>,
	key_client: std::sync::Arc<aziot_key_client_async::Client>,
	key_engine: std::sync::Arc<futures_util::lock::Mutex<openssl2::FunctionalEngine>>,
	cert_client: std::sync::Arc<aziot_cert_client_async::Client>
}

impl IdentityManager {
	pub fn new(
		key_client: std::sync::Arc<aziot_key_client_async::Client>,
		key_engine: std::sync::Arc<futures_util::lock::Mutex<openssl2::FunctionalEngine>>,
		cert_client: std::sync::Arc<aziot_cert_client_async::Client>,
	) -> Self {
		IdentityManager {
			locks: Default::default(),
			key_client,
			key_engine,
			cert_client,
		}
	}

	pub async fn get_local_identity(&self, _module_id: &str) -> Result<aziot_identity_common::Identity, Error> {
		Ok(aziot_identity_common::Identity::Local(aziot_identity_common::LocalIdSpec {
			attributes: aziot_identity_common::LocalIdAttributes::Server,
			issuer: "local-id-issuer".to_owned(),
			auth: aziot_identity_common::AuthenticationInfo {
				auth_type: aziot_identity_common::AuthenticationType::X509,
				key_handle: aziot_key_common::KeyHandle("mqtt-server".to_owned()),
				cert_id: Some("mqtt-server".to_owned()),
			}
		}))
	}
}
