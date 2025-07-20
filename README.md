# Nyw (nywida)

A declarative package manager helper. Define your entire system package configuration in a single JSON file and let nyw handle the rest.

## What is nyw?

Nyw allows you to declaratively manage packages across different operating systems and package managers using a simple JSONC configuration file. Instead of manually installing packages one by one, you describe your desired system state and nyw makes it happen.

## Features

### Fully Supported Package Managers
- **dnf/yum** (Fedora/RHEL)
- **brew** (macOS/Linux)
- **pacman** (Arch Linux)
- **apk** (Alpine Linux)
- **apt** (Debian/Ubuntu)
- **paru** (AUR helper)
- **yay** (AUR helper)

### Experimental Support
- **winget** (Windows) - Currently in testing phase and is not fully implemented

### Planned Support
- **Android Package Manager** via ADB - Planned

## Getting Started

1. Create your configuration file based on the provided `example.jsonc`
2. Run nyw to apply your configuration
3. Your system will be configured exactly as described

## Configuration

Nyw uses a JSONC (JSON with Comments) configuration file where you can define all your packages. Check out `example.jsonc` in this repository for a complete example of how to structure your configuration.

The configuration file allows you to:
- Specify packages for different package managers
- Set up package repositories
- Define system-specific configurations
- Comment your configuration for better maintainability

## Installation

```bash
# AUR
paru -S nyw

# Linux/MacOS
already supported but no install instructions
```

## Usage

```bash
# Apply your configuration
nyw
```

## Why nyw?

Born from the frustration of manually managing packages across different systems, nyw brings the declarative package management philosophy of NixOS to any Linux distribution (and beyond). Whether you're managing a single machine or multiple systems, nyw ensures consistency and reproducibility.

## Contributing

We welcome contributions! Whether it's adding support for new package managers, improving existing functionality, or enhancing documentation.

## License

Apache license 2.0

---

