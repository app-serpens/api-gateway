# API Gateway

API Gateway is a project for Serpens Group which responsibility is to authenticate all incoming requests an checking its validity to provide usage of other microservices.

## Installation

Make sure to have [Rust](https://www.rust-lang.org/pt-BR) installed (currently on version 1.70.0).

You can follow Rust's instructions of installation [here](https://www.rust-lang.org/pt-BR/tools/install).

### Linux

If you are running on any Linux distro (Ubuntu 22.04 in my case) run the command:

```bash
sudo apt install build-essential
```

This will download essential libs for GNU GCC C/C++ compilers and it is required to run this project.

### Windows

If you are running on any version of [Windows](https://www.microsoft.com/pt-br/windows/?r=1) make sure to download the Microsoft C++ Build Tools from Visual Studio, some instructions can be found [here](https://stackoverflow.com/questions/40504552/how-to-install-visual-c-build-tools).

---

After completing installation run:

```bash
cargo build
```

This command will build the target folder, which contain the compiled code for this project.

## Usage

For this branch (test) we have 2 binaries to be run:

- auth-server
- auth-client

First, run the auth-server binary by typing the command:

```bash
cargo run --bin auth-server
```

You should see something like this:

![](/img/auth-server-run.png)

This means that the GRPC server is now running on port *50051* in your localhost.

Now you should open another command line tool and run:

```bash
cargo run --bin auth-client
```

This line should send a request to our previously started GRPC server and then the response should be this:

![](/img/auth-client-run.png)
