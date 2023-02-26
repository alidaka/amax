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
> npm install
> npm run dev
```

### Deployment
1. npm run build
2. dotnet build
  1. make sure this gets the `dist` from frontend build

