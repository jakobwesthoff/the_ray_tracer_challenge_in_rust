# Notes about WASM experiment

This branch of the raytracer is a very hackish experiment. It is a first try to quickly build a WASM version of the raytracer code in order to utilize it within the a browser.

## Prerequisites

As a workaround on my M1 Mac I needed to install the wasm-opt binary manually and disable integrated support for it. It is available via homebrew:

```shell
brew rew install binaryen
```

## Usage

**Compile command:**

```shell
wasm-pack build --release && pushd pkg && wasm-opt -O3 -o out.wasm the_ray_tracer_challenge_bg.wasm && mv out.wasm the_ray_tracer_challenge_bg.wasm && popd
```

**Open it within a browser:**

Go into the `www` directory execute an `npm install` there.

After that start the dev server with `npm start` open `http://localhost:8080/` in your browser and see the magic happen :^)
