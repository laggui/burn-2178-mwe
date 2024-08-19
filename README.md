# Matmul Web MWE

Matmul operation returns zero on web with wgpu.

## Reproduction Steps

> **Important**  
> The `Cargo.toml` assumes your burn dev repository to be at the [parent level of this repository folder](./Cargo.toml#L15) (`../burn/crates/burn`). Modify accordingly.

1. Clone this repo

```sh
git clone https://github.com/laggui/burn-2178-mwe.git
```

2. Build

```sh
./build-for-web.sh
```

3. Run

```sh
./run-server.sh
```

4. Open your browser at [`http://localhost:8000/`](http://localhost:8000/) and check the console logs.
The correct computation should display:
```
Matmul result [2, 2]:
[10.0, 13.0, 22.0, 29.0]
```