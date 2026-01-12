# Extension Examples - How to Add New Features

This document shows how easy it is to extend the system thanks to SOLID principles, particularly the Open-Closed Principle (OCP).

## Adding a New Beverage Type

Want to add Latte to the menu? Here's the complete code needed:

### 1. Create the new beverage type

Add this to `src/domain/beverage.rs`:

```rust
/// A latte with customizable shots and milk type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Latte {
    pub size: Size,
    pub shots: u8,
    pub milk_type: String, // "Whole", "Skim", "Oat", "Almond", etc.
}

impl Beverage for Latte {
    fn name(&self) -> String {
        format!("Latte ({} shot{}, {} milk)",
            self.shots,
            if self.shots > 1 { "s" } else { "" },
            self.milk_type
        )
    }
    
    fn base_price(&self) -> f64 {
        let base = 4.50;
        let shot_price = (self.shots as f64).max(1.0) - 1.0;
        let milk_premium = if self.milk_type == "Oat" || self.milk_type == "Almond" {
            0.75
        } else {
            0.0
        };
        
        base + shot_price * 0.75 + milk_premium
    }
    
    fn size(&self) -> Size {
        self.size
    }
}
```

### 2. That's it!

No other changes needed. The system automatically supports Latte:
- OrderService works with it (depends on Beverage trait)
- PricingCalculator works with it (depends on Beverage trait)
- Repository can store it (Order serializes beverages)
- All existing code continues to work

### 3. Use it

```rust
let latte = Latte {
    size: Size::Large,
    shots: 2,
    milk_type: "Oat".to_string(),
};

let beverages: Vec<Box<dyn Beverage>> = vec![Box::new(latte)];
let order = service.place_order(customer, beverages)?;
```

**That's OCP in action!**

---

## Adding a New Payment Method

Want to add cryptocurrency payment? Here's how:

### 1. Create the payment processor

Create `src/adapters/crypto_payment.rs`:

```rust
use crate::ports::{PaymentError, PaymentProcessor};
use uuid::Uuid;

pub struct CryptoPayment {
    wallet_address: String,
    blockchain: String, // "Bitcoin", "Ethereum", etc.
}

impl CryptoPayment {
    pub fn new(wallet_address: String, blockchain: String) -> Self {
        Self { wallet_address, blockchain }
    }
}

impl PaymentProcessor for CryptoPayment {
    fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
        println!("₿ Processing {} payment of ${:.2}", self.blockchain, amount);
        
        // In real implementation:
        // 1. Convert USD to crypto
        // 2. Create transaction
        // 3. Wait for confirmations
        // 4. Return transaction hash
        
        let tx_hash = format!("CRYPTO-{}", Uuid::new_v4());
        println!("✓ Payment confirmed: {}", tx_hash);
        
        Ok(tx_hash)
    }
    
    fn payment_method_name(&self) -> &str {
        &self.blockchain
    }
}
```

### 2. Register in adapters module

Add to `src/adapters/mod.rs`:

```rust
pub mod crypto_payment;
pub use crypto_payment::CryptoPayment;
```

### 3. Use it

```rust
let payment = CryptoPayment::new(
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
    "Bitcoin".to_string()
);
let service = OrderService::new(repository, payment, notifier);
```

**Zero changes to OrderService, Order, or any business logic!**

---

## Adding a New Storage Backend

Want to use PostgreSQL instead of memory or JSON? Here's how:

### 1. Add dependencies

In `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }
tokio = { version = "1.0", features = ["full"] }
```

### 2. Create the repository

Create `src/adapters/postgres_storage.rs`:

```rust
use crate::domain::Order;
use crate::ports::{OrderRepository, RepositoryError};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn save(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let order_json = serde_json::to_string(order)
            .map_err(|e| RepositoryError::SaveFailed(e.to_string()))?;
        
        sqlx::query!(
            "INSERT INTO orders (id, data) VALUES ($1, $2)",
            order.id,
            order_json
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::SaveFailed(e.to_string()))?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, RepositoryError> {
        let row = sqlx::query!(
            "SELECT data FROM orders WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::LoadFailed(e.to_string()))?;
        
        match row {
            Some(r) => {
                let order: Order = serde_json::from_str(&r.data)
                    .map_err(|e| RepositoryError::LoadFailed(e.to_string()))?;
                Ok(Some(order))
            }
            None => Ok(None)
        }
    }
    
    // ... implement other methods
}
```

### 3. Use it

```rust
let repository = PostgresOrderRepository::new("postgres://...").await?;
let service = OrderService::new(repository, payment, notifier);
```

**Business logic remains unchanged!**

---

## Adding a New Notification Channel

Want to send SMS notifications? Here's how:

### 1. Create the notifier

Create `src/adapters/sms_notifier.rs`:

```rust
use crate::domain::Order;
use crate::ports::{NotificationError, Notifier};

pub struct SmsNotifier {
    twilio_client: TwilioClient,
}

impl SmsNotifier {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        Self {
            twilio_client: TwilioClient::new(account_sid, auth_token)
        }
    }
}

impl Notifier for SmsNotifier {
    fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError> {
        let message = format!(
            "Your order #{} has been placed! Total: ${:.2}",
            &order.id.to_string()[..8],
            order.total_price
        );
        
        if let Some(phone) = &order.customer.phone {
            self.twilio_client
                .send_sms(phone, &message)
                .map_err(|e| NotificationError::SendFailed(e.to_string()))?;
        }
        
        Ok(())
    }
    
    fn notify_order_ready(&self, order: &Order) -> Result<(), NotificationError> {
        let message = format!(
            "Your order #{} is ready for pickup!",
            &order.id.to_string()[..8]
        );
        
        if let Some(phone) = &order.customer.phone {
            self.twilio_client
                .send_sms(phone, &message)
                .map_err(|e| NotificationError::SendFailed(e.to_string()))?;
        }
        
        Ok(())
    }
    
    fn notify_order_cancelled(&self, order: &Order) -> Result<(), NotificationError> {
        // Similar implementation
        Ok(())
    }
}
```

### 2. Use multiple notifiers

Want both console AND SMS? Create a composite:

```rust
pub struct CompositeNotifier {
    notifiers: Vec<Box<dyn Notifier>>,
}

impl CompositeNotifier {
    pub fn new() -> Self {
        Self { notifiers: vec![] }
    }
    
    pub fn add(&mut self, notifier: Box<dyn Notifier>) {
        self.notifiers.push(notifier);
    }
}

impl Notifier for CompositeNotifier {
    fn notify_order_placed(&self, order: &Order) -> Result<(), NotificationError> {
        for notifier in &self.notifiers {
            // Try all, ignore individual failures
            let _ = notifier.notify_order_placed(order);
        }
        Ok(())
    }
    
    // ... other methods
}

// Usage
let mut notifier = CompositeNotifier::new();
notifier.add(Box::new(ConsoleNotifier));
notifier.add(Box::new(SmsNotifier::new(...)));
notifier.add(Box::new(EmailNotifier::new(...)));

let service = OrderService::new(repository, payment, notifier);
```

**Now orders trigger console, SMS, AND email notifications!**

---

## Key Takeaways

1. **Adding new features is easy** - create new types, implement traits, done
2. **Existing code doesn't change** - that's Open-Closed Principle
3. **Everything is substitutable** - that's Liskov Substitution Principle
4. **Business logic stays pure** - that's Dependency Inversion Principle
5. **Each component is focused** - that's Interface Segregation Principle
6. **Each module has one job** - that's Single Responsibility Principle

**That's SOLID working together to make the codebase maintainable, extensible, and testable!**
