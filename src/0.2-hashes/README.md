# Commit hashes

So far, each commit we make is stored under a folder with an incrementally
increasing version number, and we store a reference to the maximum version
in the file `HEAD`. Each time we need to create a commit folder, we simply
increment the `HEAD`.

```
───1───2───3───4───5───►

HEAD: 5
```

This works great. In fact, it works absolutely amazingly. Until, that is, any
other person is involved in your project in any way at all.

Imagine this scenario.

```
Eleanor                      Samira      
───1───2───3───4───5───►     ───1───2───3───4───5───6───►

HEAD: 5                      HEAD: 6
Uncommitted changes
```

Eleanor and Samira are both disconnected from each other at the moment. Once
Eleanor goes to make her commit, as far as her local repository knows, `HEAD` is
currently at 5, so the new commit should be named 6. However, when they both get
together to discuss the project, this will probably cause confusion.

To avoid this, you could have them connect to a central repository which knows
the real `HEAD` at all times when a commit is being attempted, so that at every
point both local repositories never have any naming conflicts. And this is,
in fact, a reasonable solution that many version control systems before Git,
such as CVS, used. However, distributed version control systems like Git take a
different approach.

Since a commit, fundamentally, is nothing more than a snapshot of the state of
the repository at the time of the commit, we could simply use the contents of
those files as a unique identifier of the commit. In reality, of course, those
are usually far too long to be practical. But there exist certain functions
\- called hash functions - that take some data and produce an identifier that
uniquely corresponds to it, within a certain guaranteed amount of characters.
These functions are one-way, of course, otherwise they'd be absurdly efficient
compression algorithms, but they do enable us to know for certain that two
pieces of data with the same hash are the same and vice-versa.

Git uses the SHA-1 hash function for this, but that has since been proven to be
[insecure](https://security.googleblog.com/2017/02/announcing-first-sha1-collision.html),
and it's in the process of migrating to a newer and substantially more secure
version, SHA-256. For our purposes, we'll use SHA-256 from the beginning to make
things clear.

Nothing about our commit process has really changed, but when we name our commit
folder, we use the SHA-256 hash of the total contents of the working directory
(including any files in subdirectories, sub-sub-directories, and so on) instead
of a simple number, and update the `HEAD` file accordingly.

So the complicated identifier that you've see in commands like `git log` and
may have used in `git checkout`? That's simply a hash of the contents of the
commit... mostly. There are some caveats and issues you may have spotted with
this idea, which we'll address next.
