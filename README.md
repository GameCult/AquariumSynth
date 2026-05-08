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

`export_faust` writes `.dsp` source files under `target/faust`. The crate does
not bundle Faust yet; install a Faust compiler or libfaust in the host toolchain
when we are ready to turn these sources into C++, C#, LLVM, or WASM. Use
`validate_faust_source_with_command` to compile-check emitted source when
`faust` is available.

## Direction

- Keep patches deterministic and cheap enough for interactive UI feedback.
- Keep the Rust patch language as the authoring, analysis, and prototyping
  layer; emit Faust source when the graph needs a production DSP backend.
- Treat agent chirps and control responses as stateful signals, not loose sound
  effects.
- Preserve analysis hooks so visual systems can map low-frequency energy into
  Grid waves and higher octaves into finer surface vibration.
