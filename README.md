# Rust-vs-Go

Most idiotic comparison in the whole world, building the same thing, twice, in different languages.

This repo contains the same API built using Rust and Go. **If I won't be ultra lazy, I'll write some utility software to performance test both approaches.**

## Design

```
/tweets
    GET: list last 50 tweets
    POST: create a new tweet
/tweets/:id
    GET: find a tweet by its ID
    DELETE: delete a tweet by its ID
/tweets/:id/likes
    GET: list all likes attached to a tweet
    POST: add +1 like to a tweet
    DELETE: add -1 like to a tweet
```

## Notes

As I don't know enough about Rust, I'm loosely following [this](https://docs.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1) tutorial.
