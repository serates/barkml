# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.3 (2025-06-10)

### Chore

 - <csr-id-5c3f0800bc2dd43e9f750d9d11435dd83a34d31b/> update dependencies and Cargo.lock
   This commit updates several Rust dependencies to their latest versions, including:
   - Bump hashbrown to 0.15.4
   - Update snafu to 0.8.6
   - Upgrade uuid to 1.17.0
   - Add new dependencies like bumpalo and js-sys
   - Refresh Cargo.lock to reflect dependency changes
 - <csr-id-07393eac37f6fc542fc16a1a732a06c1b105896e/> improve error message clarity and formatting in error handling

### New Features

 - <csr-id-9f4c382e88e9909a5607b21f1ca31129336a336a/> enhance BarkML loader documentation and improve code comments
   This commit improves the documentation for the BarkML loader modules by:
   - Adding comprehensive module and struct-level documentation
   - Clarifying the purpose and behavior of loader methods
   - Improving inline comments to explain complex logic
   - Using more precise language in docstrings
   - Enhancing code readability and understanding of the loading process
 - <csr-id-00589d25fd01c8bfe897d8a1c8aeee9b0e94d99a/> enhance AST module with improved documentation and structure
   This commit improves the Abstract Syntax Tree (AST) module by:
   - Adding comprehensive module-level documentation
   - Introducing a new `prelude` module for easy imports
   - Expanding docstrings for key structs and enums
   - Improving code organization and readability
   - Adding a constructor method for the `Statement` struct
 - <csr-id-14e38e02a7462f38cf14cff83a472cd5190ec4a5/> enhance Location struct with more context and error reporting details
   This commit improves the Location struct by:
   - Adding source text and length tracking
   - Implementing new constructors for more flexible location creation
   - Improving display and context methods
   - Updating error handling to include source context
   - Enhancing lexer location generation with more detailed information
 - <csr-id-e8a24d6ec53150e9c7be5a7cccabc83b6e85d2b6/> enhance parser comment and token handling
   - Improve comment processing to support multiple comments
   - Add support for more token types in type parsing
   - Optimize array parsing with pre-allocated vectors
   - Refactor token extraction and error handling
   - Expand supported type tokens for more flexible parsing

### Bug Fixes

 - <csr-id-b7a9d207f7b4b0d70c4029f4c4f67d35c2aa7f6a/> multiline strings were not lexing correctly

### Other

 - <csr-id-61a51e14d845fcaf689e5c95acd231c89fb0cb98/> print out line numbers and column numbers instead of character count
 - <csr-id-be6b9fcb574d30e6f109b83cedf8e0a1d680eaa5/> fix missing null and some labels

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release over the course of 445 calendar days.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update dependencies and Cargo.lock ([`5c3f080`](https://github.com/serates/barkml/commit/5c3f0800bc2dd43e9f750d9d11435dd83a34d31b))
    - Enhance BarkML loader documentation and improve code comments ([`9f4c382`](https://github.com/serates/barkml/commit/9f4c382e88e9909a5607b21f1ca31129336a336a))
    - Enhance AST module with improved documentation and structure ([`00589d2`](https://github.com/serates/barkml/commit/00589d25fd01c8bfe897d8a1c8aeee9b0e94d99a))
    - Improve error message clarity and formatting in error handling ([`07393ea`](https://github.com/serates/barkml/commit/07393eac37f6fc542fc16a1a732a06c1b105896e))
    - Enhance Location struct with more context and error reporting details ([`14e38e0`](https://github.com/serates/barkml/commit/14e38e02a7462f38cf14cff83a472cd5190ec4a5))
    - Enhance parser comment and token handling ([`e8a24d6`](https://github.com/serates/barkml/commit/e8a24d6ec53150e9c7be5a7cccabc83b6e85d2b6))
    - Simple update of dependencies to keep up to date ([`a4b59e7`](https://github.com/serates/barkml/commit/a4b59e73f60efb0a29733fbea1f30e0d6f8460f4))
    - Fix statement grouped logic ([`b8140ff`](https://github.com/serates/barkml/commit/b8140ffdf7e0560e993506f69671ffc6efb49404))
    - 0.8.1 bugfixes ([`c1c2b78`](https://github.com/serates/barkml/commit/c1c2b780c7378fcb74f834ad4730ca7ec138accf))
    - 0.8.0 Symbols ([`a0b4972`](https://github.com/serates/barkml/commit/a0b49727519e485fe256573f29e27bf76a3dfea6))
    - Bump version and sync README and Changelog ([`296455c`](https://github.com/serates/barkml/commit/296455c1eecb4732108cf306b610e6f109017e00))
    - Fix testing with label change and missed float type bug ([`3c659bd`](https://github.com/serates/barkml/commit/3c659bd619ce5b4acf68cd5741e17beb792c00e6))
    - Fix injection ids and load operations ([`9c0d8e9`](https://github.com/serates/barkml/commit/9c0d8e98752ecff42710cc6085a36f3bc014a09f))
    - Consolidate error types now that we have a set ([`ddccbb3`](https://github.com/serates/barkml/commit/ddccbb388c7bcc1b0e0059b433d1d06beac40520))
    - Restructuring, track location on files through ast ([`56bd2a8`](https://github.com/serates/barkml/commit/56bd2a8f3a7f18484044db58a3e5b6fdb04ab543))
    - Migrate to new AST objects ([`6108479`](https://github.com/serates/barkml/commit/6108479d1edaca000db8841f7808aaa7375b4112))
    - Print out line numbers and column numbers instead of character count ([`61a51e1`](https://github.com/serates/barkml/commit/61a51e14d845fcaf689e5c95acd231c89fb0cb98))
    - Multiline strings were not lexing correctly ([`b7a9d20`](https://github.com/serates/barkml/commit/b7a9d207f7b4b0d70c4029f4c4f67d35c2aa7f6a))
    - Fix missing null and some labels ([`be6b9fc`](https://github.com/serates/barkml/commit/be6b9fcb574d30e6f109b83cedf8e0a1d680eaa5))
    - Release 0.6.5 ([`13d69b6`](https://github.com/serates/barkml/commit/13d69b612677ef301409028fd57429dd4d9682a1))
    - New recursive descent parser ([`2266eb7`](https://github.com/serates/barkml/commit/2266eb75485200ebf5cbd93759592da4595366dc))
    - Migration to logos+chumsky complete ([`0893eeb`](https://github.com/serates/barkml/commit/0893eebbd5ba63b482c612f1b04f11e76f780871))
    - Finish typing system ([`aa54b93`](https://github.com/serates/barkml/commit/aa54b9392fe9d2f393ec5e5f482391608c73673c))
    - Change block id to always include labels to distinguish them ([`b37dd5b`](https://github.com/serates/barkml/commit/b37dd5b6b5b0073cc134a766d2c518a9cf697b41))
    - Finish typing system ([`c715c05`](https://github.com/serates/barkml/commit/c715c05d9ed04a5cc466c45d7bfeb3db02a9cc9e))
    - Scope based macro resolution ([`a38bd86`](https://github.com/serates/barkml/commit/a38bd8691befe4275b96b1d82f67a10e796000de))
    - Fix standard loader map ordering ([`99f8161`](https://github.com/serates/barkml/commit/99f8161a12f8e605d2b98f763d5fc4157d3d5259))
    - Fix issue with inconsistent ordering with read_dir ([`7d5c780`](https://github.com/serates/barkml/commit/7d5c780a9a0bba202ee7da8c133f015ecf3bf4a1))
    - Support for versions and version req ([`9a291fb`](https://github.com/serates/barkml/commit/9a291fb16acc6cf8fc5c6b455572f3c593d2dd9e))
    - Introduce modules and standardize loader trait ([`f492f53`](https://github.com/serates/barkml/commit/f492f53dc4476a73f31277f7d3154276a4df3872))
    - Fix import and unimplemented feature ([`296f846`](https://github.com/serates/barkml/commit/296f846699315c23e91fc989d0c9d4277bca1ea6))
    - Configuration loader implementation ([`d6beb64`](https://github.com/serates/barkml/commit/d6beb642eb4bfe017845ad1e1d76d0a373ce9356))
    - Loader feature to allow multiple configuration files to be loaded at once ([`4ff4c8d`](https://github.com/serates/barkml/commit/4ff4c8d124bf2c27e33b23fed74dba804a5a1ed3))
    - Support for precision integers and floats ([`5fa8c8b`](https://github.com/serates/barkml/commit/5fa8c8b654df6e5cbf4ac1a0b4d4cca0ccd1c6e7))
    - Consolidate statement and value ([`094f82f`](https://github.com/serates/barkml/commit/094f82fa46413a168a8569f0eea5cc992623062a))
    - Create rust.yml ([`5773da7`](https://github.com/serates/barkml/commit/5773da74156861f140391eef92133fb3fe0ee6b1))
    - Add binary encoding support through messagepack ([`4f341f3`](https://github.com/serates/barkml/commit/4f341f3ac7e197dffa270f848b84bd27256a3e62))
    - Update description ([`9c3db02`](https://github.com/serates/barkml/commit/9c3db026a03c5856c7d430bc4d1cc37e4acc62c0))
    - Initial release of BarkML ([`9460e17`](https://github.com/serates/barkml/commit/9460e17c7a7dfb92f8dcafba2880009141716db5))
    - Initial commit ([`7999de1`](https://github.com/serates/barkml/commit/7999de1f5c179329ebe09e861a87993845b8413c))
</details>

## 0.8.2

Fixes:

* Blocks are not treated as grouped in statement

## 0.8.1

Fixes:

* Fixes implicit conversion from symbol to string

## 0.8.0

Changes:

* New get_child method in Walk for loading child statements
* Label's can no longer be a standalone value as this causes conflicts in parsing
* Introduces Symbol identifiers that start with :, this replaces using standalone label values

## 0.7.0

Changes:

* Introduces 128-bit integer types
* Refactors crate into a more maintainable structure
* Resplits Statements and Values for better data representation
* Improves error messages
* Implements a new Walker for easily reading and converting data
* Makes Statement and Value implement Serialize + Deserialize
* Consolidates comments and labels into new Metadata type

Removes:

* binary feature no longer supported, instead Value and Statement implement Serialize and Deserialize
  allowing users to use rmp-serde to serialize the AST types directly.


## 0.6.8

Changes:

* Moves the Lexer and Parser to keep track of line and column
* Updates StandardLoader to parse each module by itself for filename reporting

## 0.6.7

Fixes:

* Multiline strings were not properly lexing

## 0.6.6

Fixes:

* Comments and Labels weren't propagated to a few value types
* Null was not being parsed nor reserved

## 0.6.5

Version is bumping from a .0 -> .5 because there are no major changes in the language syntax but the lexer/parser has
been moved away from peg.

Changes:

* Implements a new token lexer using Logos
* New Recursive descent parser
* Improved multi-file handling as a single file in Multi mode
* Removes need for statements to be new-line delimited
* Commas are now optional in arrays and tables
* All assignment identifiers can be an identifier (no collision with a keyword) or a string
* Reserves keywords for future use

## 0.6.0

Features:

* Reworks value structure to track id, label and even value type in a better way.
* Adds typing system and type hint to the language
* Upgrades macro resolution to a full scope system

## 0.5.2

Fixes:

* Switch StandardLoader to use BTreeMap so macro resolution doesn't indeterministically fail on module order pre-merge
  or append

## 0.5.1

Fixes:

* read_dir causing inconsistent resolution of macros with multiple configuration files

## 0.5.0

Features:

* Added Module type

> Files in barkml are now loaded into a parent Value known as a Module. This allows better handling for multiple
> file loaders

* Moved definition of a configuration loader to a trait
* Added Semantic Version and Version Requirement types and support

> Users can now define standard semantic versions in their configuration files as well
> as version requirements

## 0.4.1

Fixes:

* Fix import issue in lib.rs
* Remove unimplemented feature

## 0.3.0 - 0.4.0

Features:

* Added config loader construct

> The Loader builder gives users the ability to load configuration files with more control. It also
> introduces an ability to load and merge multiple configuration files in a directory

* Added precision numeral values

> Users can now define a precision for integers using standard rust suffixes (u8, i8, u16, i16, u32, i32, u64, i64).
> Users can also define a precision for floating point numbers using suffixes (f32, f64)

Changes:

* Merged Value and Statement into one enum for easier use

## 0.2.0 - Binary encoding support

Added feature 'binary' that implements a conversion layer
to encode BarkML into a binary representation through the use
of `MessagePack`

## 0.1.0 - Initial Release

Initial release of barkml.

