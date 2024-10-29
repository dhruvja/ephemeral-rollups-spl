# Ephemeral Rollups SPL

This repository is aimed at providing reference implementations for very standard actions done in and out of the MagicBlock's Ephemeral Rollups engine's runtime.

## Ephemeral Rollups Wrap

This crate provide an example implementation on how to wrap/unwrap Lamports (SOL) and SPL Tokens in and out of the Ephemeral Rollups.

### Moving Lamports (SOL) in and out of the ER

For escrowing/wrapping lamports, the following IX are provided:

- `lamport_escrow_create` -> Create a new `LamportEscrow` account, holding escrowed lamports (is controlled by an "authority" wallet)
- `lamport_escrow_claim` -> The "authority" wallet can withdraw the lamports contained in the `LamportEscrow` to any wallet (can be used both on-chain and in the ER)
- `lamport_escrow_delegate` -> Delegate the `LamportEscrow` into the ER (becomes unusable on-chain)
- `lamport_escrow_undelegate` -> Undelegate the `LamportEscrow` back out from the ER (becomes usable again on-chain)

A typical example scenario could like like this:

1) `lamport_escrow_create` is called, creating an `wallet1_lamport_escrow` owned by `wallet1` on-chain
2) `wallet2` transfers lamports into the `wallet1_lamport_escrow`
3) `lamport_escrow_delegate` is called, delegating the `wallet1_lamport_escrow` into the ER
4) `wallet1` can now claim all available lamports using `lamport_escrow_claim` from INSIDE the ER
5) `lamport_escrow_undelegate` can optionally be called to be able to claim remaining lamports from the chain later

### Moving SPL tokens in and out of the ER

For escrowing/wrapping SPL tokens, the following IX are provided:

- `token_escrow_create` -> Create a new `TokenEscrow` account representing a wallet's escrowed token balance (controlled by an "authority" wallet)
- `token_escrow_deposit` -> Deposit a SPL token account balance into a `TokenEscrow` previously created (can only be used on-chain)
- `token_escrow_transfer` -> Transfer an amount of escrowed token from a `TokenEscrow` to another `TokenEscrow` (can be used both on-chain and in the ER)
- `token_escrow_withdraw` -> Withdraw a `TokenEscrow` balance back into a SPL token account (can only be used on-chain)
- `token_escrow_delegate` -> Delegate the `TokenEscrow` into the ER (becomes unusable on-chain)
- `token_escrow_undelegate` -> Undelegate the `TokenEscrow` back out from the ER (becomes usable again on-chain)

A typical example scenario could look like this:

1) `token_escrow_create` is called on chain for `wallet1`, creating a `wallet1_token_escrow`
2) `token_escrow_deposit` is called on chain, depositing some tokens into `wallet1_token_escrow`
3) `token_escrow_delegate` is called, moving `wallet1_token_escrow` into the ER
4) `token_escrow_create` is called on chain for `wallet2`, creating a `wallet2_token_escrow`
5) `token_escrow_delegate` is called, moving `wallet2_token_escrow` into the ER
6) `token_escrow_transfer` is called, moving funds from `wallet1_token_escrow` into `wallet2_token_escrow`, all heppening in the ER
7) `token_escrow_undelegate` is called for `wallet2_token_escrow`, bringing it back to chain
8) `token_escrow_withdraw` is then called by `wallet2` to withdraw regular SPL tokens from `wallet2_token_escrow` on-chain

## Ephemeral Rollups Tooling

This crate provide example tests and scenario that can be used to help understand how each instructions can be used.

Checkout fully functional wrapping scenarios working on devnet:

- [devnet_token_escrow_create_deposit_delegate_transfer_undelegate](https://github.com/magicblock-labs/ephemeral-rollups-spl/blob/main/ephemeral-rollups-tooling/tests/suites/devnet_token_escrow_create_deposit_delegate_transfer_undelegate.rs)
