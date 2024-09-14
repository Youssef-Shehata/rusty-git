# Git CLI Tool In Rust

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Basic Usage](#basic-usage)
4. [Soon](#Soon)

## Introduction

Trying to write `git` from scratch in Rust , implementing almost the same functionality as the original in an attempt to learn the internals of 
git to use it better.

refrences that helped during this process:
[freecodecamp visual guide to git internals](https://www.freecodecamp.org/news/git-internals-objects-branches-create-repo/)

so far ive implemented :

- `init` initialize an empty repo in the desired folder; creating HEAD,objects/ and refs/.
- `hash-object` hash a blob file , with the option to write to git/objects.
- `cat-file` write the contents of a compressed blob.
- `ls-tree` write the contents of a git tree object with objects to recurse,write namesonly, etc...
- `write-tree` write the current tree of your working directory with all the folders and files (no staging area yet).

## Installation

this project is still in development , so you will have to clone this repo and compile it yourself.
for convenience you will find a small script called `rusty-git.sh` that will compile and run for you.

or you can build it manually:

```sh
cargo build --release
```

## Basic Usage

rusty-git.sh [COMMAND] [OPTION]...[FILE]...


-help menu:
```sh
    ./rusty-git.sh --help
```


-initialize an empty repo:
```sh
    ./rusty-git.sh init .
```

or

```sh
    ./rusty-git.sh init my-new-project-directory
```

-hash a file and store it in your objectsd dir :
```sh
    ./rusty-git.sh hash-object -w file.txt
```

