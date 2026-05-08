# Faust Backend

Aquarium Synth keeps the patch script, graph structs, fast Rust renderer, and
analysis metrics as the authoring layer. Faust is the backend target for the
same graph when we want generated DSP in C++, C#, LLVM, WASM, plugins, or other
host shapes without hand-porting every oscillator.

## Current Export Surface

- `export_patch_to_faust(&SynthPatch, FaustExportOptions)` emits `.dsp` source.
- `export_script_to_faust(&str, FaustExportOptions)` parses patch script first,
  then emits Faust source.
- `FaustExport` returns the generated source plus warnings for partial lowerings.
- `find_faust_command` finds `faust` on PATH or the default Windows installer
  location.
- `validate_faust_source` and `validate_faust_source_with_command` shell out to
  an installed Faust compiler and return status, stdout, and stderr.
- `compile_faust_source` and `compile_faust_source_with_command` write generated
  Faust backend output such as C, C++, or Rust source.
- `examples/export_faust.rs` writes sample `.dsp` files to `target/faust`.

The first lowering covers oscillators, envelopes, repeat age, arpeggio switches,
pitch motion, vibrato, FM index, sample-hold and periodic modulators, noise mix,
drive, wavefolding, low/high-pass filtering, approximate low-pass resonance,
phaser delay, formants, tremolo, patch gain, soft clipping, stereo duplication,
and patch/voice modulators.

When Faust is installed, the test matrix compile-checks every built-in classic
SFXR, 808, FM bell, and wobble bass primitive script through the real Faust
compiler.

## Partial Lowerings

- Low-pass resonance is mapped to Faust `fi.resonlp`, which is useful but not an
  exact clone of the Rust renderer's one-pole SFXR-style damping.
- Sample-hold randomness comes from Faust noise latched by an oscillator reset
  clock, not the Rust renderer's seeded hash slots.

Compatibility caveats should stay visible in this file or the public export
warnings. The backend should confess where it is soft, because pretending is how
code learns to lie.

## Next Steps

- Add a C# or C++ emission example for the future Vortice engine path.
- Build a parity harness that renders Rust output and compiled Faust output,
  then feeds both into the existing log-mel, envelope, and feature comparison.
- Replace placeholder sample-hold, phaser, and repeat lowerings with explicit
  Faust implementations.
