# Rust-vs-Go

Most idiotic comparison in the whole world, building the same thing, twice, in different languages.

This repo contains the same API built using Rust and Go. **If I won't be ultra lazy, I'll write some utility software to performance test both approaches.**

## Design

Tweet struct:

```
ID: uuid
Author: string
Body: string
Likes: int
```

API routes:

```
api/tweets
    GET: list last 50 tweets
    POST: create a new tweet
api/tweets/:id
    GET: find a tweet by its ID
    DELETE: delete a tweet by its ID
api/tweets/:id/likes
    GET: list all likes attached to a tweet
    POST: add +1 like to a tweet
    DELETE: add -1 like to a tweet
```

Database:

Postgres, but designed as plug-in, so we can change the db anytime.

## Notes

~~As I don't know enough about Rust, I'm loosely following [this](https://docs.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1) tutorial.~~ Turns out, this "tutorial" leaves out A TON of info...

More sources:

* [this](https://medium.com/swlh/build-your-first-rest-api-using-rust-language-and-actix-framework-8f827175b30f)
* [and this](https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/)

## Sources

* [Rust book](https://doc.rust-lang.org/book/)
