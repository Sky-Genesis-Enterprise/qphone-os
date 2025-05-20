# QPhone OS – The Open Mobile Operating System

QPhone OS is the flagship mobile operating system powering the **QPhone** – an open-source, eco-conscious, privacy-first smartphone.  
The OS is built in **Rust** for performance and safety, with **Lua** as the scripting language for all default and third-party apps.
This project is developed and maintained mainly by (Sky Genesis Enterprise)[https://skygenesisenterprise.com] and its teams of developers.

## 🚀 Project Goals

- 🦀 Core system written in **Rust**
- 🧩 Modular application system powered by **Lua**
- 🔐 Transparent and privacy-respecting architecture
- 🌱 Designed for long-term support and repairability
- 📱 Companion hardware: QPhone (6.3" open hardware device)

---

## 📦 Features

- QuantumOS core built in Rust with async runtime
- Custom Lua VM integration for UI and apps
- Official app store via `marketplace.quantum-os.org`
- OTA updates via `packages.quantum-os.org`
- Open PGP-based developer authentication
- Full SDK and developer portal: `developer.quantum-os.org`

---

## 🧪 Local Development

You can run the OS core and preview tools using Docker.

### 🐳 Run with Docker

```bash
git clone https://github.com/quantum-os/qphone-os.git
cd qphone-os

# Build and run in a container
docker build -t qphone-os .
docker run -it qphone-os
````

> ℹ️ You can modify the Dockerfile to mount your local volume or forward ports for preview/debug.

---

## 🧑‍💻 Contributing

We welcome contributions from developers, designers, and makers.

### Step 1: Fork & Clone

```bash
git clone https://github.com/YOUR-USERNAME/qphone-os.git
cd qphone-os
```

### Step 2: Install Dev Tools

You’ll need:

* Rust toolchain (`rustup`)
* Node.js (v18+)
* Docker (optional for full build preview)

Install dependencies:

```bash
npm install
```

### Step 3: Submit a PR via Script

All PRs are handled using a unified workflow script.

```bash
npm run pr
```

This will:

* Lint your code
* Run minimal checks
* Ask for a PR description
* Create and push a feature branch
* Open the pull request via GitHub API

---

## 📂 Repository Structure

```bash
qphone-os/
├── src/             # Rust system core
├── apps/            # Default Lua apps
├── scripts/         # Automation & dev tools
│   └── pr.mjs       # PR submission script
├── docs/            # Specs and documentation
├── Dockerfile       # For preview & builds
├── README.md        # This file
├── LICENSE          # AGPL-3.0 license
```

---

## 📜 License

QPhone OS is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.
See [`LICENSE`](./LICENSE) for full details.

---

## 🌍 Community & Resources

* 🌐 Website: [quantum-os.org](https://quantum-os.org)
* 🛠 Developer Portal: [developer.quantum-os.org](https://developer.quantum-os.org)
* 🧑‍🎓 Getting Started: `docs/getting-started.md`
* 📘 Specs: [`docs/qphone-specs.md`](./docs/qphone-specs.md)

---

## 🙌 Support & Contact

If you experience issues or would like to suggest a feature, feel free to:

* Open an [issue](https://github.com/quantum-os/qphone-os/issues)
* Contact the core team via [hello@quantum-os.org](mailto:hello@quantum-os.org)

> ✨ Let’s build the next open and ethical mobile platform — together.
