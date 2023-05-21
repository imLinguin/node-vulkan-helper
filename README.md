# vulkan-helper

This project was created using [napi](https://napi.rs)

## Overview

This library provides misc function for checking vulkan versions. And can be easily extended for fetching other data.

Currently there are two functions you can use.

`getInstanceVersion` - returns vulkan loader version `[major, minor, patch]`
`getPhisicalVersions` - reutrns array of devices and vulkan versions they support

## Installing vulkan-helper

You can install the project with yarn. In the project directory, run:

```sh
$ yarn
```

This fully installs the project, including installing any dependencies.

## Building vulkan-helper

If you have already installed the project and only want to run the build, run:

```sh
$ yarn build
```
