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

You'll need:
- `keyboard.json` or `info.json` — find it in your QMK keyboard directory (e.g. `qmk_firmware/keyboards/<your_keyboard>/`)
- (Optional) `keymap.json` — export from [QMK Configurator](https://config.qmk.fm) to migrate your keymap
- (Optional) `via.json` — if your keyboard supports VIA/Vial

```shell
# Interactive — just follow the prompts
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

You'll need:
- `.keymap` file — from your ZMK config repo (e.g. `zmk-config/config/<keyboard>.keymap`)
- (Optional) `.conf` file — for keyboard name and split/BLE settings

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

If you have a `vial.json` (exported from Vial or from your keyboard's source repo), you can generate a project skeleton with the correct matrix dimensions and physical layout. Note that Vial JSON does not contain keymap data, so you'll need to fill in the keymap manually.

```shell
rmkit migrate --from vial \
  --config vial.json \
  --chip nrf52840 \
  --name my_keyboard \
  --layers 2 \
  --target-dir ./my_keyboard
```

### After migration

1. `cd` into the generated project directory
2. Search for `TODO` in `keyboard.toml` — fill in pin definitions, split config, etc.
3. Review the migration warnings — they list keycodes that couldn't be automatically mapped and suggest alternatives
4. Run `cargo build --release` to build your firmware

> **Tip:** Some QMK/ZMK features (Mod-Tap, Tap Dance, RGB controls, etc.) cannot be mapped automatically. The migration will warn you about these — check the [RMK documentation](https://haobogu.github.io/rmk/) for how to configure them in RMK.
