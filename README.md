# rmkit

rmkit is a toolkit set for [RMK keyboard firmware](https://github.com/haobogu/rmk).

Now rmkit can be used to generate RMK project directly from `keyboard.toml` and `vial.json`, or interactively.

## Usage

1. Install rmkit:
   
   If you have Rust installed in your machine, you can use Cargo to install rmkit

    ```shell
    cargo install rmkit

    # If you have cargo-binstall, you can use it to speedup the installation:
    cargo binstall rmkit
    ```
    
   rmkit also provides install script that you can use:

   ```shell
    # macOS/linux
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/haobogu/rmkit/releases/download/v0.0.1/rmkit-installer.sh | sh

    # Windows(powershell)
    powershell -ExecutionPolicy ByPass -c "irm https://github.com/haobogu/rmkit/releases/download/v0.0.1/rmkit-installer.ps1 | iex"
   ```

2. Create RMK project from `keyboard.toml` and `vial.json`:

    ```
    rmkit create --keyboard-toml-path keyboard.toml --vial-json-path vial.json
    ```

3. Or, you can create RMK project from project template

    ```
    rmkit init
    ```

    The available project template can be found at [rmk-template](https://github.com/HaoboGu/rmk-template)

4. Migrate from QMK/ZMK/Vial to RMK:

    ```
    rmkit migrate
    ```

    This will guide you through an interactive migration. You can also pass all options via CLI for non-interactive use.

### Migrate from QMK

You'll need your QMK `keyboard.json` (or `info.json`). Optionally, export a `keymap.json` from [QMK Configurator](https://config.qmk.fm) to migrate your keymap, and provide a `via.json` for Vial support.

```shell
# Interactive
rmkit migrate --from qmk

# Non-interactive
rmkit migrate --from qmk \
  --config keyboard.json \
  --keymap keymap.json \
  --via-json via.json \
  --chip rp2040 \
  --name my_keyboard \
  --target-dir ./my_keyboard
```

### Migrate from ZMK

You'll need your ZMK `.keymap` file. Optionally provide the `.conf` file for keyboard name and split/BLE settings.

```shell
# Interactive
rmkit migrate --from zmk

# Non-interactive
rmkit migrate --from zmk \
  --config corne.keymap \
  --conf corne.conf \
  --chip nrf52840 \
  --split true \
  --name my_corne \
  --target-dir ./my_corne
```

### Migrate from Vial

If you have a `vial.json` exported from Vial, you can use it to generate a project skeleton.

```shell
rmkit migrate --from vial --config vial.json --chip nrf52840 --name my_keyboard
```

### After migration

- Search for `TODO` in the generated `keyboard.toml` to find sections that need manual input (pin definitions, split config, etc.)
- Review the migration warnings — they list keycodes that couldn't be automatically mapped
- Run `cargo build --release` to build your firmware
