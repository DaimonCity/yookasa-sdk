use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

use rust_decimal::Decimal;
use serde_json::{Value, json};
use yookasa_sdk::model::{
    ConfirmationData, CreatePaymentRequest, CreateWebhookRequest, CurrencyCode, Locale,
    MonetaryAmount, NotificationEventType, PaymentId, PaymentListQuery, PaymentStatus, Recipient,
    TimeFilter,
};
use yookasa_sdk::{ApiErrorBody, Auth, ErrorCode, YookassaClient, YookassaError};

#[derive(Debug)]
struct CapturedRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct MockResponse {
    status_line: &'static str,
    body: String,
    content_type: &'static str,
}

fn spawn_mock_server(response: MockResponse) -> (String, mpsc::Receiver<CapturedRequest>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
    let address = listener.local_addr().expect("local addr");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept request");
        let request = read_request(&mut stream);

        let response_text = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            response.status_line,
            response.content_type,
            response.body.len(),
            response.body
        );

        stream
            .write_all(response_text.as_bytes())
            .expect("write response");
        stream.flush().expect("flush response");
        tx.send(request).expect("send captured request");
    });

    (format!("http://{}", address), rx)
}

fn read_request(stream: &mut std::net::TcpStream) -> CapturedRequest {
    let mut buffer = Vec::new();
    let mut header_end = None;

    while header_end.is_none() {
        let mut chunk = [0_u8; 1024];
        let read = stream.read(&mut chunk).expect("read request chunk");
        assert!(read > 0, "unexpected EOF while reading headers");
        buffer.extend_from_slice(&chunk[..read]);
        header_end = find_header_end(&buffer);
    }

    let header_end = header_end.expect("header terminator");
    let header_bytes = &buffer[..header_end];
    let mut body = buffer[header_end + 4..].to_vec();

    let header_text = String::from_utf8(header_bytes.to_vec()).expect("headers are utf8");
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().expect("request line");
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().expect("request method").to_string();
    let path = request_parts.next().expect("request path").to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    let content_length = headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);

    while body.len() < content_length {
        let mut chunk = vec![0_u8; content_length - body.len()];
        let read = stream.read(&mut chunk).expect("read request body");
        assert!(read > 0, "unexpected EOF while reading body");
        body.extend_from_slice(&chunk[..read]);
    }

    CapturedRequest {
        method,
        path,
        headers,
        body,
    }
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

fn payment_json(id: &str, status: &str) -> Value {
    json!({
        "id": id,
        "status": status,
        "amount": {
            "value": "100.00",
            "currency": "RUB"
        },
        "recipient": {
            "account_id": "acc-123",
            "gateway_id": "gw-123"
        },
        "created_at": "2026-04-25T10:00:00Z",
        "test": true,
        "paid": false,
        "refundable": false
    })
}

fn build_client(base_url: String, auth: Auth) -> YookassaClient {
    YookassaClient::builder(auth)
        .base_url(base_url)
        .build()
        .expect("build client")
}

fn create_payment_request() -> CreatePaymentRequest {
    CreatePaymentRequest {
        amount: MonetaryAmount::new(Decimal::new(10000, 2), CurrencyCode::Rub),
        description: Some("Order #42".to_string()),
        receipt: None,
        recipient: Some(Recipient {
            gateway_id: "gw-001".into(),
        }),
        payment_token: None,
        payment_method_id: None,
        payment_method_data: None,
        confirmation: Some(ConfirmationData::Redirect {
            return_url: "https://example.com/return".to_string(),
            enforce: Some(true),
            locale: Some(Locale::RuRu),
        }),
        save_payment_method: Some(true),
        capture: Some(true),
        client_ip: Some("127.0.0.1".to_string()),
        metadata: None,
        airline: None,
        transfers: None,
        deal: None,
        merchant_customer_id: None,
        payment_order: None,
        receiver: None,
        statements: None,
    }
}

#[tokio::test]
async fn create_payment_sends_basic_auth_body_and_idempotence_key() {
    let (base_url, rx) = spawn_mock_server(MockResponse {
        status_line: "200 OK",
        body: payment_json("pay-123", "pending").to_string(),
        content_type: "application/json",
    });
    let client = build_client(base_url, Auth::basic("shop", "secret"));

    let payment = client
        .create_payment(&create_payment_request(), Some("fixed-key_123"))
        .await
        .expect("create payment");

    let request = rx.recv().expect("captured request");
    let body: Value = serde_json::from_slice(&request.body).expect("request json");

    assert_eq!(payment.id, PaymentId("pay-123".to_string()));
    assert_eq!(payment.status, PaymentStatus::Pending);
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/payments");
    assert_eq!(
        request.headers.get("authorization").map(String::as_str),
        Some("Basic c2hvcDpzZWNyZXQ=")
    );
    assert_eq!(
        request.headers.get("idempotence-key").map(String::as_str),
        Some("fixed-key_123")
    );
    assert_eq!(body["amount"]["value"], "100.00");
    assert_eq!(body["confirmation"]["type"], "redirect");
    assert_eq!(body["confirmation"]["locale"], "ru_RU");
    assert_eq!(body["recipient"]["gateway_id"], "gw-001");
}

#[tokio::test]
async fn list_payments_sends_bearer_auth_and_query_params() {
    let (base_url, rx) = spawn_mock_server(MockResponse {
        status_line: "200 OK",
        body: json!({
            "type": "list",
            "items": [payment_json("pay-1", "succeeded")],
            "next_cursor": "cursor-2"
        })
        .to_string(),
        content_type: "application/json",
    });
    let client = build_client(base_url, Auth::oauth("oauth-token"));
    let query = PaymentListQuery {
        time: TimeFilter {
            created_at_gte: None,
            created_at_gt: None,
            created_at_lte: None,
            created_at_lt: None,
            limit: Some(3),
            next_cursor: Some("cursor-1".to_string()),
        },
        captured_at_gte: None,
        captured_at_gt: None,
        captured_at_lte: None,
        captured_at_lt: None,
        payment_method: None,
        status: Some(PaymentStatus::Pending),
    };

    let response = client.list_payments(&query).await.expect("list payments");

    let request = rx.recv().expect("captured request");
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].id, PaymentId("pay-1".to_string()));
    assert_eq!(request.method, "GET");
    assert!(request.path.starts_with("/payments?"));
    assert!(request.path.contains("limit=3"));
    assert!(request.path.contains("cursor=cursor-1"));
    assert!(request.path.contains("status=pending"));
    assert_eq!(
        request.headers.get("authorization").map(String::as_str),
        Some("Bearer oauth-token")
    );
}

#[tokio::test]
async fn get_payment_parses_api_error_body() {
    let (base_url, rx) = spawn_mock_server(MockResponse {
        status_line: "404 Not Found",
        body: json!({
            "type": "error",
            "id": "err-1",
            "code": "not_found",
            "description": "payment not found",
            "parameter": "payment_id"
        })
        .to_string(),
        content_type: "application/json",
    });
    let client = build_client(base_url, Auth::oauth("oauth-token"));

    let error = client
        .get_payment(&PaymentId("missing".to_string()))
        .await
        .expect_err("expected api error");

    let request = rx.recv().expect("captured request");
    assert_eq!(request.method, "GET");
    assert_eq!(request.path, "/payments/missing");

    match error {
        YookassaError::Api { status, body } => {
            assert_eq!(status.as_u16(), 404);
            assert_eq!(body.code, Some(ErrorCode::NotFound));
            assert_eq!(body.description.as_deref(), Some("payment not found"));
            assert_eq!(body.parameter.as_deref(), Some("payment_id"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn get_payment_returns_unexpected_status_for_non_json_error() {
    let (base_url, rx) = spawn_mock_server(MockResponse {
        status_line: "502 Bad Gateway",
        body: "upstream failure".to_string(),
        content_type: "text/plain",
    });
    let client = build_client(base_url, Auth::oauth("oauth-token"));

    let error = client
        .get_payment(&PaymentId("pay-502".to_string()))
        .await
        .expect_err("expected unexpected status");

    let request = rx.recv().expect("captured request");
    assert_eq!(request.path, "/payments/pay-502");

    match error {
        YookassaError::UnexpectedStatus { status, body } => {
            assert_eq!(status.as_u16(), 502);
            assert_eq!(body, "upstream failure");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn create_payment_rejects_invalid_idempotence_key_before_request() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind unused server");
    let base_url = format!("http://{}", listener.local_addr().expect("local addr"));
    drop(listener);

    let client = build_client(base_url, Auth::basic("shop", "secret"));
    let error = client
        .create_payment(&create_payment_request(), Some("contains space"))
        .await
        .expect_err("expected idempotence validation error");

    assert!(matches!(error, YookassaError::InvalidIdempotenceKey));
}

#[test]
fn api_error_body_deserializes_unknown_error_code() {
    let body: ApiErrorBody = serde_json::from_value(json!({
        "code": "new_future_error_code",
        "description": "future-proof"
    }))
    .expect("deserialize api error");

    assert_eq!(body.code, Some(ErrorCode::Unknown));
    assert_eq!(body.description.as_deref(), Some("future-proof"));
}

#[tokio::test]
async fn webhook_endpoints_require_oauth_before_request() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind unused server");
    let base_url = format!("http://{}", listener.local_addr().expect("local addr"));
    drop(listener);

    let client = build_client(base_url, Auth::basic("shop", "secret"));

    let create_error = client
        .create_webhook(
            &CreateWebhookRequest {
                event: NotificationEventType::PaymentSucceeded,
                url: "https://example.com/webhook".to_string(),
            },
            Some("create-webhook-1"),
        )
        .await
        .expect_err("expected oauth required");
    assert!(matches!(create_error, YookassaError::OAuthRequired));

    let list_error = client
        .list_webhooks()
        .await
        .expect_err("expected oauth required");
    assert!(matches!(list_error, YookassaError::OAuthRequired));

    let delete_error = client
        .delete_webhook(
            &"1da5c87d-0984-50e8-a7f3-8de646dd9ec9".into(),
            Some("delete-webhook-1"),
        )
        .await
        .expect_err("expected oauth required");
    assert!(matches!(delete_error, YookassaError::OAuthRequired));
}
