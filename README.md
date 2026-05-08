# Aquarium Synth

Procedural synth and audio-analysis module extracted from Epiphany Aquarium.

This crate keeps the audio work visible on its own timeline: SFXR ground-truth
checks, fast patch playback, modulation routing, voice-capacity probes, and the
early shape of agent interaction sound. It was born inside the Bevy prototype,
but it deserves a smaller room with less renderer dust in the vents.

## Run

```powershell
cargo test
cargo run --release --example voice_capacity
cargo run --example export_faust
```

`export_faust` writes `.dsp` and `.cpp` source files under `target/faust` when
Faust is installed. Use `validate_faust_source` or
`validate_faust_source_with_command` to compile-check emitted source, and
`compile_faust_source` to write generated backend code.

When Faust is installed, `cargo test` compile-checks the built-in SFXR, 808,
FM bell, and wobble patch families through the real compiler. With MSYS2
`gcc`/`libsndfile` available, the test suite also renders generated Faust audio
through `faust2sndfile` and compares it against the Rust renderer.

## Direction

- Keep patches deterministic and cheap enough for interactive UI feedback.
- Keep the Rust patch language as the authoring, analysis, and prototyping
  layer; emit Faust source when the graph needs a production DSP backend.
- Treat agent chirps and control responses as stateful signals, not loose sound
  effects.
- Preserve analysis hooks so visual systems can map low-frequency energy into
  Grid waves and higher octaves into finer surface vibration.
