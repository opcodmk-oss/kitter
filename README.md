<p align="center">
  <img src="./assets/readme/hero.en.png" width="100%" alt="Kitter — one Skill library where every project gets only what it needs">
</p>

<p align="center">
  <a href="./README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-3f8997" alt="Apache-2.0 license"></a>
  <img src="https://img.shields.io/badge/desktop-macOS%20%7C%20Windows%20%7C%20Linux-15191a" alt="macOS, Windows, and Linux desktop app">
  <img src="https://img.shields.io/badge/built_with-Rust-b8aaa0" alt="Built with Rust">
</p>

<p align="center"><strong>One Skill library. Every project gets only what it needs.</strong></p>

Kitter is a desktop app and CLI for managing Agent Skills across projects. Keep your Skills in one library, install the right combination for each project, and update them in one place.

Built entirely in Rust with GPUI, Kitter pairs a straightforward interface with a small footprint and smooth native performance.

<p align="center">
  <img src="./assets/readme/skill-workflow.png" width="100%" alt="Kitter maintains one Skill library and links selected Skills to projects and user-level installations">
</p>

## Why Kitter

Working across projects often means maintaining several copies of the same Skill and keeping track of what each Agent can use. Kitter keeps those connections visible:

- **Maintain once** — projects link to the same Skill source, so one update reaches every linked installation.
- **Choose per project** — give each project its own Skill set, with user-level installation for Skills you use everywhere.
- **See what is active** — inspect the Skills each Agent discovers, including installations outside Kitter, along with their sources and estimated context cost.

## Install Kitter

Download the app for your platform from [GitHub Releases](https://github.com/what1f/kitter/releases/latest).

- **macOS (Apple Silicon)** — open the `.dmg` and drag `Kitter.app` into `Applications`.
- **Windows (x64)** — download `Kitter-<version>-desktop-windows-x86_64.exe` and run it directly.
- **Linux (x64)** — extract `Kitter-<version>-desktop-linux-x86_64.tar.gz` and run `./Kitter` from the extracted `Kitter` directory.

Kitter is not yet signed with an Apple Developer ID. If macOS blocks the first launch, confirm that you downloaded it from the official release, then go to **System Settings → Privacy & Security → Open Anyway** and follow the prompts. See [Apple’s instructions](https://support.apple.com/102445).

<details>
<summary>Alternative: open using Terminal</summary>

For a copy downloaded from the official release, you can instead run the following command, then open Kitter again:

```bash
xattr -dr com.apple.quarantine /Applications/Kitter.app
```

</details>

The desktop app and CLI are separate release artifacts built on the same core. Standalone CLI packages for macOS, Windows, and Linux are available from [GitHub Releases](https://github.com/what1f/kitter/releases/latest). The built-in Kitter Skill resolves that standalone CLI and guides you through downloading it when needed.

## Manage your Skills with Kitter

### 1. Build one library

Use **+** to add Skills from a local folder, GitHub or a skills.sh-compatible source, or a Claude plugin source. If Skills are already scattered across projects, choose **Existing installations** to inspect and adopt them without moving their source directories.

Kitter keeps one maintained source for each Skill. Open its **Installs** tab to immediately see every project using it, every installation location, and the Agents that can discover it.

<p align="center">
  <img src="./assets/readme/skill-library.en.png" width="100%" alt="Kitter Skill library showing one managed Skill installed across several projects">
</p>

### 2. Install only where needed

Select a Skill, choose a project, then install it into the shared `.agents/skills` directory or an Agent-specific directory. Kitter creates managed links instead of independent copies, so projects can use different combinations without creating update drift.

<p align="center">
  <img src="./assets/readme/install-skill.en.png" width="100%" alt="Kitter installation dialog for selecting a project and Agent targets">
</p>

Skills you use across all projects can also be installed at the user level.

### 3. Verify what is actually active

Open **Projects** to see the complete effective Skill set for every Agent—not just installations managed by Kitter. The view discovers project, parent, user-level, built-in, and plugin-provided capabilities, then shows where each one came from.

The per-Agent token estimate helps you spot Skills that add unnecessary context overhead.

<p align="center">
  <img src="./assets/readme/project-effective-skills.en.png" width="100%" alt="Kitter project view showing managed and unmanaged effective Skills, plugins, Agents, and estimated context cost">
</p>

### 4. Update once

Run **Check for updates** from the desktop app or use `kitter check` and `kitter update`. Every managed project continues to use the same maintained source.

The equivalent CLI workflow is intentionally small:

```bash
kitter add npx https://github.com/owner/repository --skill skill-a
kitter install skill-a --project /path/to/project --target universal
kitter project /path/to/project
kitter update skill-a
```

## Standalone CLI and Agent Skill

You do not need the desktop app to use Kitter. Download the standalone CLI from [GitHub Releases](https://github.com/what1f/kitter/releases/latest), put `kitter` on your `PATH`, and install the [`$kitter` Skill](./resources/skills/kitter) directly:

```bash
npx skills add what1f/kitter --skill kitter
```

The Skill lets an Agent inspect the current machine, add or adopt Skill sources, install the right project combination, and verify the result through the standalone `kitter` CLI. If the CLI is missing, the Skill can guide you through downloading it from an official Release.

<details>
<summary><strong>Build from source</strong></summary>

```bash
git clone https://github.com/what1f/kitter.git
cd kitter
cargo run --release --locked --features desktop --bin kitter-desktop
```

</details>

## Platform status

- **macOS (Apple Silicon)** — desktop application and standalone CLI.
- **Windows (x64)** — desktop application and standalone CLI, tested on Windows with platform-specific startup and performance fixes.
- **Linux (x64)** — standalone CLI and desktop build available; the desktop app still needs validation on real systems.

## Local data

Kitter stores configuration and source records in the operating system's application-data directory. Skill contents live in the library directory:

| Platform | Default Skill library |
| --- | --- |
| macOS | `~/Library/Application Support/Kitter/skills` |
| Windows | `%LOCALAPPDATA%\Kitter\skills` |
| Linux | `$XDG_DATA_HOME/Kitter/skills` or `~/.local/share/Kitter/skills` |

View or change the location with `kitter library` and `kitter library --set /absolute/path`.

## Contributing

Issues and pull requests are welcome. Please open an [issue](https://github.com/what1f/kitter/issues) before starting a large behavioral or UI change so the scope can be aligned first.

If Kitter makes your Skill setup calmer, consider [starring the repository](https://github.com/what1f/kitter). It helps more multi-project developers find it.

## License

Kitter is available under the [Apache License 2.0](./LICENSE). Licenses for bundled fonts, icons, and other third-party material are listed in [THIRD_PARTY_LICENSES.md](./THIRD_PARTY_LICENSES.md).
