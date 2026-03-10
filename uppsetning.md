# Uppsetning á Rust þróunarumhverfi fyrir VSCode

Fara eftir leiðbeiningunum [hér](https://rust-lang.org/tools/install/).

Ef það gengur ekki, fylgið leiðbeiningum hér fyrir neðan.

- [Windows](#windows)
- [macOS](#macos)
- [Ubuntu](#ubuntu)

## Windows

Setja inn Ubuntu með WSL (Windows Subsystem for Linux) ef þarf, ætti að vera hjá flestum síðan í KEST1VL. (Leiðbeiningar [hér](https://github.com/gestskoli/KEST1VL/blob/master/WSL.md))

Síðan í Ubuntu:
```bash
sudo apt update && sudo apt upgrade
```
síðan
```bash
sudo apt install build-essential
```
og loks
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### VSCode stillingar fyrir Windows WSL
Setja inn eftirfarandi Extensions:
- WSL
- Rust-Analyzer
- CodeLLDB
- Error Lens

## macOS

Keyra eftirfarandi í terminal:
```bash
xcode-select --install
```
og síðan
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### VSCode stillingar fyrir macOS
Setja inn eftirfarandi Extensions:
- Rust-Analyzer
- CodeLLDB
- Error Lens

## Ubuntu
```bash
sudo apt update && sudo apt upgrade
```
síðan
```bash
sudo apt install build-essential
```
og loks
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### VSCode stillingar fyrir Ubuntu
Setja inn eftirfarandi Extensions:
- Rust-Analyzer
- CodeLLDB
- Error Lens