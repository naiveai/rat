# rat

`rat` is a simple version control system written in Rust designed primarily
to be easy to understand and modify, with as few external dependencies as is
practical. The final version implements a (very) minimal subset of git.

Git can often end up being a sort of magic box, even to many who use it on a
daily basis. `rat` is intended to demystify its internals so you can develop a
better mental model of git, and understand how elegant it really is.

Secondarily, it also demonstrates a fairly idiomatic Rust program, so you can
improve your understanding of how large Rust programs can be structured one step
at a time.

All the material here assumes a decent amount of familiarity with Rust,
but only a surface-level one with git. If you want to learn Rust, the
[book](https://doc.rust-lang.org/book/) is a great place to start, and the [git
tutorial](https://git-scm.com/docs/gittutorial) has the basics you need for git.

You can use the crate documentation for any version to find what you're
looking for, which is useful as the code gets larger.

```sh
cargo doc --open
```

Run `rat` with:

```sh
cargo run <COMMAND> <ARGUMENTS>
```

## Questions

If you have any questions about the source code or the
explanations surrounding it, feel free to ask them on the
[Discussions](https://github.com/naiveai/rat/discussions) page.

Before you ask about the name, though, it's just "Rust + Git" = "rit", but then
I thought of the absurd pun of calling the repo a "nest", so now it's called
`rat`.

## License

`rat` is intended to be a purely educational tool released freely into the
public domain wherever possible. See the [license](LICENSE) for the full
terms.

## Credits

`rat` owes a profound debt of inspiration and guidance from many sources, but
most especially [Write Yourself a Git](https://wyag.thb.lt/) and [The Git
Parable](https://tom.preston-werner.com/2009/05/19/the-git-parable.html).

## Contributing

If you find a problem at any point, whether technical or educational, I'd
appreciate if you opened an [issue](https://github.com/naiveai/rat/issues/new)
about it. Pull requests are also greatly appreciated, though make sure to
*explicitly* specify that you agree to have your contribution released into the
public domain.

By contributing, you agree to adhere to the
[Code of Conduct](CODE_OF_CONDUCT.md).
