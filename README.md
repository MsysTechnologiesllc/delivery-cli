# Delivery CLI

The CLI for Chef Delivery. Written in Rust, super experimental, will probably hurt your kittens.

## Usage

Start using `delivery` by issuing the setup command:

```shell
$ delivery setup --user USER --server SERVER --ent ENTERPRISE --org ORGANIZATION --config-path /Users/adam
```

This will configure delivery to, by default, contact the delivery server at SERVER, with a default
ENTERPRISE and ORGANIZATION.

### Job

The Delivery CLI is going to also encompass the act of seting up a workspace,
configuring it to run, and then actually running a delivery job. The goal is:

#### To be able to run any delivery phase from the command line, as if your laptop was a build node.

As a developer, it's good best practice to verify that your code will work
locally before submitting it. What if we could validate that with the identical
behavior we would have on a build node in our pipeline?

You can run:

```bash
$ delivery job verify unit
Chef Delivery
Loading configuration from /Users/adam/src/opscode/delivery/opscode/delivery-cli
Starting job for verify unit
Creating workspace
Cloning repository, and merging adam/job to master
Configuring the job
Running the job
Starting Chef Client, version 11.18.0.rc.1
resolving cookbooks for run list: ["delivery_rust::unit"]
Synchronizing Cookbooks:
  - delivery_rust
  - build-essential
Compiling Cookbooks...
Converging 2 resources
Recipe: delivery_rust::unit
  * execute[cargo clean] action run
    - execute cargo clean
  * execute[cargo test] action run
    - execute cargo test

Running handlers:
Running handlers complete
Chef Client finished, 2/2 resources updated in 32.770955 seconds
```

Which will keep a persistent, local cache, and behave as a build node would.

This also has a delightful side effect, which is that anyone can use the delivery
cli to get the *job* behaviors of delivery, including integrating them in to existing
legacy solutions.

2) Make the setup and execution of the build job straightforward and easy
   to debug.

First we create the workspace directories, then we clone the project we are to
build, configure the Chef environent, and execute the job.

To setup a job, the delivery cli reads the `.delivery/config.json` file, and
looks for its `build_cookbook` parameter. It takes 3 forms:

#### From a local directory
```json
{
    "version": "1",
    "build_cookbook": {
      name: "delivery_rust",
      path: "cookbooks/delivery_rust"
    },
    "build_nodes": {
        "default"    : ["name:delivery-builder*"]
    }
}
```

#### From a Git source
```json
{
    "version": "1",
    "build_cookbook": {
      name: "delivery_rust",
      git: "ssh://..."
    },
    "build_nodes": {
        "default"    : ["name:delivery-builder*"]
    }
}
```

#### From a Supermarket
```json
{
    "version": "1",
    "build_cookbook": {
      name: "delivery_rust",
      supermarket: "https://supermarket.chef.io"
    },
    "build_nodes": {
        "default"    : ["name:delivery-builder*"]
    }
}
```

It will then retrieve the source, and execute a `berks vendor` on it. This fetches
any dependencies it may have. We then execute:

```bash
$ chef-client -z -j ../chef/dna.json -c ../chef/config.rb -r 'delivery_rust::unit'
```

3) Optimize things like the idempotence check for build node setup.

When dispatching the job, we check to see if we have executed the build node setup
recipe. We do the following:

* Check to see if we have run it before - if we haven't, execute it.
* If we have run it in the last 24 hours, check the guard
* If we have a guard, use it to determine if we should run again

We determine if we have run it before though, at the end of the build step, writing
out a cache with the checksum of the build cookbooks lib, recipes/default.rb, resource,
provider, file and template directories. If any of them has changed, we assume we need
to try again.

The cache has a deadline of 24 hours - when it passes, we run it again no matter what.

The guard statement is optional.

## Development

While the Rust Language is now moving towards 1.0, and things should begin to stabilize, follow-on releases sometimes introduce non-backwardly-compatable changes, which can break this build. Until Rust truly stabilizes, you'll need to install rust (the easiest way on mac):

```bash
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```bash

If this repo fails to build, using the instructions below, you might try:

```bash
$ cargo clean
$ cargo update
```bash

This may update the Cargo.lock file, which is currently checked in. If there are changes, they should likely be included in your CR.

If there are syntax or other errors, well, good luck!

## Build me

```bash
cargo build
```

## Test me

```bash
cargo test
```

## Develop me

Hack about, then:

```bash
cargo run -- review ...
```

Where "review" and friends are the arguments you would pass to the delivery cli.