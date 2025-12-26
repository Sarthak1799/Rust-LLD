use std::sync::Arc;

use spliter::{
    models::split::{EqualSplit, PercentageSplit},
    services::splitter_service::Service,
};

fn test_basic_expense_and_settlement() {
    println!("\n=== Test 1: Basic Expense and Settlement ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone()]);
    let split = EqualSplit::new(vec![alice.clone(), bob.clone()], 100.0);

    service
        .add_expense_to_group(
            &group_id,
            String::from("Dinner"),
            100.0,
            alice.clone(),
            vec![Arc::new(split)],
        )
        .expect("Failed to add expense to group");

    let balances = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance sheet");
    println!("Bob's balance after expense:");
    for (user_id, amount) in &balances {
        println!("  Owes {} -> ${:.2}", user_id, amount);
    }
    assert!(!balances.is_empty(), "Bob should have a balance");

    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle balance");

    let balances = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance sheet");
    println!("Bob's balance after settlement:");
    if balances.is_empty() {
        println!("  All settled! ✓");
    }
    assert!(
        balances.is_empty(),
        "Bob should have no balance after settlement"
    );
}

fn test_multiple_expenses() {
    println!("\n=== Test 2: Multiple Expenses Accumulate ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());
    let charlie = service.add_user("charlie".to_string(), "charlie@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone(), charlie.clone()]);

    // Expense 1: Alice pays $90 for all three
    let split1 = EqualSplit::new(vec![alice.clone(), bob.clone(), charlie.clone()], 90.0);
    service
        .add_expense_to_group(
            &group_id,
            "Lunch".to_string(),
            90.0,
            alice.clone(),
            vec![Arc::new(split1)],
        )
        .expect("Failed to add expense 1");

    // Expense 2: Alice pays $60 for all three
    let split2 = EqualSplit::new(vec![alice.clone(), bob.clone(), charlie.clone()], 60.0);
    service
        .add_expense_to_group(
            &group_id,
            "Movie".to_string(),
            60.0,
            alice.clone(),
            vec![Arc::new(split2)],
        )
        .expect("Failed to add expense 2");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    println!("Bob's balance after 2 expenses:");
    for (user_id, amount) in &bob_balance {
        println!("  Owes {} -> ${:.2}", user_id, amount);
    }
    assert_eq!(
        *bob_balance.get(&alice).unwrap(),
        50.0,
        "Bob should owe Alice $50 (30+20)"
    );

    // Settle balances
    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle Bob's balance");
    service
        .settle_balance(&charlie, &alice)
        .expect("Failed to settle Charlie's balance");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("After settlement:");
    println!(
        "  Bob's balance: {}",
        if bob_balance.is_empty() {
            "Settled ✓"
        } else {
            "Still has debt"
        }
    );
    println!(
        "  Charlie's balance: {}",
        if charlie_balance.is_empty() {
            "Settled ✓"
        } else {
            "Still has debt"
        }
    );

    assert!(
        bob_balance.is_empty(),
        "Bob should have no balance after settlement"
    );
    assert!(
        charlie_balance.is_empty(),
        "Charlie should have no balance after settlement"
    );
}

fn test_three_people_split() {
    println!("\n=== Test 3: Three People Equal Split ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());
    let charlie = service.add_user("charlie".to_string(), "charlie@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone(), charlie.clone()]);

    // Alice pays $90 split equally among all three
    let split = EqualSplit::new(vec![alice.clone(), bob.clone(), charlie.clone()], 90.0);
    service
        .add_expense_to_group(
            &group_id,
            "Dinner".to_string(),
            90.0,
            alice.clone(),
            vec![Arc::new(split)],
        )
        .expect("Failed to add expense");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("Balances after $90 dinner paid by Alice:");
    println!(
        "  Bob owes Alice -> ${:.2}",
        bob_balance.get(&alice).unwrap_or(&0.0)
    );
    println!(
        "  Charlie owes Alice -> ${:.2}",
        charlie_balance.get(&alice).unwrap_or(&0.0)
    );

    assert_eq!(
        *bob_balance.get(&alice).unwrap(),
        30.0,
        "Bob should owe $30"
    );
    assert_eq!(
        *charlie_balance.get(&alice).unwrap(),
        30.0,
        "Charlie should owe $30"
    );

    // Settle balances
    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle Bob's balance");
    service
        .settle_balance(&charlie, &alice)
        .expect("Failed to settle Charlie's balance");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("After settlement:");
    println!("  Bob: Settled ✓");
    println!("  Charlie: Settled ✓");

    assert!(
        bob_balance.is_empty(),
        "Bob should have no balance after settlement"
    );
    assert!(
        charlie_balance.is_empty(),
        "Charlie should have no balance after settlement"
    );
}

fn test_one_person_split() {
    println!("\n=== Test 4: Single Person Split (Bob only) ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone()]);

    // Alice pays $100 but only Bob is in the split
    let split = EqualSplit::new(vec![bob.clone()], 100.0);
    service
        .add_expense_to_group(
            &group_id,
            "Bob's item".to_string(),
            100.0,
            alice.clone(),
            vec![Arc::new(split)],
        )
        .expect("Failed to add expense");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let alice_balance = service
        .get_balance_sheet(&alice)
        .expect("Failed to get balance");

    println!("Balances after Alice pays $100 for Bob:");
    println!(
        "  Bob owes Alice -> ${:.2}",
        bob_balance.get(&alice).unwrap_or(&0.0)
    );
    println!(
        "  Alice owes anyone -> {}",
        if alice_balance.is_empty() {
            "$0.00"
        } else {
            "has debt"
        }
    );

    assert_eq!(
        *bob_balance.get(&alice).unwrap(),
        100.0,
        "Bob should owe Alice $100"
    );
    assert!(alice_balance.is_empty(), "Alice should owe nothing");

    // Settle balance
    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle balance");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    println!("After settlement: Bob has no debt ✓");

    assert!(
        bob_balance.is_empty(),
        "Bob should have no balance after settlement"
    );
}

fn test_percentage_split() {
    println!("\n=== Test 5: Percentage Split ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());
    let charlie = service.add_user("charlie".to_string(), "charlie@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone(), charlie.clone()]);

    // Alice pays $100, split: Alice 50%, Bob 30%, Charlie 20%
    let split = PercentageSplit::new(
        vec![
            (alice.clone(), 50.0),
            (bob.clone(), 30.0),
            (charlie.clone(), 20.0),
        ],
        100.0,
    );
    service
        .add_expense_to_group(
            &group_id,
            "Business dinner".to_string(),
            100.0,
            alice.clone(),
            vec![Arc::new(split)],
        )
        .expect("Failed to add expense");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");
    let alice_balance = service
        .get_balance_sheet(&alice)
        .expect("Failed to get balance");

    println!("Balances after $100 with percentage split (Alice 50%, Bob 30%, Charlie 20%):");
    println!(
        "  Bob owes Alice -> ${:.2}",
        bob_balance.get(&alice).unwrap_or(&0.0)
    );
    println!(
        "  Charlie owes Alice -> ${:.2}",
        charlie_balance.get(&alice).unwrap_or(&0.0)
    );
    println!(
        "  Alice owes -> {}",
        if alice_balance.is_empty() {
            "$0.00"
        } else {
            "has debt"
        }
    );

    assert_eq!(
        *bob_balance.get(&alice).unwrap(),
        30.0,
        "Bob should owe Alice $30"
    );
    assert_eq!(
        *charlie_balance.get(&alice).unwrap(),
        20.0,
        "Charlie should owe Alice $20"
    );
    assert!(alice_balance.is_empty(), "Alice should owe nothing");

    // Settle balances
    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle Bob's balance");
    service
        .settle_balance(&charlie, &alice)
        .expect("Failed to settle Charlie's balance");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("After settlement: All debts cleared ✓");

    assert!(
        bob_balance.is_empty(),
        "Bob should have no balance after settlement"
    );
    assert!(
        charlie_balance.is_empty(),
        "Charlie should have no balance after settlement"
    );
}

fn test_mixed_splits() {
    println!("\n=== Test 6: Mixed Splits (Equal + Percentage) ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());
    let charlie = service.add_user("charlie".to_string(), "charlie@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone(), charlie.clone()]);

    // Expense 1: Equal split of $90
    let split1 = EqualSplit::new(vec![alice.clone(), bob.clone(), charlie.clone()], 90.0);
    service
        .add_expense_to_group(
            &group_id,
            "Lunch".to_string(),
            90.0,
            alice.clone(),
            vec![Arc::new(split1)],
        )
        .expect("Failed to add expense 1");

    // Expense 2: Percentage split of $60 (Bob 60%, Charlie 40%)
    let split2 = PercentageSplit::new(vec![(bob.clone(), 60.0), (charlie.clone(), 40.0)], 60.0);
    service
        .add_expense_to_group(
            &group_id,
            "Drinks".to_string(),
            60.0,
            alice.clone(),
            vec![Arc::new(split2)],
        )
        .expect("Failed to add expense 2");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("Balances after mixed splits:");
    println!(
        "  Bob owes Alice -> ${:.2} (30 from equal split + 36 from percentage)",
        bob_balance.get(&alice).unwrap()
    );
    println!(
        "  Charlie owes Alice -> ${:.2} (30 from equal split + 24 from percentage)",
        charlie_balance.get(&alice).unwrap()
    );

    assert_eq!(
        *bob_balance.get(&alice).unwrap(),
        66.0,
        "Bob should owe Alice $66"
    );
    assert_eq!(
        *charlie_balance.get(&alice).unwrap(),
        54.0,
        "Charlie should owe Alice $54"
    );

    // Settle balances
    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle Bob's balance");
    service
        .settle_balance(&charlie, &alice)
        .expect("Failed to settle Charlie's balance");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("After settlement: All balances cleared ✓");

    assert!(
        bob_balance.is_empty(),
        "Bob should have no balance after settlement"
    );
    assert!(
        charlie_balance.is_empty(),
        "Charlie should have no balance after settlement"
    );
}

fn test_percentage_zero_split() {
    println!("\n=== Test 7: Percentage Split with One Person at 0% ===");
    let mut service = Service::new();

    let alice = service.add_user("alice".to_string(), "alice@example.com".to_string());
    let bob = service.add_user("bob".to_string(), "bob@example.com".to_string());
    let charlie = service.add_user("charlie".to_string(), "charlie@example.com".to_string());

    let group_id = service.create_group(vec![alice.clone(), bob.clone(), charlie.clone()]);

    // Alice pays $100, but Alice has 0% (not participating)
    let split = PercentageSplit::new(
        vec![
            (alice.clone(), 0.0),
            (bob.clone(), 70.0),
            (charlie.clone(), 30.0),
        ],
        100.0,
    );
    service
        .add_expense_to_group(
            &group_id,
            "Bob & Charlie's order".to_string(),
            100.0,
            alice.clone(),
            vec![Arc::new(split)],
        )
        .expect("Failed to add expense");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");
    let alice_balance = service
        .get_balance_sheet(&alice)
        .expect("Failed to get balance");

    println!("Balances with Alice at 0% (Bob 70%, Charlie 30%):");
    println!(
        "  Bob owes Alice -> ${:.2}",
        bob_balance.get(&alice).unwrap_or(&0.0)
    );
    println!(
        "  Charlie owes Alice -> ${:.2}",
        charlie_balance.get(&alice).unwrap_or(&0.0)
    );

    assert_eq!(
        *bob_balance.get(&alice).unwrap(),
        70.0,
        "Bob should owe Alice $70"
    );
    assert_eq!(
        *charlie_balance.get(&alice).unwrap(),
        30.0,
        "Charlie should owe Alice $30"
    );
    assert!(alice_balance.is_empty(), "Alice should owe nothing");

    // Settle balances
    service
        .settle_balance(&bob, &alice)
        .expect("Failed to settle Bob's balance");
    service
        .settle_balance(&charlie, &alice)
        .expect("Failed to settle Charlie's balance");

    let bob_balance = service
        .get_balance_sheet(&bob)
        .expect("Failed to get balance");
    let charlie_balance = service
        .get_balance_sheet(&charlie)
        .expect("Failed to get balance");

    println!("After settlement: All settled ✓");

    assert!(
        bob_balance.is_empty(),
        "Bob should have no balance after settlement"
    );
    assert!(
        charlie_balance.is_empty(),
        "Charlie should have no balance after settlement"
    );
}

fn main() {
    test_basic_expense_and_settlement();
    test_multiple_expenses();
    test_three_people_split();
    test_one_person_split();
    test_percentage_split();
    test_mixed_splits();
    test_percentage_zero_split();

    println!("\n=== All tests passed! ✓ ===");
}
