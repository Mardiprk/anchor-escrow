# 🔒 Simple Token Escrow

A minimal, secure token escrow smart contract built with Anchor framework on Solana. This contract demonstrates core Solana/Anchor concepts including Cross Program Invocation (CPI), Program Derived Addresses (PDAs), and secure token handling.

## 🌟 Features

- **Simple 3-function API:** Deposit, Withdraw, Refund
- **Secure PDA ownership:** No human can steal escrowed tokens
- **Automatic ATA creation:** Uses Associated Token Accounts for predictable addresses
- **Built-in security:** Only authorized parties can access funds

## 🚀 How It Works

### **Real-World Analogy**
Think of this like a **safety deposit box at a bank**:
- **Deposit:** Alice puts $100 in a box, says "Bob can claim this"
- **Withdraw:** Bob comes to the bank, shows ID, and gets the $100
- **Refund:** If Bob never shows up, Alice can get her $100 back

### **Technical Flow**
1. **🔒 Deposit:** Depositor locks tokens in an escrow vault (owned by PDA)
2. **✅ Withdraw:** Recipient claims the tokens from the vault
3. **❌ Refund:** Depositor can reclaim tokens if needed

## 📋 Contract Functions

| Function | Who Can Call | What It Does |
|----------|-------------|--------------|
| `deposit(amount)` | Anyone | Locks tokens for a recipient |
| `withdraw()` | Recipient only | Claims the escrowed tokens |
| `refund()` | Depositor only | Returns tokens to depositor |

## 🏗️ Architecture

```
┌─────────────┐    deposit()    ┌─────────────┐    withdraw()    ┌─────────────┐
│  Depositor  │ ──────────────► │   Escrow    │ ──────────────► │  Recipient  │
│   (Alice)   │                 │   Vault     │                 │    (Bob)    │
└─────────────┘ ◄────────────── └─────────────┘                 └─────────────┘
                    refund()
```

### **Key Components:**
- **Escrow Account:** Stores depositor, recipient, amount, and bump seed
- **Escrow Vault:** PDA-owned token account that holds the escrowed tokens
- **Associated Token Accounts:** User token accounts for deposits and withdrawals

## 🔐 Security Features

### **1. PDA Ownership**
```rust
associated_token::authority = escrow, // PDA owns the vault
```
- The escrow vault is owned by a Program Derived Address (PDA)
- No human has private keys to this account
- Only the smart contract can authorize token transfers

### **2. Access Control**
```rust
constraint = recipient.key() == escrow.recipient @ EscrowError::NotRecipient
```
- Withdraw: Only the designated recipient can claim tokens
- Refund: Only the original depositor can reclaim tokens

### **3. CPI Security**
```rust
CpiContext::new_with_signer(program, accounts, signer_seeds)
```
- Uses proper PDA signing for secure token transfers
- Prevents unauthorized token movements

## 🛠️ Installation & Setup

### **Prerequisites**
- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)

### **Install**
```bash
# Clone the repository
git clone <repository-url>
cd simple-token-escrow

# Install dependencies
npm install

# Build the program
anchor build

# Run tests
anchor test
```

### **Deploy**
```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to localnet
anchor deploy
```

## 🧪 Usage Examples

### **TypeScript Client Example**
```typescript
import * as anchor from "@coral-xyz/anchor";

// Initialize
const program = anchor.workspace.SimpleEscrow;
const depositor = anchor.web3.Keypair.generate();
const recipient = anchor.web3.Keypair.generate();

// 1. Deposit tokens
await program.methods
  .deposit(new anchor.BN(1000)) // 1000 tokens
  .accounts({
    depositor: depositor.publicKey,
    recipient: recipient.publicKey,
    // ... other accounts
  })
  .signers([depositor])
  .rpc();

// 2. Recipient withdraws
await program.methods
  .withdraw()
  .accounts({
    recipient: recipient.publicKey,
    // ... other accounts  
  })
  .signers([recipient])
  .rpc();
```

### **CLI Usage**
```bash
# Deposit 100 tokens
anchor run deposit -- --amount 100 --recipient <RECIPIENT_PUBKEY>

# Withdraw tokens
anchor run withdraw

# Refund tokens
anchor run refund
```

## 📊 Account Structure

### **Escrow Account**
```rust
pub struct Escrow {
    pub depositor: Pubkey,  // Who deposited (32 bytes)
    pub recipient: Pubkey,  // Who can withdraw (32 bytes)  
    pub amount: u64,        // Token amount (8 bytes)
    pub bump: u8,          // PDA bump seed (1 byte)
}
// Total: 73 bytes + 8 byte discriminator = 81 bytes
```

### **PDA Seeds**
```rust
seeds = [b"escrow", depositor.key(), recipient.key()]
```

## 🎓 Educational Concepts

This contract teaches:

### **1. Cross Program Invocation (CPI)**
- **Regular CPI:** Human signs token transfer
- **PDA CPI:** Program signs on behalf of PDA using seeds

### **2. Program Derived Addresses (PDAs)**  
- Deterministic addresses owned by programs
- Enable secure token custody without private keys
- Generated from seeds + program ID

### **3. Associated Token Accounts**
- Standard pattern for token storage on Solana
- Predictable addresses derived from owner + mint
- Automatic creation and management

### **4. Anchor Account Constraints**
- Compile-time and runtime security checks
- Automated account validation and initialization
- Custom error handling

## ⚠️ Security Considerations

### **What This Contract Prevents:**
- ✅ Token theft (PDA ownership)
- ✅ Unauthorized withdrawals (access control)
- ✅ Double spending (single-use escrow)

### **What This Contract Doesn't Handle:**
- ❌ Time-based escrows (no expiration)
- ❌ Partial withdrawals (all-or-nothing)
- ❌ Multi-signature escrows (single recipient)
- ❌ Fee mechanisms (no protocol fees)

## 🧪 Testing

```bash
# Run all tests
anchor test

# Run specific test file
anchor test --skip-deploy tests/simple-escrow.ts

# Run tests with detailed logs
anchor test -- --reporter verbose
```

### **Test Coverage**
- ✅ Successful deposit flow
- ✅ Successful withdraw flow  
- ✅ Successful refund flow
- ✅ Unauthorized access attempts
- ✅ Error conditions and edge cases

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Interview Preparation

This contract demonstrates key concepts interviewers look for:

**Technical Skills:**
- ✅ CPI with PDA signing
- ✅ Secure token handling
- ✅ Account validation and constraints
- ✅ Error handling and custom errors

**Best Practices:**
- ✅ Minimal, focused functionality
- ✅ Clear code organization
- ✅ Comprehensive testing
- ✅ Security-first design

**Common Interview Questions:**
- *"How do you prevent rug pulls?"* → PDA ownership
- *"What's the difference between `new` and `new_with_signer`?"* → Human vs PDA authority
- *"How do PDAs work?"* → Deterministic addresses from seeds
- *"Why use Associated Token Accounts?"* → Predictable, standard pattern

## 📚 Additional Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [SPL Token Program](https://spl.solana.com/token)
- [Program Derived Addresses](https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)

---

**Built with ❤️ on Solana**