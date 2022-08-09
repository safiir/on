### Usage

do some stuff when the specified file changed.

```sh
on path event action

event: prechange | predelete | create | change | chmod | delete | remove | rescan | error
```

### Example

```sh
on main.rs change cat main.rs
```

### Cross Compile (Mac -> Linux)

```sh
# add linux target
rustup target add x86_64-unknown-linux-gnu
# install cross compiler
brew tap SergioBenitez/osxct
brew install x86_64-unknown-linux-gnu
# build
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --release --target=x86_64-unknown-linux-gnu
```
