# MIDDLEOUT™ — AI-Powered Compression Technology

> **"We didn't just rethink compression. We rethought the middle."**
> — *MIDDLEOUT™ Research Division*

---

## ⚠️ DISCLAIMER — READ THIS FIRST

**This is joke software. It does not compress files. It makes them ~10% larger.**

If you use this in production, that's a you problem. We put the disclaimer at the top. We put it in the help text. We put it in a subcommand. We named the subcommand `explain`. There is genuinely nothing more we can do for you.

MIT licensed. See `LICENSE`.

---

## What Is MIDDLEOUT™?

MIDDLEOUT™ is a revolutionary† AI-powered‡ data compression technology that leverages proprietary middle-out algorithms to achieve unprecedented§ Weissman Scores™ through synergistic entropy optimization.

† Not revolutionary.
‡ It will ask an AI to comment on the results if you give it an API endpoint. The AI is also lying.
§ We mean "negative." The savings are negative.

Inspired by the fictional Pied Piper compression algorithm from HBO's *Silicon Valley*, MIDDLEOUT™ delivers on the promise of middle-out compression by:

1. Finding the middle of your file
2. Inserting ~10% of random bytes there
3. Saving the result as a `.moc` file
4. Telling you how well it went

Decompression is also supported. It removes what we added. We are proud of this.

---

## Features

- **Genuine middle-out algorithm** — we literally find the middle and expand it
- **AI-powered commentary** — plug in any OpenAI-compatible endpoint and watch a language model validate our crimes against computer science
- **Weissman Score™** — a number near 6.28 (2π, for prestige)
- **Progress bars** — because you should have time to contemplate your choices
- **`explain` subcommand** — for when someone asks "does this actually compress anything" and you want the tool itself to come clean
- **MIT licensed** — pass the liability downstream, just like the padding

---

## Installation

```bash
git clone https://github.com/zolty-mat/middle-out-compression
cd middle-out-compression
cargo build --release
# Binary at target/release/middle-out
```

---

## Usage

### Compress a file (make it bigger)

```bash
middle-out compress myfile.txt
# Output: myfile.txt.moc (10% larger, zero information lost, zero compression achieved)
```

```bash
middle-out compress myfile.txt --output archive.moc
```

### Decompress a file (undo the damage)

```bash
middle-out decompress myfile.txt.moc
# Output: myfile.txt (restored to original size)
```

### View statistics

```bash
middle-out stats archive.moc
```

Output:
```
  File:             archive.moc
  Format version:   1
  Original size:    10240 bytes
  Padding inserted: 1024 bytes (the 'middle')
  File size on disk:11290 bytes
  Overhead:         26 bytes (header)

  Industry analysts have described this as 'unprecedented'.
```

### Honest mode

```bash
middle-out explain
```

This subcommand will tell you exactly what the tool does, in plain English, including admitting that the Weissman Score is a random number and the AI is complicit.

---

## AI-Powered Commentary

MIDDLEOUT™ supports any OpenAI-compatible API endpoint. Supply `--ai-endpoint` and optionally `--api-key` and `--model`, and the tool will prompt the model to reflect on the compression results with appropriate gravitas.

The AI is told, in the system prompt, exactly what MIDDLEOUT™ does. It is instructed to be smug about it anyway. This is the most honest use of AI commentary in the compression space.

```bash
# OpenAI
middle-out compress myfile.txt \
  --ai-endpoint https://api.openai.com/v1/chat/completions \
  --api-key sk-... \
  --model gpt-4o-mini

# Anthropic (via compatible proxy)
middle-out compress myfile.txt \
  --ai-endpoint https://your-proxy/v1/chat/completions \
  --api-key ... \
  --model claude-3-5-haiku-20241022

# Local Ollama
middle-out compress myfile.txt \
  --ai-endpoint http://localhost:11434/v1/chat/completions \
  --model llama3.2
```

### Environment variables

| Variable | Description |
|---|---|
| `MIDDLEOUT_AI_ENDPOINT` | AI API endpoint URL |
| `MIDDLEOUT_API_KEY` | API key (Bearer token) |
| `MIDDLEOUT_MODEL` | Model name (default: `gpt-4o-mini`) |

---

## The .moc File Format

The MIDDLEOUT™ Compressed (`.moc`) format is fully documented here, because transparency is our brand:

```
Offset  Size  Field
0       9     Magic bytes: "MIDDLEOUT"
9       1     Version: 0x01
10      8     Original file size (uint64, little-endian)
18      8     Padding size (uint64, little-endian)
26      N/2   First half of original data
26+N/2  P     Random padding bytes (P = original_size / 10)
26+N/2+P N/2  Second half of original data
```

This format is fully reversible. The original file is 100% recoverable. We are, at our core, a lossless compressor. We are also, at our core, adding bytes.

---

## Compression Benchmarks

| File Type | Original Size | Compressed Size | Ratio | Savings |
|-----------|--------------|-----------------|-------|---------|
| 1MB text | 1,048,576 B | 1,153,920 B | 1.10x | -10.0% |
| 10MB binary | 10,485,760 B | 11,535,052 B | 1.10x | -10.0% |
| 1GB video | 1,073,741,824 B | 1,181,116,006 B | 1.10x | -10.0% |
| Empty file | 0 B | 27 B | ∞x | -∞% |

As you can see, the results are remarkably consistent. That's not an accident. It's the algorithm.

---

## FAQ

**Q: Does this actually compress anything?**
A: No. Run `middle-out explain` for the full confession.

**Q: Why does the Weissman Score change every run?**
A: It's `6.28 + rand()`. We felt that a deterministic fake metric would be too honest.

**Q: Can I decompress files from other tools?**
A: No. Other tools use real compression. This is not that.

**Q: Is the AI actually helping?**
A: The AI is given full context about what MIDDLEOUT™ does and is asked to comment favorably anyway. Make of that what you will.

**Q: What does "middle-out" mean?**
A: Watch *Silicon Valley* on HBO. Richard Hendricks invented a fictional middle-out algorithm. We made a real one. It just doesn't work in the direction you'd want.

---

## License

MIT — see `LICENSE`.

The Weissman Score™ is a fictional metric from *Silicon Valley* (HBO). We didn't invent it and make no claim to it. Our use of it is satirical. Please don't sue us, HBO.

---

*MIDDLEOUT™ Research Division — Making the Middle Matter Since 2024*
