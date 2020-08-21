// Copyright (c) Microsoft. All rights reserved.

pub(super) fn handle(
	req: hyper::Request<hyper::Body>,
	inner: std::sync::Arc<futures_util::lock::Mutex<aziot_keyd::Server>>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<hyper::Response<hyper::Body>, hyper::Request<hyper::Body>>> + Send>> {
	Box::pin(async move {
		if req.uri().path() != "/decrypt" {
			return Err(req);
		}

		let (http::request::Parts { method, headers, .. }, body) = req.into_parts();
		let content_type = headers.get(hyper::header::CONTENT_TYPE).and_then(|value| value.to_str().ok());

		if method != hyper::Method::POST {
			return Ok(super::err_response(
				hyper::StatusCode::METHOD_NOT_ALLOWED,
				Some((hyper::header::ALLOW, "POST")),
				"method not allowed".into(),
			));
		}

		if content_type.as_deref() != Some("application/json") {
			return Ok(super::err_response(
				hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
				None,
				"request body must be application/json".into(),
			));
		}

		let body = match hyper::body::to_bytes(body).await {
			Ok(body) => body,
			Err(err) => return Ok(super::err_response(
				hyper::StatusCode::BAD_REQUEST,
				None,
				super::error_to_message(&err).into(),
			)),
		};
		let body: aziot_key_common_http::decrypt::Request = match serde_json::from_slice(&body) {
			Ok(body) => body,
			Err(err) => return Ok(super::err_response(
				hyper::StatusCode::UNPROCESSABLE_ENTITY,
				None,
				super::error_to_message(&err).into(),
			)),
		};
		let mechanism = match body.parameters {
			aziot_key_common_http::encrypt::Parameters::Aead { iv, aad } => aziot_key_common::EncryptMechanism::Aead { iv: iv.0, aad: aad.0 },

			aziot_key_common_http::encrypt::Parameters::RsaPkcs1 => aziot_key_common::EncryptMechanism::RsaPkcs1,

			aziot_key_common_http::encrypt::Parameters::RsaNoPadding => aziot_key_common::EncryptMechanism::RsaNoPadding,
		};

		let mut inner = inner.lock().await;
		let inner = &mut *inner;

		let plaintext = match inner.decrypt(&body.key_handle, mechanism, &body.ciphertext.0) {
			Ok(plaintext) => plaintext,
			Err(err) => return Ok(super::ToHttpResponse::to_http_response(&err)),
		};

		let res = aziot_key_common_http::decrypt::Response {
			plaintext: http_common::ByteString(plaintext),
		};
		let res = super::json_response(hyper::StatusCode::OK, &res);
		Ok(res)
	})
}
