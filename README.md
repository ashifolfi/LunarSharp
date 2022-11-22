# LunarSharp

LunarSharp is a programming language that compiles into Lua code with a syntax similar to languages like C# or Rust.

LunarSharp tries to be almost as simple as Lua (with only a slightly more advanced syntax) but adds many optional features that can make code look better or make some things (like metatables) easier to code.

LunarSharp does not compile to a specfic version of Lua: flags can be toggled to alter the output to allow most if not all versions or modifications of Lua to be compiled to with LunarSharp.

Additionally support packages (WIP) can be used to change how certain internal functions act to match any changes in your specific Lua implementation. (See `src/support/SRB2` for an example on how this can be done)

## Coming Soon
These features are planned but are still being added
- Class and namespace support
- availability keyword support
- case statements
- support packages
- preprocessor

For any suggestion or bug you can make a github issue.

## General syntax differences
### From Lua:
- Code blocks are now inside `{}` instead of `then`/`do`/`repeat` and `end`/`until`
- Comments are made with `// ...` or `/* ... */`
- availability of a variable, function, or class must be explicitly defined
- Variables are created with a specific type that cannot be changed*

\*Variables created inside a function using the `var` keyword lack a type. Doing this is discouraged but not counted as an error as it can be useful.

### From CSharp:
- Classes are encouraged but not required
- JavaDoc comments are used instead of XML for a much smaller impact on line count

If you want a complete documentation of every change and addition in LunarSharp check [the wiki](https://github.com/ClueLang/Clue/wiki).

## Example code
TODO: this

## How to install

### Using Cargo
TODO: this

### Manual insallation
1. Download the latest release and save it somewhere
2. Open your system environment variables
3. Add the path to the directory that contains `lunarsharp.exe` in the PATH variable
4. Type `lunarsharp` in your cmd/PowerShell to run the compiler, it will explain the rest

## Why LunarSharp?
MoonSharp was already taken by a .NET project and ClueSharp just sounds odd
