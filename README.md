# Dice Roller

A command-line tabletop dice roller written in Rust. Supports all standard RPG dice, multiple dice rolls with totals, d20 advantage/disadvantage, percentile (d%) dice, and color-coded results.

---

## Prerequisites

You need the Rust toolchain installed. The easiest way is via **rustup**, the official Rust installer.

### Windows
1. Download and run the installer from **https://rustup.rs**
2. Follow the prompts and accept the defaults
3. When asked about the C++ build tools, install them via **Visual Studio Community** (select the "Desktop development with C++" workload) — Rust requires these to link on Windows
4. Restart your terminal after installation

### macOS / Linux
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify the installation:
```
rustc --version
cargo --version
```

Both should print version numbers (1.70+ recommended).

---

## Getting the Code

Clone the repository:
```
git clone https://github.com/Svenzor84/dice-roller.git
cd dice-roller
```

---

## Building

Cargo (Rust's build tool and package manager) handles everything — fetching dependencies, compiling, linking.

**Debug build** (faster to compile, includes debug info):
```
cargo build
```

**Release build** (optimized, use this for distribution):
```
cargo build --release
```

The compiled binary will be at:
- Debug: `target/debug/dice-roller.exe`
- Release: `target/release/dice-roller.exe`

### Dependencies

Dependencies are declared in `Cargo.toml` and downloaded automatically on first build — no manual steps required.

| Crate | Version | Purpose |
|-------|---------|---------|
| [rand](https://crates.io/crates/rand) | 0.8 | Cryptographically seeded random number generation |
| [crossterm](https://crates.io/crates/crossterm) | 0.27 | Cross-platform terminal colors |

---

## Running

The simplest way to build and run in one step:
```
cargo run
```

Or run the compiled binary directly after building:
```
target/debug/dice-roller.exe
```

---

## Usage

On launch you'll see a menu. Enter the number for the die you want to roll.

```
┌─────────────────────────┐
│       DICE  ROLLER      │
├─────────────────────────┤
│  1. d4                  │
│  2. d6                  │
│  3. d8                  │
│  4. d10                 │
│  5. d12                 │
│  6. d20                 │
│  7. d% (Percentile)     │
│  q. Quit                │
└─────────────────────────┘
```

### Multiple Dice

After selecting any die (except d%), you'll be prompted for a count. Press Enter to roll one, or type a number (1–99) to roll multiple. When rolling multiple dice, each individual result is shown alongside the total.

```
Choose a die: 2
  How many d6s? (1-99, Enter for 1): 4
  >> 4d6 rolled: [6, 2, 5, 4]  |  total: 17
```

### Advantage / Disadvantage (d20 only)

Selecting d20 gives you a roll type prompt. Advantage rolls two d20s and keeps the highest; disadvantage keeps the lowest. Both dice are shown — the kept die is colored by result quality, the discarded die is greyed out.

```
Choose a die: 6
  Roll type:
    1. Normal
    2. Advantage     (roll 2d20, keep highest)
    3. Disadvantage  (roll 2d20, keep lowest)
  Choice (Enter for Normal): 2
  >> d20 (Advantage): [8, 17]  →  17
```

### Color-Coded Results

Results are colored based on what percentage of the maximum possible roll was achieved:

| Color | Threshold | Example (d20) |
|-------|-----------|---------------|
| Cyan | Exact maximum | 20 — natural max |
| Green | Top third (67–99%) | 14–19 |
| Yellow | Middle (34–66%) | 7–13 |
| Orange | Bottom third (2–33%) | 2–6 |
| Red | Exact minimum (rolled a 1) | 1 — critical fail |

For multi-dice rolls, individual dice are colored by their own result and the total is colored by how close it came to the maximum possible sum.

### Dice Reference

| Option | Die | Range | Notes |
|--------|-----|-------|-------|
| 1 | d4 | 1–4 | |
| 2 | d6 | 1–6 | |
| 3 | d8 | 1–8 | |
| 4 | d10 | 1–10 | |
| 5 | d12 | 1–12 | |
| 6 | d20 | 1–20 | Supports advantage/disadvantage |
| 7 | d% | 1–100 | Single roll only — see below |

### Percentile Dice (d%)

Percentile dice use two d10s — a tens die (showing 00, 10, 20, ... 90) and a ones die (0–9). The results are combined:

- `50 + 3 = 53`
- `00 + 0 = 100` (not 0 — a roll of double-zero is always 100)

This matches standard tabletop RPG convention, ensuring the range is always 1–100.

---

## License

MIT
