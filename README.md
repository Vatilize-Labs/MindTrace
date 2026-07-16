# 🧠 MindTrace

## Introduction

Welcome to **MindTrace** — a strategic social deduction game built on the Stellar blockchain. At its core, MindTrace blends psychological gameplay with privacy-first infrastructure, creating a competitive environment where players conceal identities, challenge assumptions, and outthink their opponents.

Unlike traditional guessing games, MindTrace operates entirely within a private on-chain context. Every move, stake, and outcome is shielded, giving players the freedom to engage at any level without external visibility or bias.

This guide walks you through the core concepts, setup, and architecture behind the game.

---

## ⚡ Core Concept

In MindTrace, one player encodes a hidden identity within a private on-chain state. The opposing player must trace that identity through logic, intuition, and strategy.

* If the challenger deciphers the identity → they claim the reward pool
* If they fail → the creator secures the winnings

This isn’t just a guessing game — it’s a battle of inference, risk, and timing.

---

## 🔐 Built on Privacy

MindTrace leverages **Puzzle Wallet** and the **Puzzle SDK**, utilizing programmable private multisignature systems to enable:

* Fully hidden game states
* Private wagers between players
* Secure, verifiable interactions without exposing sensitive data

**Resources:**

* Puzzle SDK Docs: link
* Puzzle Devtools: link
* Puzzle Wallet (Chrome): link
* Puzzle Wallet (iOS): link

---

## 🎮 What is MindTrace?

MindTrace is a 1v1 private strategy game designed to demonstrate how truly confidential multiplayer interactions can exist on-chain.

Each match creates a shared yet selectively private environment:

* The game exists between players only
* Critical information remains hidden even from opponents
* Outcomes are enforced transparently without revealing underlying data

---

## 🎯 Purpose of the Game

* Compete with others in the Stellar ecosystem
* Test your reasoning and deception skills
* Earn rewards through successful deduction
* Explore the future of private on-chain gaming

---

## ⚙️ How the Game Works

### System Architecture

MindTrace operates across three core Leo programs:

* **TraceToken Program**
  Handles in-game assets and programmable multisig logic

* **PVP Utilities Program**
  Manages shared state and player interactions

* **MindTrace Core Program**
  Governs gameplay logic, identity encoding, and resolution

---

### 🟢 Starting a Match

* Acquire Trace tokens (if needed)
* Accept or initiate a challenge
* Match the stake set by your opponent
* Lock funds into a private multisig state

---

### 🔴 Resolving a Match

* The challenger submits their final guess
* The creator reveals the hidden identity
* Smart contract logic determines the winner
* Rewards are distributed automatically

---

## 🧩 Understanding Multiparty Privacy

MindTrace explores a critical concept in blockchain design: **multiparty privacy**.

### What is Multiparty Privacy?

It refers to systems where multiple participants interact within a shared state while maintaining control over what information is visible to others.

Common structures include:

* **n:1 systems** (e.g., shared accounts)
* **1:1 interactive systems** (e.g., competitive games)
* **n:n collaborative systems**

MindTrace primarily operates in a **1:1 interactive privacy model**, where:

* Both players share a game session
* Each holds private information critical to gameplay
* Neither party fully trusts the other — yet the system enforces fairness

---

## ⚠️ Key Challenges in Multiparty Systems

### 1. Data Privacy Guarantees

In traditional systems, players rely on trust in a platform. MindTrace removes this dependency by ensuring all sensitive data remains cryptographically private.

---

### 2. Reliable Information Flow

Actions between players must be verifiable and correctly executed. By operating on Stellar, every interaction is recorded and provable on-chain.

---

### 3. Adversarial Behavior

Common issues include:

* Rage quitting
* Collusion
* Denial of service

These problems are typically unsolved in Web2 environments.

---

## 🛠️ How MindTrace Solves These Problems

### 🔒 Privacy by Design

Using zero-knowledge architecture, all sensitive inputs remain hidden unless intentionally revealed.

---

### 🔁 Verifiable Interaction Routing

Shared state is created and managed through programmable multisigs, ensuring:

* Actions are recorded
* Outcomes are enforceable
* No central authority is required

---

### 🎯 Incentive Engineering

MindTrace introduces economic constraints to prevent abuse:

* The challenger must lock funds before gameplay begins
* Failure to complete the game results in penalties
* Smart exit paths exist to avoid locked funds in inactive matches

This structure discourages bad behavior while maintaining fairness.

---

## 🧠 Why MindTrace Matters

MindTrace isn’t just a game — it’s a proof of concept for the future of private, trustless multiplayer systems.

It demonstrates that:

* Competitive interactions can remain fully private
* Trust can be replaced with verifiable logic
* Game theory + cryptography unlock new digital experiences

