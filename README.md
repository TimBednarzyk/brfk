# brfk
A Brainfuck Interpreter, Compiler, Debugger, and Optimizer written in Rust.

## Features
* **Supported Languages**
  * ["Vanilla" Brainfuck](https://esolangs.org/wiki/Brainfuck)
  * [Brainfuck Extended Type I](https://esolangs.org/wiki/Extended_Brainfuck#Extended_Type_I)
* **Interpreter**
  * Supports all supported languages
* 30,000 8-bit cells that wrap
* EOF results in a 0

## Planned Features
* **Languages**
  * [Extended Type II](https://esolangs.org/wiki/Extended_Brainfuck#Extended_Type_II)
  * [Extended Type III](https://esolangs.org/wiki/Extended_Brainfuck#Extended_Type_III)
  * [BrainPlus](http://www.primaryobjects.com/2015/01/05/self-programming-artificial-intelligence-learns-to-use-functions/)
  * [Self-modiying Brainfuck](https://esolangs.org/wiki/Self-modifying_Brainfuck)
  * A brfk-only variant
* **Compiler**[<sup>1</sup>](#footnote-1)
  * To an executable
  * To C
  * To Rust
  * To Java
  * To JavaScript
  * To Python
  * To Brainfuck
    * This will allow one to generate optimized Brainfuck code and/or code for another variant of Brainfuck
  * _To any language that is suggested_
* **Debugger**
  * Will support all supported languages
* **Optimizations**[<sup>1</sup>](#footnote-1)
* Ability to choose cell size of 8, 16, 32, and 64 bits
* Ability to choose a custom number of cells or to have a variable number of cells
  * Extended Types II and III will always have a variable number of cells due to their ability to add/remove cells

## License
brfk is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in this repository for more information.

---

<b id="footnote-1">1: </b>Due to their self-modifying nature, the following variants will never be supported:
  * [Extended Type II](https://esolangs.org/wiki/Extended_Brainfuck#Extended_Type_II)
  * [Extended Type III](https://esolangs.org/wiki/Extended_Brainfuck#Extended_Type_III)
  * [Self-modiying Brainfuck](https://esolangs.org/wiki/Self-modifying_Brainfuck)

---

```sh
brfk 0.1.1
Tim Bednarzyk <timbednarzyk@gmail.com>
A Brainfuck Interpreter, Compiler, Debugger, and Optimizer.
Currently only has a working interpreter, and only supports basic and Extended Type I Brainfuck.

USAGE:
    brfk [OPTIONS] --path <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <FILE>    The path to the Brainfuck source code
        --mode=<MODE>    The mode that the Brainfuck code should be parsed in.
                         b     Basic mode (Default)
                         x1    Extended Type I mode
```
