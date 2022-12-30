<p align="center">
	<img width="600" src="./assets/banner-light.png">
</p>

## Introduction

Procyon is a companion app for Elite Dangerous. It provides

- Mapping integration
- Route planning
- Commodity location

## Quick Start

Procyon is still experimental and no release is available as of yet. You can compile a development build by following the steps outlined in the contribution guidelines.

## Contributing

Contributions to Procyon are welcome - feel free to open an issue or pull request for missing features or bugs.

### Prerequisites

Procryon is built using [Tauri](https://tauri.app), Rust, and TypeScript. You will need the following prerequisites:

- [`rustup`](https://rustup.rs)
- Node v16 or later
- [`pnpm`](https://pnpm.io)

### Setup

Set up the project by installing its dependencies:

```bash
pnpm i
```

You can then run each of the 2 apps available in the `./apps` folder using the following commands:

#### `desktop`

```bash
pnpm tauri dev
```

#### `landing`

```bash
pnpm dev
```

### Linting

Changes made to the source code are automatically linted and formatted when commits are made - your code won't be able to be pushed until it passes the linter.

### Testing

Testing is done on GitHub using various actions workflows. While not required for contribution, we ask that you check if your code runs on your local branch before committing as compute time is expensive!

## License

Procyon is licensed under the GNU General Public License, version 3. You can find the full license in the [LICENSE](./LICENSE) file.
