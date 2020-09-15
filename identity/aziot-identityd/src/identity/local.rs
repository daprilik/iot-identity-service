// Copyright (c) Microsoft. All rights reserved.

pub struct IdentityManager {
	locks: std::sync::Mutex<std::collections::BTreeMap<String, std::sync::Arc<std::sync::Mutex<()>>>>,
	key_client: std::sync::Arc<aziot_key_client_async::Client>,
	key_engine: std::sync::Arc<futures_util::lock::Mutex<openssl2::FunctionalEngine>>,
	cert_client: std::sync::Arc<aziot_cert_client_async::Client>,
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
}
