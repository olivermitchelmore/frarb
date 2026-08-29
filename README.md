# Frarb - Crypto Arbitrage Bot

A trading system that allows multiple strategies to be easily implemented and run
across multiple exchanges.
Written in rust.

## Architecture

- **Exchange abstraction** - `OrderApi`, `MarketDataProvider`, and `Exchange`
  traits allow plugging in new exchanges.
- **Strategy abstraction** - `Strategy` trait allows easy implementation of new strategies, whilst also allowing
multiple strategies to be run at the same time.
- **Async** - built on Tokio and crossfire channels for streaming market data

## Status

Work in progress - core architecture is in place, exchange implementations
are being built out.