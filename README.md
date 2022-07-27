<p align="center">
    <img src="docs/libi_logo.svg" width="300">
</p>

<h1 align="center">
Libi - The common sense C++ package manager
</h1>

Libi is a cross-platform package manager for C++ projects that handles library management for you so you can focus on writing code instead of building and linking all the libraries your project depends on.

Libi installs libraries using their git repo link. In the future, I'd like to implement some kind of list of commonly installed libaries so they can be installed by their name alias, i.e.
```bash
$ libi add glfw
```

instead of
```bash
$ libi add https://github.com/glfw/glfw.git
```

However, for now the latter is the only supported method of installing libraries. This does have one advantage where the source of the libary can be hosted anywhere and there's no need for a single global repository system like NPM or Crates.io

## Installing

### For Windows/macOS
You can install the latest build of Libi for Windows/macOS on the [Releases](https://github.com/jakerieger/libi/release/latest) section of this repository.

### For Linux:
Run the following command in your command line:
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://github.com/jakerieger/libi/releases/download/latest/install-linux.sh | sh
```
> **Note**: We recommend **never** blindly running shell scripts downloaded from the internet. Please verify the contents of the above script before executing it.

### Building from source
Libi is written in Rust so you'll need the Rust buildchain in order to build Libi from source. 

With the Rust buildchain installed, you can clone the repo, `cd` in to the project root and run:
```bash
$ cargo build --release
```

Once finished executing, the binary can be found in the `targets/release` directory inside the project root. You can then copy this to whatever directory you desire and add it to your $PATH environment variable.

## Quickstart

### Installing a library in the current project directory
```bash
$ libi add <git repo>
```

### Remove a library from the current project
```bash
$ libi remove <git repo>
```

### Set default compiler for building libaries
You can simply use the name of the compiler (such as gcc, clang, msvc, etc) and Libi will search your system for the install path
```bash
$ libi config --compiler clang
```

If you want Libi to detect the compiler used in the current project, you can set the `compiler` flag to `detect`:
```bash
$ libi config --compiler detect
```