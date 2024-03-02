# StellwerkSim Launcher

This is a Launcher for the java game Stellwerk Simulator found [here](https://stellwerksim.de). 
Inspiration for this came from my Nix packaging adventures, where I discovered its kinda hard to package Java web apps to linux platforms natively.
I made this to better integrate StellwerkSim for native platforms.

## Development (Flakes)

This repo uses [Flakes](https://nixos.wiki/wiki/Flakes) from the get-go.

```bash
# Dev shell
nix develop

# or run via cargo
nix develop -c cargo run

# build
nix build
```

We also provide a [`justfile`](https://just.systems/) for Makefile'esque commands.
