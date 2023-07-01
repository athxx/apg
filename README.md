# Rust + Axum + Prisma

A template for creating services in Rust using `Axum`, `Prisma` and `async-graphql`. This uses the super cool `Prisma`.

## Getting Started

You'll want to have `cargo-watch` installed for the best DX, however it isn't required.

```bash
cargo install cargo-watch
```

Then you can run the `cargo-watch -x run` command to watch for changes and automatically rebuild the project.

## Prisma

To set up prisma, run:

1. `cargo prisma generate` create the `prisma.rs` file, see `prisma/schema.prisma`
2. `cargo prisma db push` sync the code and database

```bash
cargo prisma generate # outputs client to src/prisma.rs
cargo prisma db push # outputs sqlite db to prisma/dev.db (specified in schema.prisma)
cargo seed # seeds the database with some data (unimplemented, create a seed based on your needs)
```

For more in-depth information about the prisma client, see the [Prisma Client Rust Docs](https://github.com/Brendonovich/prisma-client-rust/tree/main/docs).

## Run the Server

To run the server, run:

```bash
cargo run # or cargo-watch -x run
```

## GraphQL Playground

Go to [localhost:8080/api/graphql](http://localhost:8080/api/graphql) to see the playground. You can see the schema and the docs, but a few examples would be:

```graphql
# Create user
mutation {
  createUser(input: { displayName: "oromei" }) {
    id
  }
}

# Delete user
mutation {
  deleteUser(id: "cf5a7caf-6e18-4759-bbfc-8da15978dec9") {
    id
  }
}

# Create post
mutation {
  createPost(
    input: {
      content: "Woah there!"
      userId: "5ab80953-c38c-4ec8-8b4b-3ecc4bc1196f"
    }
  ) {
    id
    content
    user {
      displayName
    }
  }
}
# Get one user
query {
  getUser(id: "cf5a7caf-6e18-4759-bbfc-8da15978dec9") {
    id
    displayName
  }
}

# Get all users
query {
  getUsers {
    id
    displayName
  }
}

# Get one post
query {
  getPost(id: "d97920e2-83fb-44d0-81c1-93e13b69b7ff") {
    id
    content
    user {
      id
      displayName
    }
  }
}

# Get all posts
query {
  getPosts {
    id
    content
    user {
      displayName
    }
  }
}
```

## Future

I will try to make it become a more and more complete

## Notes

This template uses Axum, but the bulk of the setup is for async_graphql + prisma. You should be able to easily swap out Axum for your preferred framework (e.g. Rocket, actix, etc).

The simple use of async_graphql means that queries are done in a less efficient manner than could be, since fetching relations using `with` is never utilized and relations are loaded separately. Additionally, dataloader is not utilized because I can't be bothered.
