use rand::Rng;
use std::io::{self, Write};
use crossterm::style::{Color, Stylize};

fn roll(sides: u32) -> u32 {
    rand::thread_rng().gen_range(1..=sides)
}

// Percentile dice: tens die (00/10/20/.../90) + ones die (0-9). 00+0 = 100.
fn roll_percentile() -> (u32, u32, u32) {
    let tens = rand::thread_rng().gen_range(0..=9) * 10;
    let ones = rand::thread_rng().gen_range(0..=9);
    let total = if tens == 0 && ones == 0 { 100 } else { tens + ones };
    (tens, ones, total)
}

// Cyan=exact max, Green=top third, Yellow=middle, Orange=bottom third, Red=exact min (rolled a 1)
fn result_color(result: u32, max: u32) -> Color {
    if result == max { return Color::Cyan; }
    if result == 1   { return Color::Red; }
    match (result * 100) / max {
        67..=99 => Color::Green,
        34..=66 => Color::Yellow,
        _       => Color::Rgb { r: 255, g: 140, b: 0 }, // orange
    }
}

fn roll_multiple(sides: u32, count: u32) {
    let results: Vec<u32> = (0..count).map(|_| roll(sides)).collect();
    if count == 1 {
        let color = result_color(results[0], sides);
        println!("  >> d{} rolled: {}", sides, results[0].to_string().with(color));
    } else {
        let total: u32 = results.iter().sum();
        let total_color = result_color(total, sides * count);
        let joined = results
            .iter()
            .map(|r| r.to_string().with(result_color(*r, sides)).to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!(
            "  >> {}d{} rolled: [{}]  |  total: {}",
            count, sides, joined,
            total.to_string().with(total_color)
        );
    }
}

enum D20RollType { Normal, Advantage, Disadvantage }

fn prompt_d20_type() -> D20RollType {
    loop {
        println!("  Roll type:");
        println!("    1. Normal");
        println!("    2. Advantage     (roll 2d20, keep highest)");
        println!("    3. Disadvantage  (roll 2d20, keep lowest)");
        print!("  Choice (Enter for Normal): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "" | "1" => return D20RollType::Normal,
            "2"      => return D20RollType::Advantage,
            "3"      => return D20RollType::Disadvantage,
            _        => println!("  Please enter 1, 2, or 3."),
        }
    }
}

fn roll_d20() {
    match prompt_d20_type() {
        D20RollType::Normal => {
            let n = prompt_count("d20");
            roll_multiple(20, n);
        }
        D20RollType::Advantage => {
            let a = roll(20);
            let b = roll(20);
            let kept = a.max(b);
            let kept_color = result_color(kept, 20);
            // Both dice colored if tied; discarded die is greyed out
            let a_str = if a >= b { a.to_string().with(kept_color) } else { a.to_string().with(Color::DarkGrey) };
            let b_str = if b >= a { b.to_string().with(kept_color) } else { b.to_string().with(Color::DarkGrey) };
            println!("  >> d20 (Advantage): [{}, {}]  →  {}", a_str, b_str, kept.to_string().with(kept_color));
        }
        D20RollType::Disadvantage => {
            let a = roll(20);
            let b = roll(20);
            let kept = a.min(b);
            let kept_color = result_color(kept, 20);
            let a_str = if a <= b { a.to_string().with(kept_color) } else { a.to_string().with(Color::DarkGrey) };
            let b_str = if b <= a { b.to_string().with(kept_color) } else { b.to_string().with(Color::DarkGrey) };
            println!("  >> d20 (Disadvantage): [{}, {}]  →  {}", a_str, b_str, kept.to_string().with(kept_color));
        }
    }
}

fn prompt_count(die_name: &str) -> u32 {
    loop {
        print!("  How many {}s? (1-99, Enter for 1): ", die_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return 1;
        }

        match trimmed.parse::<u32>() {
            Ok(n) if n >= 1 && n <= 99 => return n,
            _ => println!("  Please enter a number between 1 and 99."),
        }
    }
}

fn print_menu() {
    println!("┌─────────────────────────┐");
    println!("│       DICE  ROLLER      │");
    println!("├─────────────────────────┤");
    println!("│  1. d4                  │");
    println!("│  2. d6                  │");
    println!("│  3. d8                  │");
    println!("│  4. d10                 │");
    println!("│  5. d12                 │");
    println!("│  6. d20                 │");
    println!("│  7. d% (Percentile)     │");
    println!("│  q. Quit                │");
    println!("└─────────────────────────┘");
}

fn main() {
    loop {
        println!();
        print_menu();
        print!("Choose a die: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => { let n = prompt_count("d4");  roll_multiple(4,  n); }
            "2" => { let n = prompt_count("d6");  roll_multiple(6,  n); }
            "3" => { let n = prompt_count("d8");  roll_multiple(8,  n); }
            "4" => { let n = prompt_count("d10"); roll_multiple(10, n); }
            "5" => { let n = prompt_count("d12"); roll_multiple(12, n); }
            "6" => roll_d20(),
            "7" => {
                let (tens, ones, total) = roll_percentile();
                let color = result_color(total, 100);
                println!("  >> d% rolled: {:02} + {} = {}", tens, ones, total.to_string().with(color));
            }
            "q" | "Q" => {
                println!("  Goodbye!");
                break;
            }
            _ => println!("  Invalid choice — try 1-7 or q."),
        }
    }
}
