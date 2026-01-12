// Coffee Shop Order System - Interactive Demo
//
// This demo shows all SOLID principles working together in a real application.
//
// Run: cargo run
//
// The demo allows us to:
// 1. Create orders with different beverages
// 2. Choose payment methods (demonstrating OCP - open close principle)
// 3. See how components work together (demonstrating DIP - dependency inversion principle)
// 4. Swap implementations easily (demonstrating LSP - Liskov substitution principle)

use coffee_shop_solid::*;
use std::io::{self, Write};

fn main() {
    println!("☕ Coffee Shop Order System - SOLID Principles Demo");
    println!("====================================================\n");

    // DEPENDENCY INJECTION (DIP in action)
    // We create concrete implementations and inject them into OrderService
    // OrderService depends on TRAITS, not these specific types
    let repository = MemoryOrderRepository::new();
    let payment = CashPayment;
    let notifier = ConsoleNotifier;

    // Create the service
    // Notice: OrderService is generic over the trait bounds
    // It doesn't know it's using Memory, Cash, or Console
    // It only knows about OrderRepository, PaymentProcessor, and Notifier traits
    let mut service = OrderService::new(repository, payment, notifier);

    println!("📝 System initialized with:");
    println!("  - Storage: In-Memory (fast, no persistence)");
    println!("  - Payment: Cash");
    println!("  - Notifications: Console");
    println!("\n💡 TIP: To use different implementations, just change the initialization above!");
    println!("  Example: let repository = JsonOrderRepository::new(\"orders.json\".into())?;");
    println!("  Example: let payment = CreditCardPayment::new(...);");
    println!("\n");

    // Interactive demo loop
    loop {
        println!("\n=== Main Menu ===");
        println!("1. Place a new order");
        println!("2. List all orders");
        println!("3. Demonstrate OCP (Open-Closed Principle)");
        println!("4. Demonstrate LSP (Liskov Substitution Principle)");
        println!("5. Demonstrate DIP (Dependency Inversion Principle)");
        println!("6. Exit");
        print!("\nChoose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => place_order_interactive(&mut service),
            "2" => list_orders(&service),
            "3" => demonstrate_ocp(),
            "4" => demonstrate_lsp(),
            "5" => demonstrate_dip(),
            "6" => {
                println!("\nThank you for exploring SOLID principles! 🎉");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

/// Interactive order placement
fn place_order_interactive<R, P, N>(service: &mut OrderService<R, P, N>)
where
    R: OrderRepository,
    P: PaymentProcessor,
    N: Notifier,
{
    println!("\n=== Place New Order ===");

    // Get customer info
    println!("\nCustomer Information:");
    print!("Name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    print!("Email: ");
    io::stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin().read_line(&mut email).unwrap();

    let customer = Customer::new(name.trim().to_string(), email.trim().to_string(), None);

    // Get beverage order
    println!("\n=== Beverage Selection ===");
    println!("Available beverages:");
    println!("1. Coffee (Small: $2.80, Medium: $3.50, Large: $4.20)");
    println!("2. Tea (Small: $2.00, Medium: $2.50, Large: $3.00)");
    println!("3. Smoothie (Small: $4.00, Medium: $5.00, Large: $6.00)");

    print!("\nChoose beverage type (1-3): ");
    io::stdout().flush().unwrap();
    let mut beverage_choice = String::new();
    io::stdin().read_line(&mut beverage_choice).unwrap();

    print!("Choose size (S/M/L): ");
    io::stdout().flush().unwrap();
    let mut size_choice = String::new();
    io::stdin().read_line(&mut size_choice).unwrap();

    let size = match size_choice.trim().to_uppercase().as_str() {
        "S" => Size::Small,
        "L" => Size::Large,
        _ => Size::Medium,
    };

    // Create beverage
    // OCP: We can add new beverage types without modifying this code
    let beverage: Box<dyn Beverage> = match beverage_choice.trim() {
        "1" => {
            print!("Extra shots? (0-3): ");
            io::stdout().flush().unwrap();
            let mut shots = String::new();
            io::stdin().read_line(&mut shots).unwrap();
            let extra_shots = shots.trim().parse().unwrap_or(0);

            Box::new(Coffee { size, extra_shots })
        }
        "2" => {
            print!("Tea variety (Green/Black/Herbal): ");
            io::stdout().flush().unwrap();
            let mut variety = String::new();
            io::stdin().read_line(&mut variety).unwrap();

            Box::new(Tea {
                size,
                variety: variety.trim().to_string(),
            })
        }
        "3" => {
            print!("Fruits (comma-separated, e.g., Strawberry,Banana): ");
            io::stdout().flush().unwrap();
            let mut fruits_input = String::new();
            io::stdin().read_line(&mut fruits_input).unwrap();

            let fruits: Vec<String> = fruits_input
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            Box::new(Smoothie { size, fruits })
        }
        _ => Box::new(Coffee {
            size,
            extra_shots: 0,
        }),
    };

    // Show price preview
    println!("\n--- Order Summary ---");
    println!("Beverage: {}", beverage.description());
    println!("Price: ${:.2}", beverage.price());

    print!("\nConfirm order? (y/n): ");
    io::stdout().flush().unwrap();
    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm).unwrap();

    if confirm.trim().to_lowercase() != "y" {
        println!("Order cancelled.");
        return;
    }

    // Place the order
    // DIP: service.place_order() works with any repository, payment, notifier
    // It doesn't know we're using Memory, Cash, Console
    match service.place_order(customer, vec![beverage]) {
        Ok(order) => {
            println!("\n✅ Order placed successfully!");
            println!("Order ID: {}", order.id);
            println!("Status: {:?}", order.status);
        }
        Err(e) => {
            println!("\n❌ Error placing order: {}", e);
        }
    }
}

/// List all orders
fn list_orders<R, P, N>(service: &OrderService<R, P, N>)
where
    R: OrderRepository,
    P: PaymentProcessor,
    N: Notifier,
{
    println!("\n=== All Orders ===");

    match service.list_all_orders() {
        Ok(orders) => {
            if orders.is_empty() {
                println!("No orders yet. Place one to get started!");
            } else {
                for order in orders {
                    println!("\n---------------------------");
                    println!("Order ID: {}", order.id);
                    println!(
                        "Customer: {} ({})",
                        order.customer.name, order.customer.email
                    );
                    println!("Items: {}", order.items.len());
                    println!("Total: ${:.2}", order.total_price);
                    println!("Status: {:?}", order.status);
                    println!("Created: {}", order.created_at.format("%Y-%m-%d %H:%M:%S"));
                }
            }
        }
        Err(e) => {
            println!("Error listing orders: {}", e);
        }
    }
}

/// Demonstrate Open-Closed Principle
fn demonstrate_ocp() {
    println!("\n=== OPEN-CLOSED PRINCIPLE (OCP) ===");
    println!(
        "\nOCP states: \"Software should be open for extension but closed for modification.\""
    );
    println!("\nIn this system:");
    println!("\n1. BEVERAGES (Open for Extension):");
    println!("   - We have Coffee, Tea, Smoothie");
    println!("   - To add a new beverage (e.g., Latte), we:");
    println!("     • Create a new struct: pub struct Latte {{ size: Size, shots: u8 }}");
    println!("     • Implement Beverage trait: impl Beverage for Latte {{ ... }}");
    println!("     • That's it! No changes to OrderService, PricingCalculator, or any other code");
    println!("\n2. PAYMENT METHODS (Open for Extension):");
    println!("   - We have CashPayment, CreditCardPayment");
    println!("   - To add MobilePayment:");
    println!("     • Create: pub struct MobilePayment;");
    println!("     • Implement: impl PaymentProcessor for MobilePayment {{ ... }}");
    println!("     • No changes to OrderService!");
    println!("\n3. STORAGE BACKENDS (Open for Extension):");
    println!("   - We have MemoryOrderRepository, JsonOrderRepository");
    println!("   - To add PostgresOrderRepository:");
    println!("     • Create the struct with database connection");
    println!("     • Implement OrderRepository trait");
    println!("     • No changes to business logic!");
    println!("\n💡 The system is CLOSED for modification (existing code doesn't change)");
    println!("   but OPEN for extension (new features are easy to add).");
}

/// Demonstrate Liskov Substitution Principle
fn demonstrate_lsp() {
    println!("\n=== LISKOV SUBSTITUTION PRINCIPLE (LSP) ===");
    println!("\nLSP states: \"Any implementation of a trait should be substitutable.\"");
    println!("\nIn this system:");
    println!("\n1. REPOSITORY SUBSTITUTION:");
    println!("   This code works with ANY OrderRepository:");
    println!("   ```rust");
    println!("   fn save_and_retrieve(repo: &mut dyn OrderRepository) {{");
    println!("       repo.save(&order)?;");
    println!("       let found = repo.find_by_id(order.id)?;");
    println!("   }}");
    println!("   ```");
    println!("   - Works with MemoryOrderRepository");
    println!("   - Works with JsonOrderRepository");
    println!("   - Would work with PostgresOrderRepository");
    println!("   All implementations honor the SAME CONTRACT.");
    println!("\n2. PAYMENT SUBSTITUTION:");
    println!("   OrderService doesn't care which payment method:");
    println!("   ```rust");
    println!("   let service1 = OrderService::new(repo, CashPayment, notifier);");
    println!("   let service2 = OrderService::new(repo, CreditCardPayment::new(...), notifier);");
    println!("   ```");
    println!("   Both work identically. Same interface, predictable behavior.");
    println!("\n💡 LSP ensures we can swap implementations without surprises.");
    println!("   If it implements the trait, it MUST behave correctly.");
}

/// Demonstrate Dependency Inversion Principle
fn demonstrate_dip() {
    println!("\n=== DEPENDENCY INVERSION PRINCIPLE (DIP) ===");
    println!("\nDIP states: \"Depend on abstractions, not concretions.\"");
    println!("\nTraditional (BAD) dependency flow:");
    println!("   OrderService -> PostgresOrderRepository -> postgres crate");
    println!("   (high-level)    (low-level)               (external)");
    println!("\nWith DIP (GOOD) - dependencies point inward:");
    println!("   OrderService -> OrderRepository <- PostgresOrderRepository");
    println!("   (high-level)    (abstraction)      (low-level)");
    println!("\nIn this system:");
    println!("\n1. ORDERSERVICE DEPENDS ON TRAITS:");
    println!("   ```rust");
    println!("   pub struct OrderService<R, P, N>");
    println!("   where");
    println!("       R: OrderRepository,    // Abstraction, not MemoryOrderRepository");
    println!("       P: PaymentProcessor,   // Abstraction, not CashPayment");
    println!("       N: Notifier,          // Abstraction, not ConsoleNotifier");
    println!("   ```");
    println!("\n2. BENEFITS:");
    println!("   - Business logic is INDEPENDENT of infrastructure");
    println!("   - We can test with mock implementations (no database needed)");
    println!("   - We can swap implementations at runtime");
    println!("   - Different teams can work on adapters independently");
    println!("\n3. TESTING EXAMPLE:");
    println!("   ```rust");
    println!("   let test_repo = MemoryOrderRepository::new();");
    println!("   let test_payment = MockPayment;");
    println!("   let test_notifier = MockNotifier;");
    println!("   let service = OrderService::new(test_repo, test_payment, test_notifier);");
    println!("   // Test business logic without ANY infrastructure!");
    println!("   ```");
    println!("\n💡 DIP is the key to Clean Architecture.");
    println!("   It allows us to keep business logic pure and testable.");
}
