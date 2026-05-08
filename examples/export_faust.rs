use aquarium_synth::{
    FaustCompileOptions, FaustExportOptions, FaustTargetLanguage,
    WOBBLE_BASS_PRIMITIVE_GOLF_SCRIPTS, compile_faust_source, export_patch_to_faust,
    export_script_to_faust, presets, validate_faust_source,
};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let output_dir = PathBuf::from("target").join("faust");
    fs::create_dir_all(&output_dir)?;

    let pluck = export_patch_to_faust(
        &presets::aquarium_pluck(),
        FaustExportOptions {
            name: "aquarium_pluck".to_owned(),
            stereo: true,
        },
    )?;
    fs::write(output_dir.join("aquarium_pluck.dsp"), pluck.source)?;
    compile_if_possible(
        "aquarium_pluck",
        &fs::read_to_string(output_dir.join("aquarium_pluck.dsp"))?,
        &output_dir.join("aquarium_pluck.cpp"),
    )?;
    validate_if_possible(
        "aquarium_pluck",
        &fs::read_to_string(output_dir.join("aquarium_pluck.dsp"))?,
    )?;
    for warning in pluck.warnings {
        eprintln!("aquarium_pluck: {warning}");
    }

    let (name, script) = WOBBLE_BASS_PRIMITIVE_GOLF_SCRIPTS[0];
    let wobble = export_script_to_faust(
        script,
        FaustExportOptions {
            name: format!("aquarium_wobble_{name}"),
            stereo: true,
        },
    )?;
    fs::write(
        output_dir.join(format!("aquarium_wobble_{name}.dsp")),
        wobble.source,
    )?;
    compile_if_possible(
        &format!("aquarium_wobble_{name}"),
        &fs::read_to_string(output_dir.join(format!("aquarium_wobble_{name}.dsp")))?,
        &output_dir.join(format!("aquarium_wobble_{name}.cpp")),
    )?;
    validate_if_possible(
        &format!("aquarium_wobble_{name}"),
        &fs::read_to_string(output_dir.join(format!("aquarium_wobble_{name}.dsp")))?,
    )?;
    for warning in wobble.warnings {
        eprintln!("aquarium_wobble_{name}: {warning}");
    }

    println!("wrote Faust DSP files to {}", output_dir.display());
    Ok(())
}

fn compile_if_possible(
    name: &str,
    source: &str,
    output_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    if let Some(validation) = compile_faust_source(
        source,
        &FaustCompileOptions {
            language: FaustTargetLanguage::Cpp,
            output_path: output_path.clone(),
        },
    )? {
        if validation.success {
            eprintln!("{name}: wrote {}", output_path.display());
        } else {
            return Err(format!("{name}: Faust C++ export failed\n{}", validation.stderr).into());
        }
    }
    Ok(())
}

fn validate_if_possible(name: &str, source: &str) -> Result<(), Box<dyn Error>> {
    if let Some(validation) = validate_faust_source(source)? {
        if validation.success {
            eprintln!(
                "{name}: Faust validation passed with {}",
                validation.command
            );
        } else {
            return Err(format!("{name}: Faust validation failed\n{}", validation.stderr).into());
        }
    }
    Ok(())
}
