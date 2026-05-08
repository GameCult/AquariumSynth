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
- `examples/export_faust.rs` writes sample `.dsp` files to `target/faust`.

The first lowering covers oscillators, envelopes, pitch motion, vibrato,
FM index, noise mix, drive, wavefolding, low/high-pass filtering, approximate
low-pass resonance, formants, tremolo, patch gain, soft clipping, stereo
duplication, and patch/voice modulators.

## Partial Lowerings

- Repeat uses absolute Faust time today, so it does not reset oscillator state
  exactly like the Rust renderer.
- Phaser is reported but not lowered yet.
- Arpeggio is reported as approximate.
- Sample-hold modulation uses smoothed noise as a placeholder until we lower a
  clocked hold primitive.

These warnings are part of the public export result on purpose. The backend
should confess where it is soft, because pretending is how code learns to lie.

## Next Steps

- Add a C# or C++ emission example for the future Vortice engine path.
- Build a parity harness that renders Rust output and compiled Faust output,
  then feeds both into the existing log-mel, envelope, and feature comparison.
- Replace placeholder sample-hold, phaser, and repeat lowerings with explicit
  Faust implementations.
