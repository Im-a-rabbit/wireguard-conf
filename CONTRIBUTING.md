# Contributing

## tl;dr

Don't be a jerk, don't break backward compatibility (unless you really want a major release) and
make useful suggestions.

1. Fork & clone
2. Install Rust, Cargo and [just (optional)](https://just.systems/). 

   Or start nix shell: `direnv allow` or just `nix develop`
3. Make changes
4. Format and lint code:
   ```shell
   just fmt  # same as `cargo fmt`
   just lint # same as `cargo clippy --all-features`

   # or fix automatically: just lint-fix
   ```
5. Commit changes (use [Conventional commits](https://www.conventionalcommits.org/en/v1.0.0/))
   ```shell
   git commit -m "feat: did something"
   ```
6. Send PR

## LLM usage

Please, don't submit PRs made entirely by LLM..

Invest in your own brain capability, learn and poke around thinks, but don't vibecode.

I'm using LLMs **only** to do some dumb tasks (e.g. implement trait same as above), throw some
ideas during mind-block (e.g. can only think of one solution) or as a fallback to google'ing, if
problem is obscure.

If possible, use local LLMs and fallback to some cloud ones.
