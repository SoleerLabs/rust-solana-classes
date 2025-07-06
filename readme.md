# 🦀 Rust Solana Classes – Assignment Submissions

Welcome to the official submission repository for the **Rust Solana Classes**   
This is where students will submit their solutions for each class episode.

---

## 📚 Structure

Each assignment will live in its own folder inside `/assignments`.  
Students should create a subfolder named after their GitHub username inside each episode folder and place their code there.

### Example:
```

assignments/
├── ep01\_intro\_to\_rust/
│   ├── charles/
│   │   └── main.rs
│   └── janedoe/
│       └── main.rs
├── ep02\_ownership\_and\_borrowing/
│   ├── charles/
│   │   └── main.rs

````

---

## 📝 Submission Instructions

1. **Fork** this repository.
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/SoleerLabs/rust-solana-classes.git
   ````

3. **Create a new branch** for your submission:

   ```bash
   git checkout -b ep01-my-solution
   ```
4. **Create a folder** inside the episode directory:

   ```bash
   mkdir -p assignments/ep01_intro_to_rust/YOUR_USERNAME
   ```
5. **Add your code** (`main.rs`, `Cargo.toml`, etc.) into your folder.

   ```bash
   cd assignments/ep01_intro_to_rust/YOUR_USERNAME
   cargo init
   ```
  
6. **Edit your `Cargo.toml` and append the lesson's folder name to the [package] section:**

   ```toml
   [package]
   name = "your_username_ep01_intro_to_rust"
   ```
   Do this for each lesson to have unique names for your Cargo.toml files.

7. **Commit and push**:

   ```bash
   git add .
   git commit -m "Add solution for EP01"
   git push origin ep01-my-solution
   ```
8. **Open a Pull Request** to the `main` branch on GitHub.

---

## ✅ Rules

* You can only modify your own folder.
* Don’t touch other students’ code or the assignment template.
* Follow best practices for clean, readable Rust.
* Use `cargo check` or `rustc` to ensure your code compiles before submission.

---

## 🧪 Bonus (Optional)

Some episodes may have bonus challenges. Completing them will earn you special mentions in the recap.

---

## 💬 Support & Questions

If you're stuck or have any questions, reach out on the Telegram channel or during the live sessions.

Happy hacking! 🦀
