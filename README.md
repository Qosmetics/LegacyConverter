# Why this?

Qosmetics is going through another rewrite, and this program will handle most of the conversion from legacy files to the new file format for you. it's a simple rust program since I would be parsing json and doing command line arguments, and I like doing those in rust since working on qpm-rust (convenience)

# Compiling the program

I believe all you need is cargo installed, then just clone the repo:

```
git clone https://github.com/Qosmetics/LegacyConverter.git
```

After that all you should need to do is run cargo build:

```
cargo build --release
```

You could obviously omit the --release, at the point of writing there is no different behaviour between release and debug (logging could change eventually?)

