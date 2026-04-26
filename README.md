# yookasa-sdk

Async Rust SDK for the [YooKassa HTTP API](https://yookassa.ru/developers/api).

`yookasa-sdk` provides a typed client for common YooKassa resources, including payments, refunds, payment methods, deals, invoices, payouts, receipts, webhooks, and `me`.

- Official API docs: <https://yookassa.ru/developers/api>
- OpenAPI specification: <https://yookassa.ru/developers/api/yookassa-openapi-specification.yaml>

## English

### Features

- Async HTTP client built on `reqwest`
- `Basic Auth` and `OAuth` support
- Typed request/response models for the main YooKassa API objects
- Built-in idempotence key handling for `POST` and `DELETE`
- Unified error type: `YookassaError`
- `rust_decimal`-based money amounts
- `chrono`-based date/time filters

### Installation

```toml
[dependencies]
yookasa-sdk = "0.1.0"
rust_decimal = "1"
```

### Quick Start

```rust
use rust_decimal::Decimal;
use yookasa_sdk::{Auth, YookassaClient};
use yookasa_sdk::model::{
    ConfirmationData, CreatePaymentRequest, CurrencyCode, Locale, MonetaryAmount,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YookassaClient::new(Auth::basic("shop_id", "secret_key"))?;

    let payment = client
        .create_payment(
            &CreatePaymentRequest {
                amount: MonetaryAmount::new(Decimal::new(10000, 2), CurrencyCode::Rub),
                description: Some("Order #42".to_string()),
                receipt: None,
                recipient: None,
                payment_token: None,
                payment_method_id: None,
                payment_method_data: None,
                confirmation: Some(ConfirmationData::Redirect {
                    return_url: "https://example.com/return".to_string(),
                    enforce: None,
                    locale: Some(Locale::RuRu),
                }),
                save_payment_method: Some(false),
                capture: Some(true),
                client_ip: None,
                metadata: None,
                airline: None,
                transfers: None,
                deal: None,
                merchant_customer_id: None,
                payment_order: None,
                receiver: None,
                statements: None,
            },
            None,
        )
        .await?;

    println!("payment_id={}", payment.id.as_ref());
    Ok(())
}
```

### Authentication

`yookasa-sdk` supports both authentication modes exposed by YooKassa:

```rust
use yookasa_sdk::Auth;

let basic = Auth::basic("shop_id", "secret_key");
let oauth = Auth::oauth("oauth_token");
```

`webhooks` are OAuth-only in YooKassa. The SDK enforces this and returns `YookassaError::OAuthRequired` if you try to call webhook endpoints with `Basic Auth`.

### Client Configuration

```rust
use std::time::Duration;
use yookasa_sdk::{Auth, YookassaClient};

let client = YookassaClient::builder(Auth::basic("shop_id", "secret_key"))
    .timeout(Duration::from_secs(15))
    .user_agent("my-app/1.0")
    .build()?;
```

Supported builder options:

- `base_url(...)`
- `timeout(...)`
- `user_agent(...)`
- `build()`

### Main API Methods

The client currently exposes methods for:

- Payments: `create_payment`, `list_payments`, `get_payment`, `capture_payment`, `cancel_payment`
- Refunds: `create_refund`, `list_refunds`, `get_refund`
- Payment methods: `create_payment_method`, `get_payment_method`
- Deals: `create_deal`, `list_deals`, `get_deal`
- Invoices: `create_invoice`, `get_invoice`
- Payouts: `create_payout`, `list_payouts`, `search_payouts`, `get_payout`
- SBP banks: `list_sbp_banks`
- Personal data: `create_personal_data`, `get_personal_data`
- Receipts: `create_receipt`, `list_receipts`, `get_receipt`
- Webhooks: `create_webhook`, `list_webhooks`, `delete_webhook`
- Settings: `get_me`

### Error Handling

All async client methods return:

```rust
Result<T, YookassaError>
```

The main error variants are:

- `YookassaError::Http`
- `YookassaError::Json`
- `YookassaError::InvalidIdempotenceKey`
- `YookassaError::OAuthRequired`
- `YookassaError::Api`
- `YookassaError::UnexpectedStatus`

Example:

```rust
use yookasa_sdk::{ErrorCode, YookassaError};

match client.get_payment(&"payment_id".into()).await {
    Ok(payment) => println!("{}", payment.id.as_ref()),
    Err(YookassaError::Api { status, body }) => {
        println!("api error: {} {:?}", status, body.code);
        if body.code == Some(ErrorCode::NotFound) {
            println!("payment not found");
        }
    }
    Err(err) => println!("unexpected error: {err}"),
}
```

### Notes

- Money is represented by `MonetaryAmount` and serialized as a string amount, for example `"100.00"`.
- IDs are represented as newtype wrappers over `String`, such as `PaymentId`, `RefundId`, `WebhookId`, and others.
- Some less frequently used nested YooKassa objects are still represented as `serde_json::Value` instead of fully typed structs.

---

## Русский

### Что это

`yookasa-sdk` — асинхронный Rust SDK для работы с HTTP API YooKassa.

Crate предоставляет:

- HTTP-клиент `YookassaClient`
- поддержку `Basic Auth` и `OAuth`
- типизированные модели для основных объектов API
- автоматическую работу с `Idempotence-Key` для `POST` и `DELETE`
- единый тип ошибок `YookassaError`
- денежные суммы на `rust_decimal`
- фильтры дат и времени на `chrono`

### Установка

```toml
[dependencies]
yookasa-sdk = "0.1.0"
rust_decimal = "1"
```

### Быстрый старт

```rust
use rust_decimal::Decimal;
use yookasa_sdk::{Auth, YookassaClient};
use yookasa_sdk::model::{
    ConfirmationData, CreatePaymentRequest, CurrencyCode, Locale, MonetaryAmount,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = YookassaClient::new(Auth::basic("shop_id", "secret_key"))?;

    let payment = client
        .create_payment(
            &CreatePaymentRequest {
                amount: MonetaryAmount::new(Decimal::new(10000, 2), CurrencyCode::Rub),
                description: Some("Order #42".to_string()),
                receipt: None,
                recipient: None,
                payment_token: None,
                payment_method_id: None,
                payment_method_data: None,
                confirmation: Some(ConfirmationData::Redirect {
                    return_url: "https://example.com/return".to_string(),
                    enforce: None,
                    locale: Some(Locale::RuRu),
                }),
                save_payment_method: Some(false),
                capture: Some(true),
                client_ip: None,
                metadata: None,
                airline: None,
                transfers: None,
                deal: None,
                merchant_customer_id: None,
                payment_order: None,
                receiver: None,
                statements: None,
            },
            None,
        )
        .await?;

    println!("payment_id={}", payment.id.as_ref());
    Ok(())
}
```

### Аутентификация

Поддерживаются оба режима аутентификации YooKassa:

```rust
use yookasa_sdk::Auth;

let basic = Auth::basic("shop_id", "secret_key");
let oauth = Auth::oauth("oauth_token");
```

Для `webhooks` YooKassa требует OAuth. SDK проверяет это явно и возвращает `YookassaError::OAuthRequired`, если вызвать webhook-метод с `Basic Auth`.

### Настройка клиента

```rust
use std::time::Duration;
use yookasa_sdk::{Auth, YookassaClient};

let client = YookassaClient::builder(Auth::basic("shop_id", "secret_key"))
    .timeout(Duration::from_secs(15))
    .user_agent("my-app/1.0")
    .build()?;
```

Доступные настройки:

- `base_url(...)`
- `timeout(...)`
- `user_agent(...)`
- `build()`

### Основные методы клиента

Сейчас в crate реализованы методы для:

- платежей
- возвратов
- способов оплаты
- сделок
- счетов
- выплат
- участников СБП
- персональных данных
- чеков
- webhook
- `me`

### Обработка ошибок

Все асинхронные методы клиента возвращают:

```rust
Result<T, YookassaError>
```

Основные варианты ошибок:

- `YookassaError::Http`
- `YookassaError::Json`
- `YookassaError::InvalidIdempotenceKey`
- `YookassaError::OAuthRequired`
- `YookassaError::Api`
- `YookassaError::UnexpectedStatus`

Пример:

```rust
use yookasa_sdk::{ErrorCode, YookassaError};

match client.get_payment(&"payment_id".into()).await {
    Ok(payment) => println!("{}", payment.id.as_ref()),
    Err(YookassaError::Api { status, body }) => {
        println!("api error: {} {:?}", status, body.code);
        if body.code == Some(ErrorCode::NotFound) {
            println!("платеж не найден");
        }
    }
    Err(err) => println!("unexpected error: {err}"),
}
```

### Важно

- Денежные суммы представлены типом `MonetaryAmount` и сериализуются строкой, например `"100.00"`.
- Идентификаторы представлены newtype-обертками над `String`: `PaymentId`, `RefundId`, `WebhookId` и другие.
- Некоторые редкие вложенные объекты YooKassa пока остаются в виде `serde_json::Value`, а не полностью типизированных структур.
