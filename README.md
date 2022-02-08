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

# Typical Usage

If you're ever stuck and don't know which commands exist, feel free to run the help command:

```
legacy-converter --help
```

To convert a legacy qosmetics file, you just run the appropriate subcommand like this:

```
legacy-converter qbloq
legacy-converter qsaber
legacy-converter qwall
```

after that you need to provide a few arguments so that the program knows what values to use in the created file.

### Cyoob

For cyoobs a command looks like this:

```
legacy-converter qbloq <FILENAME> <OBJECT_NAME> <AUTHOR> <DESCRIPTION> [OPTIONS]
```

Example usage:

```
legacy-converter qbloq ./oldbloq.qbloq "Old Bloq" "RedBrumbler" "Just an old bloq that's ready for conversion!" --hasBomb --showArrow 
```

For more info about the command you can run the specific help command:

```
legacy-converter qbloq --help
```

### Whacker

TODO: Awaiting actual implementation of a quest mod

### Dodgy

TODO: Awaiting actual implementation of a quest mod
