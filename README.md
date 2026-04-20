EduTrust: Decentralized Education Escrow

EduTrust is a secure, blockchain-based intermediary designed to eliminate "mis-trust" between independent students and tutors. By leveraging the Stellar/Soroban network, we ensure that payments are only released when educational milestones (certificates) are verified.

Deployment Details

Contract ID: CAFQ5YWPWRXBVTTMB6E25THJN3JIYKGVJZHDXUVCZQUQKSQKT5KE7V2C
WASM Hash: 8dd04d4cfcb429d6c75869857e5a9db4ce9552393511a45d07a1e1fcb2cebcc0
Deployment Transaction: View on Stellar.Expert
Contract Interface: Interact via Stellar Laboratory

Deployment Verification
<img width="1913" height="987" alt="Smart Contract Deployment" src="https://github.com/user-attachments/assets/773797fa-8842-4cff-bdaf-ea70a2e627f9" />

The Problem

In the online tutoring space, trust is a major barrier:
Students are hesitant to pay upfront to strangers for fear of not receiving the promised lessons.
Tutors risk providing their time and resources only to have the student disappear before payment.

How It Works (The Flow)

EduTrust acts as a decentralized Middle Man:
Freighter Wallet: The student deposits the course fee into the Smart Contract. The funds are locked and visible to both parties.
Mentorship: The tutor conducts the training sessions.
Verification: Upon completion, the tutor registers a unique Certificate Hash on the blockchain via the contract.
Automatic Release: Once the certificate is registered, the contract automatically releases the stored XLM/funds to the tutor's wallet.
