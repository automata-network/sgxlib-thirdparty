# SGX Thirdparty Library

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)

This repository contains a large number of third-party libraries, used for various functionalities and development purposes. One of its main features is to provide a unified interface, making it convenient to use a single piece of code simultaneously in both std and tstd environments.

### Unified Interface
The repository offers a unified interface that allows developers to write code that can seamlessly operate within both standard (std) and customized (tstd) environments. This streamlines the development process and enhances code reusability, making it easier to maintain and deploy projects across various platforms.

### Benefits
Code Reusability: Write once and run in both std and tstd environments without modification.
Ease of Use: Simplified development through a common interface that abstracts underlying complexities.
Cross-Platform Compatibility: Ensures that your code is compatible with different platforms that adhere to std and tstd specifications.

### License Declaration
Please be aware that each third-party library or package within this repository may have its own distinct license. These licenses may include different restrictions and conditions, so it is essential to thoroughly review the accompanying license documentation for any particular library or package before using it.

## Usage

```toml
[features]
default = ["std"]
std = ["serde/std", "serde_json/std", "threadpool/std"]
tstd = ["serde/tstd", "serde_json/tstd", "threadpool/tstd"]

[dependencies]
serde = { git = "https://github.com/automata-network/sgxlib-thirdparty", default-features = false }
serde_json = { git = "https://github.com/automata-network/sgxlib-thirdparty", default-features = false }
threadpool = { git = "https://github.com/automata-network/sgxlib-thirdparty", default-features = false }
```