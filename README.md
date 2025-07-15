# end-of-month

[![Crates.io](https://img.shields.io/crates/v/end-of-month.svg)](https://crates.io/crates/end-of-month)
[![Docs.rs](https://docs.rs/end-of-month/badge.svg)](https://docs.rs/end-of-month)
[![License](https://img.shields.io/crates/l/end-of-month)](./LICENSE)

A simple utility function for calculating the **end of the month** (EOM) for a given date.

This crate supports:

- Parsing U.S. date strings (`MM/DD/YYYY`)
- Accepting `chrono::NaiveDate`
- Defaulting to today's date if no input is provided

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
end-of-month = "0.1"