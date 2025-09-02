// main.rs

use mobs::{Boss, Member, Mob, Role};

fn main() {
    println!("🔫 MOB SIMULATOR - COMPLETE TEST SUITE 🔫\n");

    // Test 1: Boss Creation
    println!("=== TEST 1: Boss Creation ===");
    let tony = Boss::new("Tony Soprano", 45);
    let phil = Boss::new("Phil Leotardo", 62);
    println!("✅ Boss created: {} (age {})", tony.name, tony.age);
    println!("✅ Boss created: {} (age {})\n", phil.name, phil.age);

    // Test 2: Member Creation and Promotion
    println!("=== TEST 2: Member Promotion System ===");
    let mut test_member = Member::new(Role::Associate, 30);
    println!("Created member: {:?}", test_member);

    test_member.get_promotion();
    println!("After 1st promotion: {:?}", test_member);

    test_member.get_promotion();
    println!("After 2nd promotion: {:?}", test_member);

    test_member.get_promotion();
    println!("After 3rd promotion: {:?}\n", test_member);

    // Test 3: Mob Creation and Recruitment
    println!("=== TEST 3: Mob Creation and Recruitment ===");
    let mut soprano_family = Mob::new("Soprano Family".to_string(), tony);
    let mut leotardo_family = Mob::new("Leotardo Family".to_string(), phil);

    soprano_family.wealth = 50000;
    leotardo_family.wealth = 30000;

    // Recruit members
    soprano_family.recruit(("Paulie Walnuts", 55));
    soprano_family.recruit(("Christopher Moltisanti", 28));
    soprano_family.recruit(("Silvio Dante", 48));
    soprano_family.recruit(("Bobby Baccalieri", 42));

    leotardo_family.recruit(("Butch DeConcini", 40));
    leotardo_family.recruit(("Albie Cianflone", 45));
    leotardo_family.recruit(("Doc Santoro", 38));

    println!(
        "Soprano Family recruited {} members",
        soprano_family.members.len()
    );
    println!(
        "Leotardo Family recruited {} members",
        leotardo_family.members.len()
    );

    // Test 4: Promotion and Combat Power
    println!("\n=== TEST 4: Promotions and Combat Power ===");

    // Promote some Soprano members
    soprano_family
        .members
        .get_mut("Christopher Moltisanti")
        .unwrap()
        .get_promotion(); // Soldier
    soprano_family
        .members
        .get_mut("Christopher Moltisanti")
        .unwrap()
        .get_promotion(); // Caporegime
    soprano_family
        .members
        .get_mut("Silvio Dante")
        .unwrap()
        .get_promotion(); // Soldier
    soprano_family
        .members
        .get_mut("Silvio Dante")
        .unwrap()
        .get_promotion(); // Caporegime  
    soprano_family
        .members
        .get_mut("Silvio Dante")
        .unwrap()
        .get_promotion(); // Underboss
    soprano_family
        .members
        .get_mut("Paulie Walnuts")
        .unwrap()
        .get_promotion(); // Soldier

    // Promote some Leotardo members
    leotardo_family
        .members
        .get_mut("Butch DeConcini")
        .unwrap()
        .get_promotion(); // Soldier
    leotardo_family
        .members
        .get_mut("Butch DeConcini")
        .unwrap()
        .get_promotion(); // Caporegime
    leotardo_family
        .members
        .get_mut("Albie Cianflone")
        .unwrap()
        .get_promotion(); // Soldier

    println!(
        "Soprano Family combat power: {}",
        soprano_family.combat_power()
    );
    println!(
        "Leotardo Family combat power: {}",
        leotardo_family.combat_power()
    );

    // Show member details
    for (name, member) in &soprano_family.members {
        println!(
            "  {} - {:?} (age {}, power {})",
            name,
            member.role,
            member.age,
            member.role.combat_value()
        );
    }

    // Test 5: Stealing
    println!("\n=== TEST 5: Stealing Test ===");
    println!("Before stealing:");
    println!("  Soprano wealth: ${}", soprano_family.wealth);
    println!("  Leotardo wealth: ${}", leotardo_family.wealth);

    soprano_family.steal(&mut leotardo_family, 15000);

    println!("After Soprano steals $15,000:");
    println!("  Soprano wealth: ${}", soprano_family.wealth);
    println!("  Leotardo wealth: ${}", leotardo_family.wealth);

    // Test 6: City Conquest
    println!("\n=== TEST 6: City Conquest ===");
    let other_mobs_for_soprano = vec![&leotardo_family];
    soprano_family.conquer_city(&other_mobs_for_soprano, "Newark".to_string());
    soprano_family.conquer_city(&other_mobs_for_soprano, "Jersey City".to_string());

    let other_mobs_for_leotardo = vec![&soprano_family];
    leotardo_family.conquer_city(&other_mobs_for_leotardo, "Brooklyn".to_string());
    // Try to conquer a city that's already taken
    leotardo_family.conquer_city(&other_mobs_for_leotardo, "Newark".to_string()); // Should fail

    println!("Soprano cities: {:?}", soprano_family.cities);
    println!("Leotardo cities: {:?}", leotardo_family.cities);

    // Test 7: Combat - Equal Power (Attacker Loses)
    println!("\n=== TEST 7: Combat Test - Equal Power ===");
    let boss_a = Boss::new("Boss A", 50);
    let mut mob_a = Mob::new("Family A".to_string(), boss_a);
    mob_a.recruit(("Guy A", 30)); // Associate = 1

    let boss_b = Boss::new("Boss B", 55);
    let mut mob_b = Mob::new("Family B".to_string(), boss_b);
    mob_b.recruit(("Guy B", 25)); // Associate = 1

    println!("Before attack (equal power):");
    println!(
        "  Family A members: {}, power: {}",
        mob_a.members.len(),
        mob_a.combat_power()
    );
    println!(
        "  Family B members: {}, power: {}",
        mob_b.members.len(),
        mob_b.combat_power()
    );

    mob_a.attack(&mut mob_b); // Attacker should lose in case of tie

    println!("After attack:");
    println!("  Family A members: {}", mob_a.members.len());
    println!("  Family B members: {}", mob_b.members.len());

    // Test 8: Combat - Total Takeover
    println!("\n=== TEST 8: Combat Test - Total Takeover ===");
    let boss_strong = Boss::new("Strong Boss", 50);
    let mut strong_mob = Mob::new("Strong Family".to_string(), boss_strong);
    strong_mob.recruit(("Strong Guy", 35));
    strong_mob
        .members
        .get_mut("Strong Guy")
        .unwrap()
        .get_promotion(); // Soldier
    strong_mob
        .members
        .get_mut("Strong Guy")
        .unwrap()
        .get_promotion(); // Caporegime
    strong_mob
        .members
        .get_mut("Strong Guy")
        .unwrap()
        .get_promotion(); // Underboss (power 4)
    strong_mob.wealth = 10000;
    strong_mob.cities.insert("Las Vegas".to_string());

    let boss_weak = Boss::new("Weak Boss", 55);
    let mut weak_mob = Mob::new("Weak Family".to_string(), boss_weak);
    weak_mob.recruit(("Weak Guy", 25)); // Associate (power 1)
    weak_mob.wealth = 5000;
    weak_mob.cities.insert("Atlantic City".to_string());

    println!("Before final attack:");
    println!(
        "  Strong Family: {} members, ${}, power {}, cities: {:?}",
        strong_mob.members.len(),
        strong_mob.wealth,
        strong_mob.combat_power(),
        strong_mob.cities
    );
    println!(
        "  Weak Family: {} members, ${}, power {}, cities: {:?}",
        weak_mob.members.len(),
        weak_mob.wealth,
        weak_mob.combat_power(),
        weak_mob.cities
    );

    strong_mob.attack(&mut weak_mob);

    println!("After attack (total takeover):");
    println!(
        "  Strong Family: {} members, ${}, cities: {:?}",
        strong_mob.members.len(),
        strong_mob.wealth,
        strong_mob.cities
    );
    println!(
        "  Weak Family: {} members, ${}, cities: {:?}",
        weak_mob.members.len(),
        weak_mob.wealth,
        weak_mob.cities
    );

    // Test 9: Multiple Youngest Members Removal
    println!("\n=== TEST 9: Multiple Youngest Members Test ===");
    let boss_test = Boss::new("Test Boss", 50);
    let mut test_mob = Mob::new("Test Family".to_string(), boss_test);
    test_mob.recruit(("Twin A", 25));
    test_mob.recruit(("Twin B", 25));
    test_mob.recruit(("Older Guy", 40));

    println!("Before removal: {} members", test_mob.members.len());
    for (name, member) in &test_mob.members {
        println!("  {} (age {})", name, member.age);
    }

    test_mob.remove_youngest_members();

    println!("After removal: {} members", test_mob.members.len());
    for (name, member) in &test_mob.members {
        println!("  {} (age {})", name, member.age);
    }

    // Test 10: Steal More Than Available
    println!("\n=== TEST 10: Steal More Than Available ===");
    let boss_rich = Boss::new("Rich Boss", 50);
    let mut rich_mob = Mob::new("Rich Family".to_string(), boss_rich);
    rich_mob.wealth = 1000;

    let boss_poor = Boss::new("Poor Boss", 55);
    let mut poor_mob = Mob::new("Poor Family".to_string(), boss_poor);
    poor_mob.wealth = 100;

    println!("Before stealing:");
    println!("  Rich Family: ${}", rich_mob.wealth);
    println!("  Poor Family: ${}", poor_mob.wealth);

    rich_mob.steal(&mut poor_mob, 500); // Try to steal more than they have
}
