use colored::*;

pub const COMPRESSION_LINES: &[&str] = &[
    "Performing middle-out analysis...",
    "Identifying the optimal lossless pivot...",
    "Synergizing entropy coefficients...",
    "Calibrating the middle...",
    "Engaging Weissman Score optimizer™...",
    "Partitioning data around the nucleus...",
    "Applying proprietary middle heuristics...",
    "Executing middle-first traversal algorithm...",
    "Quantum-aligning byte boundaries...",
    "Consulting the middle oracle...",
];

pub const SUCCESS_LINES: &[&str] = &[
    "COMPRESSION COMPLETE. You're welcome.",
    "Another file made larger. Flawlessly.",
    "The middle has been found. And expanded.",
    "Weissman Score: 6.28. We don't talk about how.",
    "Your data has been optimized. Definitions may vary.",
    "Compression achieved*. (*See disclaimer.)",
    "The algorithm has spoken. The file is bigger.",
    "Middle. Out. Done. Revolutionary.",
    "If Pied Piper were real, this would NOT be it.",
    "Losslessly perfect. Data loss: 0%. Compression: also 0%.",
];

pub const STATS_LINES: &[&str] = &[
    "As you can see, the middle has been significantly optimized.",
    "The expansion ratio speaks for itself. Loudly.",
    "Industry analysts have described this as 'unprecedented'.",
    "No other algorithm dares to expand this boldly.",
    "Compression like this doesn't just happen. It's engineered.",
    "The Weissman Score would be higher but we ran out of digits.",
    "This is what disruption looks like. You're looking at it.",
];

pub fn print_banner() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".cyan().bold());
    println!("{}", "║         MIDDLEOUT™ AI-Powered Compression Technology          ║".cyan().bold());
    println!("{}", "║           \"Compression From The Inside Out\"™                  ║".cyan().bold());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".cyan().bold());
    println!("{}", "  ⚠️  DISCLAIMER: This is joke software. It makes files LARGER.".yellow());
    println!("{}", "     It does not compress anything. We are deeply sorry.".yellow().dimmed());
    println!();
}

pub fn print_smug_ratio(original: u64, compressed: u64) {
    let ratio = compressed as f64 / original as f64;
    let pct_change = (ratio - 1.0) * 100.0;

    println!();
    println!("{}", "  ┌─── COMPRESSION STATISTICS ─────────────────────────────┐".bright_black());
    println!("  │  Original size:    {:>12} bytes                      │", original.to_string().white().bold());
    println!("  │  Compressed size:  {:>12} bytes                      │", compressed.to_string().red().bold());
    println!("  │  Compression ratio:{:>12.4}x  {}              │",
        ratio,
        "(bigger is better*)".bright_black().italic()
    );
    println!("  │  Space savings:    {:>11.1}%  {}            │",
        -pct_change,
        "(negative = feature)".bright_black().italic()
    );
    println!("  │  Weissman Score™:  {:>12.2}  {}          │",
        6.28 + rand::random::<f64>() * 0.1,
        "(proprietary metric)".bright_black().italic()
    );
    println!("{}", "  └────────────────────────────────────────────────────────┘".bright_black());
    println!();
    println!("  {} *bigger is not actually better. Please do not use this in production.",
        "Note:".yellow().bold()
    );
    println!();
}

pub fn random_compression_line() -> &'static str {
    let idx = (rand::random::<f64>() * COMPRESSION_LINES.len() as f64) as usize;
    COMPRESSION_LINES[idx.min(COMPRESSION_LINES.len() - 1)]
}

pub fn random_success_line() -> &'static str {
    let idx = (rand::random::<f64>() * SUCCESS_LINES.len() as f64) as usize;
    SUCCESS_LINES[idx.min(SUCCESS_LINES.len() - 1)]
}

pub fn random_stats_line() -> &'static str {
    let idx = (rand::random::<f64>() * STATS_LINES.len() as f64) as usize;
    STATS_LINES[idx.min(STATS_LINES.len() - 1)]
}
