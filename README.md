# AMD SMI Wrapper

A Rust wrapper around [`amd-smi-lib`](https://rocm.docs.amd.com/projects/amdsmi/en/latest/) for querying AMD GPU metrics programmatically.
This project is mainly used as a wrapper in order to implement a [Joule Profiler](https://github.com/joule-profiler/joule-profiler) metric source, thus many bindings will not be generated, see the [allowlist](amdsmi-sys/allowlist.rs).

---

## Requirements

| Dependency | Purpose |
|---|---|
| `amd-smi-lib` | Runtime library (always required) |
| `clang` | Only needed to regenerate bindings |

Install the library:

```bash
sudo apt install amd-smi-lib
```

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
amdsmi = { git = "https://github.com/joule-profiler/amd-smi-wrapper", tag = "v0.1.0" }
```

Pre-generated bindings are included for the latest major version, so no extra steps are needed for most users.

---

## Regenerating Bindings

If you need to regenerate bindings:

```bash
sudo apt install clang
```

Then build with the `AMDSMI_GENERATE_BINDINGS` environment variable set:

```bash
AMDSMI_GENERATE_BINDINGS= cargo build
```

The build script looks for `amdsmi.h` in this order:
1. Current directory
2. Parent directory (`../`)
3. ROCm installation (`/opt/rocm-*/include/amd_smi/` latest version)
If none of these work, place `amdsmi.h` in the current directory or a parent before building.

---

## Custom Library Path

The library is loaded at runtime via `libloading`. If `amd-smi-lib` is not installed in a standard path, set `AMDSMI_LIB_DIR` at runtime to point to the directory containing `libamd_smi.so`:

```bash
AMDSMI_LIB_DIR=/path/to/your/lib ./your-binary
```

No `LD_LIBRARY_PATH` or linker flags needed.

---

## Troubleshooting

### Cannot find `libamd_smi.so` at runtime

Make sure `amd-smi-lib` is installed, or set `AMDSMI_LIB_DIR` to the directory containing `libamd_smi.so`. The default search path is `/opt/rocm/lib`.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- **Issues**: [GitHub Issues](https://github.com/joule-profiler/amd-smi-wrapper/issues)
- **Authors**: [@jwoirhaye](https://github.com/jwoirhaye), [@FrancoisGib](https://github.com/FrancoisGib)