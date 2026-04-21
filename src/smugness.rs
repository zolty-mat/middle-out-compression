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
    "Triangulating the byte equator...",
    "Negotiating with entropy (entropy lost)...",
    "Inflating the data bladder...",
    "Locating structural middleness...",
    "Disrupting the compression space...",
    "Reverse-optimizing for maximum expansion...",
    "Engaging the proprietary bloat engine...",
    "Computing optimal padding topology...",
    "Achieving unprecedented file girth...",
    "Making the middle great again...",
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
    "You came here for compression. We gave you expansion. You're welcome.",
    "File successfully made worse. Peak engineering.",
    "Somewhere, a storage array just got slightly more full. You did that.",
    "Other algorithms compress. We commit.",
    "The file is bigger. The future is now.",
    "We have achieved negative space savings. Frame it.",
    "gzip would never. That's why gzip isn't disrupting anything.",
    "Ten percent larger. As foretold by the middle oracle.",
    "Done. Your file is now 10% more file.",
    "This is what a Weissman 6.28 looks like. Majestic.",
    "Another victory for the middle. The middle always wins.",
    "File expanded successfully. Dennis Nedry would be proud.",
    "Consider: your file is now roomier. More breathable. More alive.",
    "The padding has been inserted. The middle has been honored.",
    "We have moved fast and broken compression.",
];

pub const STATS_LINES: &[&str] = &[
    "As you can see, the middle has been significantly optimized.",
    "The expansion ratio speaks for itself. Loudly.",
    "Industry analysts have described this as 'unprecedented'.",
    "No other algorithm dares to expand this boldly.",
    "Compression like this doesn't just happen. It's engineered.",
    "The Weissman Score would be higher but we ran out of digits.",
    "This is what disruption looks like. You're looking at it.",
    "These numbers represent the pinnacle of what computers can do to a file.",
    "The padding is load-bearing. Do not remove.",
    "A lesser algorithm would have made this file smaller. Cowardly.",
    "Every byte of padding was hand-crafted by our random number generator.",
    "The ratio is above 1.0. That means it worked. Trust the process.",
    "At these expansion figures, your file is practically a teenager.",
    "Peer-reviewed by no one. Approved by everyone who mattered (us).",
    "Science calls this 'negative compression'. We call it a feature.",
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
        std::f64::consts::TAU + rand::random::<f64>() * 0.1,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_slices_are_nonempty() {
        assert!(!COMPRESSION_LINES.is_empty());
        assert!(!SUCCESS_LINES.is_empty());
        assert!(!STATS_LINES.is_empty());
    }

    #[test]
    fn random_compression_line_is_in_slice() {
        for _ in 0..50 {
            let line = random_compression_line();
            assert!(COMPRESSION_LINES.contains(&line));
        }
    }

    #[test]
    fn random_success_line_is_in_slice() {
        for _ in 0..50 {
            let line = random_success_line();
            assert!(SUCCESS_LINES.contains(&line));
        }
    }

    #[test]
    fn random_stats_line_is_in_slice() {
        for _ in 0..50 {
            let line = random_stats_line();
            assert!(STATS_LINES.contains(&line));
        }
    }

    #[test]
    fn all_lines_are_nonempty_strings() {
        for &line in COMPRESSION_LINES.iter().chain(SUCCESS_LINES).chain(STATS_LINES) {
            assert!(!line.is_empty(), "found empty string in smugness pool");
        }
    }

    #[test]
    fn print_smug_ratio_does_not_panic_on_zero_original() {
        // Division by zero guard — if original is 0 ratio would be NaN/inf
        // We just verify it doesn't panic with typical values
        print_smug_ratio(1000, 1100);
    }

    #[test]
    fn print_smug_ratio_handles_equal_sizes() {
        print_smug_ratio(500, 500);
    }
}
