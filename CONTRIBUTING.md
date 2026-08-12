# Contributing

Don't be a jerk, don't break backward compatibility (unless you really want a major release),
make useful suggestions and write good code.

## Building

### Pre-requirements

- On systems with [nix](https://nixos.org/):
  ```
  direnv allow         # (flakes)
  # or nix develop .   # (flakes)
  # or nix-shell .
  ```
- On other systems:

  Install Rustup and [just (optional)](https://just.systems/)

### Development

Do changes and then:

- with just:
  ```
  just fmt
  just lint
  # just lint-fix
  just test
  ```

- without just:
  ```
  cargo fmt
  cargo clippy --all-features
  # cargo clippy --all-features --fix --allow-dirty
  cargo test --all-features
  ```

## LLM/AI contributions

Same as [servo's](https://book.servo.org/contributing/getting-started#ai-contributions)

> **tl;dr**
> 
> Don't contribute code made entirely by LLM.
>
> LLM tools can be only used for:
>   - translating
>   - assisting in finding bugs (you must verify the output)
>   - helping to understand codebase (warning same as above)
>   - code reviewing (same warning, be careful, it's unreliable and noisy)
