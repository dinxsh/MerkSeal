# Video Demo Script - MerkSeal on Mantle

## 🎥 3-Minute Demo for Hackathon Judges

### Recording Setup
- **Tool**: Loom, OBS, or Zoom
- **Resolution**: 1080p minimum
- **Audio**: Clear microphone (test first!)
- **Screen**: Clean desktop, close unnecessary apps
- **Browser**: Have Mantle explorer tab ready

---

## 📝 Script (3 minutes)

### [0:00-0:20] Introduction (20 seconds)

**[Show title slide or README]**

> "Hi judges! I'm excited to show you MerkSeal - a verifiable storage layer built specifically for Mantle L2.
>
> The problem: How do you prove a file hasn't been tampered with without storing it on-chain?
>
> The solution: Store files off-chain, anchor Merkle roots on Mantle for cryptographic verification.
>
> Let's see it in action."

**Key Points**:
- State the problem clearly
- Mention Mantle L2 immediately
- Promise a demo

---

### [0:20-1:00] Upload & Anchor (40 seconds)

**[Show terminal with server running]**

> "First, I have the MerkSeal server running locally. It's built in Rust with actix-web for high performance.
>
> Now I'll upload some legal documents..."

**[Run command]**:
```bash
curl -X POST http://localhost:8080/upload \
  -F "contract=@sample-contract.txt" \
  -F "nda=@sample-nda.txt"
```

**[Show JSON response]**

> "The server computed a Merkle root from these files. Notice it's just 32 bytes - that's all we need to store on-chain.
>
> Now let's anchor this root on Mantle..."

**[Run anchor script]**:
```bash
node anchor.js 0x9f86d081... "Legal docs 2026-01-15"
```

**[Show transaction output]**

> "And we're on Mantle! Transaction confirmed in under 3 seconds. Cost? Less than a tenth of a cent."

**Key Points**:
- Show actual commands (not slides)
- Highlight the 32-byte root
- Emphasize speed and cost

---

### [1:00-1:40] Verification (40 seconds)

**[Show Mantle explorer]**

> "Let's verify this on the Mantle blockchain explorer..."

**[Navigate to transaction, show BatchRegistered event]**

> "Here's our transaction. You can see the Merkle root stored on-chain, along with the timestamp and owner.
>
> Now anyone can verify these files..."

**[Run client verification]**:
```bash
cargo run -p client -- verify --batch-id 1 --mantle-batch-id 1
```

**[Show verification output]**

> "The client queries Mantle, compares the on-chain root with the local files, and confirms: these files are authentic and haven't been tampered with."

**Key Points**:
- Show the actual blockchain explorer
- Demonstrate client verification
- Emphasize "anyone can verify"

---

### [1:40-2:20] Tamper Detection (40 seconds)

**[Modify a file]**

> "Now let's see what happens if someone tampers with a file..."

**[Edit file in notepad, save]**

> "I just modified one of the documents. Let's verify again..."

**[Run verification]**:
```bash
cargo run -p client -- verify --batch-id 1
```

**[Show error output]**

> "Boom! Tamper detected. The Merkle root doesn't match, so we know the files have been modified.
>
> This is the power of cryptographic verification - you can't fake it."

**Key Points**:
- Make tampering obvious (show the edit)
- Show the verification failure clearly
- Emphasize cryptographic security

---

### [2:20-3:00] Why Mantle & Wrap-Up (40 seconds)

**[Show BENCHMARKS.md or cost comparison]**

> "Why Mantle? Three reasons:
>
> One: Cost. On Ethereum mainnet, this would cost $50. On Mantle? $0.0001. That's 99.9998% cheaper.
>
> Two: Speed. Mantle's L2 confirms in 2-3 seconds vs 15 seconds on Ethereum.
>
> Three: Scalability. We can notarize thousands of documents per second.
>
> Real-world use cases: Legal document notarization, supply chain verification, medical records, IP protection.
>
> MerkSeal is production-ready infrastructure for the Mantle ecosystem. All code is open source, fully documented, and deployed on Mantle testnet.
>
> Thank you!"

**Key Points**:
- Quantify the benefits (numbers!)
- Mention real use cases
- State it's production-ready

---

## 🎬 Recording Tips

### Before Recording
- [ ] Test your microphone
- [ ] Close unnecessary applications
- [ ] Clear browser history/bookmarks bar
- [ ] Have all commands in a text file to copy-paste
- [ ] Do a practice run (time yourself!)
- [ ] Charge your laptop

### During Recording
- [ ] Speak clearly and not too fast
- [ ] Pause briefly between sections
- [ ] Show your face (optional but builds trust)
- [ ] Smile! Enthusiasm is contagious
- [ ] If you make a mistake, pause and restart that section

### After Recording
- [ ] Watch it back (check audio/video quality)
- [ ] Add captions if possible
- [ ] Upload to YouTube (unlisted or public)
- [ ] Add to README.md and submission

---

## 📊 Alternative: Slide Deck Approach

If live demo is risky, create slides with screenshots:

### Slide 1: Title
- Project name
- Tagline: "Verifiable Storage on Mantle L2"
- Your name/team

### Slide 2: Problem
- How do you prove file integrity?
- Current solutions are expensive/slow

### Slide 3: Solution
- Merkle trees + Mantle L2
- Off-chain storage + on-chain verification

### Slide 4: Demo - Upload
- Screenshot of curl command
- Screenshot of JSON response with Merkle root

### Slide 5: Demo - Anchor
- Screenshot of anchor script output
- Screenshot of Mantle explorer transaction

### Slide 6: Demo - Verify
- Screenshot of verification success
- Screenshot of tamper detection

### Slide 7: Why Mantle
- Cost comparison chart
- Speed comparison
- Scalability numbers

### Slide 8: Use Cases
- Legal, supply chain, medical, IP
- Real-world impact

### Slide 9: Technical Stack
- Solidity, Rust, TypeScript
- Mantle L2, The Graph (if implemented)

### Slide 10: Thank You
- GitHub link
- Demo link
- Contact info

---

## 🎯 Key Messages to Emphasize

1. **Mantle-Native**: Built specifically for Mantle L2
2. **Cost Savings**: 99.9998% cheaper than Ethereum
3. **Production-Ready**: Complete, working implementation
4. **Real Use Cases**: Legal, supply chain, medical
5. **Infrastructure**: Building blocks for other dApps

---

## 📹 Video Checklist

- [ ] Introduction (problem + solution)
- [ ] Live demo (upload → anchor → verify)
- [ ] Tamper detection
- [ ] Why Mantle (cost, speed, scalability)
- [ ] Real-world use cases
- [ ] Call to action (GitHub, docs)
- [ ] Under 3 minutes
- [ ] Clear audio
- [ ] 1080p video
- [ ] Uploaded and linked in README

---

**Good luck! You've got this! 🚀**
