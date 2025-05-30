
# 🚀 **How to Contribute to PyFrame**

Thank you for considering contributing to PyFrame! 🙏

---

## 💬 **Support & Questions**

Please **do not use issues** for support questions. Issues are meant for tracking bugs and feature requests.  
👉 For questions about using PyFrame or your own code, please use the **[GitHub Discussions](https://github.com/YellowSiC/pyframe/discussions)**.

---

## 🐞 **Reporting Bugs**

When you find a bug, please include the following information:

✅ **Expected behavior:** What should happen?  
✅ **Actual behavior:** What happened instead?  
✅ **Minimal reproducible example:** If possible, a small example to reproduce the issue – this helps us a lot!  
✅ **Technical details:**  
- Python version  
- PyFrame version  
- OS and CPU architecture  
- For async protocols: `asyncio` loop implementation  
- Also check if the issue is already fixed in the latest release or latest code in the repo.

---

## 🔧 **Submitting Patches**

If you want to contribute a new feature or fix a bug:

🔹 **No open issue yet?** – Please open one to discuss it before you start!  
🔹 You can work on any issue that **does not have an open PR** or assigned maintainer.  
🔹 **Code guidelines:**  
- Use the provided formatters (`make format`)  
- Include tests if your patch changes existing behavior or adds features  
- Update relevant documentation  

---

### 🖥️ **First-time Setup (Local)**

1️⃣ Make sure you have a GitHub account and the latest `git` installed.  
2️⃣ Configure your git user:  
```bash
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

3️⃣ Fork PyFrame and clone your fork locally:  
```bash
git clone https://github.com/your-username/PyFrame
cd PyFrame
```

4️⃣ Install the latest stable version of Rust.  
5️⃣ Install [uv](https://github.com/astral-sh/uv).  
6️⃣ Initialize the environment and build PyFrame:  
```bash
make build-dev
```

---

### ✏️ **Start Coding!**

📌 **Create a new branch:**  
```bash
git fetch origin
git checkout -b 123-fix-something origin/master
```

📝 **Make your changes in your favorite editor** – and commit frequently:  
👉 [Git Commit Guide](https://afraid-to-commit.readthedocs.io/en/latest/git/commandlinegit.html#commit-your-changes)

🔍 **Run formatters and linters:**  
```bash
make format
make lint
```

📤 **Push & Create a PR:**  
- Push your branch to **your fork**  
- Create a pull request  
- Link the issue (`closes #123`) in the PR description

---

### ✅ **Running the Tests**

Run the basic test suite with these commands:  
```bash
make build-dev
make test
```

👉 This runs the most important tests locally. The full CI pipeline will run when you open your PR.

---

🎉 **Thank you for contributing!**  
Your contributions make PyFrame better – and help the entire community. 🚀
