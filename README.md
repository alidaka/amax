# Quickstart
```
make install
make run
```

# to do
- sign in
- create event
- post question
- upvote/downvote
- comment on questions
- edit post
- pseudonymous identity?

# stacks used
Don't love using all of these... any way to reduce footprint? At least they don't have dependencies on yet other languages or farmeworks, mostly.

### Ops
Makefile
Procfile

### Backend
Rust
Cargo

### Frontend
Elm
Yarn
Webpack

### Steup
```
> mkdir backend
> dotnet new webapi --name amax --output backend

> # install nvm

> mkdir frontend
> pushd frontend
> npm create svelte amax
> mv amax/* amax/.* . && rmdir amax
```

### Development
```
> dotnet new tool-manifest
> dotnet tool install dotnet-ef

> brew install postgresql
> /usr/local/opt/postgresql@14/bin/createuser -s postgres
> brew install pgadmin4 --cask

> foreman start
```

### Deployment
1. npm run build
2. dotnet build
  1. make sure this gets the `dist` from frontend build

