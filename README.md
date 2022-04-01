# Heavily-Commented Solana Exchange Booth

This is a solution to the Exchange Booth project given by Jarry Xiao at the Solana Bootcamp. [See the project spec](https://github.com/jarry-xiao/solana-bootcamp-lectures/blob/master/project_specs/Exchange_Booth_Program_Spec.pdf) as well as [Jarry's design discussion on YouTube](https://www.youtube.com/watch?v=CeODeyposD0).

The code is __not__ production-ready and is only meant to demonstrate the programming model of [Anchor](https://book.anchor-lang.com/chapter_1/what_is_anchor.html), *the Ruby on Rails of Solana*.

## Running tests

* [Install Rust, Solana, Yarn, and Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#installing-dependencies)
* `anchor test`

The test suite will run on your local machine using a test validator whose state isn't saved between runs.
