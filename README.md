# Coffee Shop Order System - SOLID Principles in Rust

A fully functional coffee shop ordering system that demonstrates all five SOLID principles in Rust.

## Quick Start

```powershell
# Clone the repository
git clone https://github.com/40tude/coffee-shop-solid
cd coffee-shop-solid

# If you are an happy Win11 users
./start.ps1

# Run tests
cargo test

# Run the interactive demo
cargo run
```

## What This Project Demonstrates

This is a companion project to the blog post "SOLID Principles in Rust: A Practical Guide". While the blog post explains the theory with code snippets, this project shows a complete, working application where all five SOLID principles work together.

### The Coffee Shop Domain

The system models a simple coffee shop where:
- Customers place orders for beverages (Coffee, Tea, Smoothie)
- Each beverage can be customized (size, extras like milk, sugar)
- Orders are processed and stored
- Payments are handled
- Customers receive notifications
- Reports can be generated in different formats

## SOLID Principles in Action
* **S**ingle Responsibility Principle
* **O**pen-Closed Principle
* **L**iskov Substitution Principle
* **I**nterface Segregation Principle
* **D**ependency Inversion Principle

### 1. Single Responsibility Principle (SRP)

**"Each module should have one, and only one, reason to change."**

**In this project:**
- `PricingCalculator` - **only** calculates prices (Accounting's responsibility)
- `OrderService` - **only** manages order workflow (Operations' responsibility)
- `OrderRepository` - **only** handles persistence (DBA's responsibility)
- `Notifier` - **only** sends notifications (Customer Service's responsibility)

**See it in code:**
- `src/services/pricing_calculator.rs` - pricing logic isolated
- `src/services/order_service.rs` - order workflow isolated
- `src/ports/repository.rs` - storage abstraction
- `src/ports/notifier.rs` - notification abstraction

**Why it matters:** If accounting changes pricing rules, only `PricingCalculator` changes. If we switch from JSON to database storage, only the repository adapter changes.

### 2. Open-Closed Principle (OCP)

**"Software should be open for extension but closed for modification."**

**In this project:**
- Adding a new beverage type (e.g., `Latte`, `Cappuccino`) requires **zero changes** to existing code
- Adding a new payment method (e.g., `MobilePayment`) requires **zero changes** to `OrderService`
- Adding a new storage backend (e.g., `PostgresStorage`) requires **zero changes** to business logic

**See it in code:**
- `src/domain/beverage.rs` - `Beverage` trait defines the abstraction
- `src/ports/payment.rs` - `PaymentProcessor` trait allows new payment methods
- `src/ports/repository.rs` - `OrderRepository` trait allows new storage backends

**Try it yourself:**
1. Add a new beverage type in `src/domain/beverage.rs`
2. Notice that `OrderService`, `PricingCalculator`, and all other code continues to work without modification

### 3. Liskov Substitution Principle (LSP)

**"Any implementation of a trait should be substitutable without breaking the system."**

**In this project:**
- Any `PaymentProcessor` (Cash, CreditCard) can be used interchangeably
- Any `OrderRepository` (Memory, JSON) can be swapped without changing the service
- Any `Notifier` can replace another

**See it in code:**
- `src/adapters/cash_payment.rs` and `src/adapters/credit_card_payment.rs` both honor the `PaymentProcessor` contract
- `src/adapters/memory_storage.rs` and `src/adapters/json_storage.rs` both implement `OrderRepository` correctly

**Try it yourself:**
1. In `main.rs`, change from `MemoryOrderRepository` to `JsonOrderRepository`
2. The application works identically - that's LSP in action

### 4. Interface Segregation Principle (ISP)

**"Don't depend on interfaces you don't use."**

**In this project:**
- `PaymentProcessor` - focused on payment only
- `OrderRepository` - focused on storage only
- `Notifier` - focused on notifications only
- `Displayable` - focused on formatting only

Instead of one giant `OrderManager` trait with 20 methods, we have small, focused traits.

**See it in code:**
- `src/ports/` directory - each trait has a single, cohesive responsibility
- Notice how each adapter implements **only** the traits it needs

**Why it matters:** A simple console notifier doesn't need to know about payments or storage. Each component depends only on what it actually uses.

### 5. Dependency Inversion Principle (DIP)

**"High-level business logic should depend on abstractions, not implementations."**

**In this project:**
- `OrderService` (high-level) depends on **traits** (`OrderRepository`, `PaymentProcessor`, `Notifier`)
- Concrete implementations (low-level) depend on these same **traits**
- Dependencies point **inward** toward abstractions

**See it in code:**
- `src/services/order_service.rs` - depends on traits from `src/ports/`
- `src/adapters/` - implementations depend on the same traits
- `src/main.rs` - wires everything together (dependency injection)

**Try it yourself:**
1. Look at `OrderService` - it's generic over `R: OrderRepository`, `P: PaymentProcessor`, `N: Notifier`
2. Business logic has **zero knowledge** of JSON files, console output, or payment details
3. We can test business logic with mock implementations

## Project Structure

```
src/
├── main.rs                          # CLI and dependency injection
├── lib.rs                           # Public API
│
├── domain/                          # Pure business entities (no dependencies)
│   ├── mod.rs
│   ├── beverage.rs                  # Beverage trait and concrete types
│   ├── order.rs                     # Order entity
│   └── customer.rs                  # Customer entity
│
├── services/                        # Business logic (depends on domain + ports)
│   ├── mod.rs
│   ├── order_service.rs             # Order workflow orchestration
│   └── pricing_calculator.rs       # Pricing rules (SRP - Accounting's responsibility)
│
├── ports/                           # Trait definitions (interfaces)
│   ├── mod.rs
│   ├── repository.rs                # Storage abstraction (DIP)
│   ├── payment.rs                   # Payment abstraction (DIP, OCP)
│   └── notifier.rs                  # Notification abstraction (DIP, ISP)
│
└── adapters/                        # Concrete implementations (depends on ports)
    ├── mod.rs
    ├── memory_storage.rs            # In-memory repository
    ├── json_storage.rs              # JSON file repository
    ├── cash_payment.rs              # Cash payment processor
    ├── credit_card_payment.rs       # Credit card payment processor
    └── console_notifier.rs          # Console notification
```

### Dependency Flow (DIP in action)

```
main.rs ──┐
          ├──> OrderService (high-level)
          │         │
          │         ├──> OrderRepository (trait)
          │         ├──> PaymentProcessor (trait)
          │         └──> Notifier (trait)
          │                   ▲
          │                   │
          └──────> MemoryOrderRepository (low-level, implements trait)
                   JsonOrderRepository (low-level, implements trait)
                   CashPayment (low-level, implements trait)
                   ConsoleNotifier (low-level, implements trait)
```

Notice: Dependencies point **inward** (toward abstractions), never outward.

## Running the Demo

### Interactive Mode

```bash
cargo run
```

This starts an interactive CLI where you can:
1. Create orders with different beverages
2. Choose payment methods
3. See how SOLID principles allow easy extension

### Extending the System

#### Add a New Beverage (OCP)

Edit `src/domain/beverage.rs`:

```rust
pub struct Latte {
    pub size: Size,
    pub shots: u8,
}

impl Beverage for Latte {
    fn name(&self) -> String {
        format!("Latte ({} shot{})", self.shots, if self.shots > 1 { "s" } else { "" })
    }

    fn base_price(&self) -> f64 {
        4.50 + (self.shots as f64 - 1.0) * 0.50
    }

    fn size(&self) -> Size {
        self.size
    }
}
```

That's it! No changes needed anywhere else. OCP in action. ✓

#### Add a New Payment Method (OCP, DIP)

Create `src/adapters/mobile_payment.rs`:

```rust
pub struct MobilePayment;

impl PaymentProcessor for MobilePayment {
    fn process_payment(&self, amount: f64) -> Result<String, PaymentError> {
        println!("💳 Processing mobile payment of ${:.2}", amount);
        Ok(format!("MOBILE-{}", Uuid::new_v4()))
    }
}
```

Wire it up in `main.rs`. Business logic unchanged. OCP + DIP in action. ✓

#### Add Database Storage (OCP, DIP)

Create `src/adapters/postgres_storage.rs`:

```rust
pub struct PostgresOrderRepository {
    pool: sqlx::PgPool,
}

impl OrderRepository for PostgresOrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), RepositoryError> {
        // SQL implementation
    }
    // ... other methods
}
```

Business logic (`OrderService`) requires **zero changes**. DIP in action. ✓

## Testing

Run tests:

```bash
cargo test
```

Notice how easy testing is thanks to SOLID:
- We can test `OrderService` with `MemoryOrderRepository` (no database needed)
- We can test pricing without touching storage
- Each component is tested in isolation

## Learning Path

1. **Start with the domain** (`src/domain/`) - pure business entities, no dependencies
2. **Read the ports** (`src/ports/`) - these are the abstractions (DIP)
3. **Explore services** (`src/services/`) - business logic depending on abstractions
4. **Check adapters** (`src/adapters/`) - implementations of abstractions
5. **See it wired together** (`src/main.rs`) - dependency injection

## Key Takeaways

- **SOLID makes change easy**: Want a new beverage? Add it. New payment method? Add it. Switch storage? Swap the adapter.
- **SOLID makes testing easy**: Business logic has no hard dependencies, so testing uses simple in-memory implementations.
- **SOLID makes collaboration easy**: Different teams can work on different adapters without conflicts.
- **Rust enforces SOLID**: The type system and module system make violations hard.

## Further Reading

- Blog post: [SOLID Principles in Rust: A Practical Guide](link-to-come)
- [Clean Architecture](https://amzn.eu/d/1KhmQKq) by Robert C. Martin
- [Detailed summary](https://github.com/serodriguez68/clean-architecture/tree/master)
- Rust Book - Traits: https://doc.rust-lang.org/book/ch10-02-traits.html

## License

MIT

## Contributing
This project is developed for personal and educational purposes. Feel free to explore and use it to enhance your own learning.

Given the nature of the project, external contributions are not actively sought nor encouraged. However, constructive feedback aimed at improving the project (in terms of speed, accuracy, comprehensiveness, etc.) is welcome. Please note that this project is being created as a hobby and is unlikely to be maintained once my initial goal has been achieved.