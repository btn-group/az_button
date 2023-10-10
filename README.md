<br />
<div align="center">
  <a href="https://github.com/btn-group">
    <img src="images/logo.png" alt="Logo" height="80">
  </a>

  <h3 align="center">Button ($BTN)</h3>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#checking-code">Checking code</a></li>
      </ul>
    </li>
    <li>
      <a href="#deployment">Deployment</a>
    </li>
    <li>
      <a href="#references">References</a>
    </li>
  </ol>
</details>

## About The Project

PSP22 smart contract for Button ($BTN) on Aleph Zero.

## Getting Started
### Prerequisites

* [Rust](https://www.rust-lang.org/)
* [Cargo](https://doc.rust-lang.org/cargo/)
* [ink!](https://use.ink/)
* [OpenBrush](https://openbrush.io/)
* [Cargo Contract v3.2.0](https://github.com/paritytech/cargo-contract)
```zsh
cargo install --force --locked cargo-contract --version 3.2.0
```

### Checking code

```zsh
cargo checkmate
cargo sort
```

## Deployment

1. Build contract:
```sh
cargo contract build --release
```
2. If setting up locally, start a local development chain:
```sh
substrate-contracts-node --dev
```
3. Upload, initialise and interact with contract at [Contracts UI](https://contracts-ui.substrate.io/).

## References

- https://openbrush.io/
- https://github.com/Brushfam/openbrush-contracts/tree/4.0.0
- https://github.com/Brushfam/openbrush-contracts/blob/21fdff1c6a56640c0767a30fe7ace3d09057404b/tests/psp22.rs#L76
