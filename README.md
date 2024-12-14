# Supererpo
A Blazingly fast monorepo/project tool, made for managing builds and runs. Giving your repo superpowers.
## Config
Every Superepo monorepo or project must use the config which must be located at the root project dir.
Here's a Example Config:
```toml
[monorepo]
name = "engine"
git = "git@github.com:GrandEngineering/superepo.git"
run = "cargo build && cargo run --bin server"
opt_run = "cargo build --release && cargo run --bin server --release"
build = "cargo build"
opt_build = "cargo build --release"

[[monorepo.libs]]
name = "enginelib"
build = "cargo build"
opt_build = "cargo build --release"

[[monorepo.libs]]
name = "engine_core"
build = "cargo build"
opt_build = "cargo build --release"

[[monorepo.bins]]
name = "server"
build = "cargo build"
opt_build = "cargo build --release"
run = "cargo run --bin server"
opt_run = "cargo run --bin server --release"

[[monorepo.bins]]
name = "client"
build = "cargo build"
opt_build = "cargo build --release"
run = "cargo run --bin client"
opt_run = "cargo run --bin client --release"
```
### Monorepo Table
| Field     | Obligatory | Type   | Description                                                   |
|-----------|------------|--------|---------------------------------------------------------------|
| name      | True       | String | The name for the current project.                             |
| git       | True       | String | The current project git repo.                                 |
| build     | True       | String | The command to build the project.                             |
| opt_build | False      | String | The command to build a release version of the project.        |
| run       | False      | String | The command to run the project.                               |
| opt_run   | False      | String | The Command to run a release version of the project.          |
| libs      | False      | Lib    | An Array of Lib tables to configure libraries of the project. |
| bins      | False      | Bin    | An Array of Bin tables to configure binaries of the project.  |
#### Lib Tables
| Field     | Obligatory | Type   | Description                                            |
|-----------|------------|--------|--------------------------------------------------------|
| name      | True       | String | Name of the library.                                   |
| build     | True       | String | The command to build the library.                      |
| opt_build | False      | String | The command to build a release version of the library. |
#### Bin Tables
| Field     | Obligatory | Type   | Description                                           |
|-----------|------------|--------|-------------------------------------------------------|
| name      | True       | String | Name of the binary.                                   |
| build     | True       | String | The command to build the binary.                      |
| opt_build | False      | String | The command to build a release version of the binary. |
| run       | True       | String | The command to run the binary.                        |
| opt_run   | False      | String | The Command to run a release version of the binary.   |
## Usage
To build or run using a release version you can use the `--release` flag.
### Run
To run the project, you can use the following command:
```bash
superepo run
```
which will run the default run command specified in the config.
Or you can run a specific binary by using:
```bash
superepo run <binary_name>
```
### Build
To build the project, you can use the following command:
```bash
superepo build
```
which will build the default build command specified in the config.
Or you can build a specific binary by using:
```bash
superepo build <binary_name/lib_name>
```
