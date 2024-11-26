# co-colo-rs

A clone of [commit-colors](https://github.com/sparkbox/commit-colors) written in Rust.

# Adding co-colo-rs to a specific repository

Create a file named `post-commit` inside the `.git/hooks` of your
repository with the following content and ensure that it's executable:

```sh
#!/bin/bash
commit-colors $(git rev-parse HEAD)
```

# Adding co-colo-rs to all you repositories

Put the `post-commit` file generated above to a directory of your
choice, then run this command:

```sh
git config --global core.hooksPath /path/to/my/centralized/hooks
```

You can follow [these instructions][so] if you have an older version
of git or want to know more details.

[so]: https://stackoverflow.com/questions/2293498/applying-a-git-post-commit-hook-to-all-current-and-future-repositories
