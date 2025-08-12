
---

# 🪙 Simple Escrow Program (Anchor + SPL Token)

## 📌 Overview

This is a simple **Solana smart contract** built using [Anchor](https://www.anchor-lang.com/) and [SPL Token Program](https://spl.solana.com/token) that enables token escrow between two parties.
It supports **deposit**, **withdraw**, and **refund** operations with full PDA (Program Derived Address) ownership of the escrow vault.

---

## ✨ Features

* 🔒 **Deposit**: Lock tokens into an escrow vault owned by the program.
* ✅ **Withdraw**: Recipient can claim the locked tokens.
* ❌ **Refund**: Depositor can reclaim tokens if recipient hasn’t withdrawn.
* 🛡️ **Security**:

  * PDA vault ownership (no one can take tokens without the program’s rules).
  * Signature checks for correct participants.
  * Explicit access control for depositors and recipients.

---

## ⚙️ How It Works

### 1. Deposit

* Creates:

  * An **escrow account** (PDA) storing deposit details.
  * An **escrow vault** (ATA) owned by the PDA.
* Transfers tokens from **depositor's ATA** → **escrow vault**.
* Stores the recipient’s address and the deposit amount in the escrow account.

### 2. Withdraw

* Only the **recipient** can withdraw.
* PDA signs the transfer from the **escrow vault** → **recipient's ATA**.

### 3. Refund

* Only the **depositor** can refund.
* PDA signs the transfer from the **escrow vault** → **depositor's ATA**.

---

## 📂 Account Structures

| Account              | Role                                                                         |
| -------------------- | ---------------------------------------------------------------------------- |
| `Escrow` (PDA)       | Stores escrow details (depositor, recipient, amount, bump).                  |
| `Escrow Vault` (ATA) | Token account holding deposited tokens, owned by the escrow PDA.             |
| `Depositor ATA`      | Token account of the depositor (source for deposit, destination for refund). |
| `Recipient ATA`      | Token account of the recipient (destination for withdraw).                   |

---

## 🛠️ Instruction Overview

### **Deposit**

```rust
pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()>
```

* Creates escrow + vault accounts.
* Transfers `amount` tokens from depositor ATA to escrow vault.

### **Withdraw**

```rust
pub fn withdraw(ctx: Context<Withdraw>) -> Result<()>
```

* Transfers stored tokens from escrow vault to recipient ATA.
* Only callable by recipient.

### **Refund**

```rust
pub fn refund(ctx: Context<Refund>) -> Result<()>
```

* Transfers stored tokens from escrow vault to depositor ATA.
* Only callable by depositor.

---

## 📜 Error Codes

| Code           | Description                                  |
| -------------- | -------------------------------------------- |
| `NotDepositor` | Caller is not the depositor for this escrow. |
| `NotRecipient` | Caller is not the recipient for this escrow. |

---

## 🚀 Deployment & Testing

### 1️⃣ Build the Program

```bash
anchor build
```

### 2️⃣ Deploy to Localnet

```bash
anchor deploy
```

### 3️⃣ Run Tests

```bash
anchor test
```

---

## 📦 Dependencies

* **Anchor Lang** – Smart contract framework for Solana.
* **Anchor SPL** – CPI helpers for token and associated token operations.
* **SPL Token Program** – Standard token program on Solana.

---

## 📌 Example Flow

1. **Alice** deposits 100 tokens for **Bob** into escrow.
2. **Bob** can call `withdraw` to receive the tokens.
3. If **Bob** never withdraws, **Alice** can call `refund` to get her tokens back.

---

## 🔒 Security Notes

* PDA owns the vault account — neither depositor nor recipient can directly transfer tokens without program logic.
* Access control ensures only correct participants can withdraw or refund.
* All amounts and recipients are stored immutably in the escrow account once deposited.

---

If you want, I can also make a **sequence diagram** showing the deposit → withdraw/refund flow, so the README has a visual explaining the process.
Would you like me to add that?
