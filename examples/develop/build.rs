#![feature(osstring_ascii)]

use std::{path::PathBuf, env, fs, path};

const FOLDER_INPUT:  &'static str = "assets/";
const FOLDER_OUTPUT: &'static str = "output/";

fn main() {
    convert_glsl();
}

fn convert_glsl() {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    
    let root_dir = env::current_dir().unwrap();

    let mut working_dir = root_dir.clone();
    working_dir.push(FOLDER_INPUT);
    
    if !path::Path::exists(&working_dir) {
        println!("Working directory does not exist: {}", working_dir.to_string_lossy().to_owned());
        return;
    }

    let mut output_dir = root_dir.clone();
    output_dir.push(FOLDER_OUTPUT);
    
    process_directory(working_dir.clone(), {
        let working_dir = &working_dir;
        let output_dir = &output_dir;
        &mut move |source_file| { 
            let mut destination_file = reprefix_path(&source_file, working_dir, output_dir).expect("Could not prefix path");
            destination_file.set_extension("spv");
            let destination_dir  = destination_file.parent().expect("Could not get folder of source file");

            fs::create_dir_all(destination_dir).expect("Failed to create directory");

            let source = fs::read_to_string(&source_file).expect("Failed to read file");

            let mut tmp = source_file.clone();
            tmp.set_extension("");
            let shader_type = tmp.extension().expect("Not shader extension").to_str().expect("Invalid symbols in shader type ext").to_ascii_uppercase();

            let shader_type = match shader_type.as_str() {
                "VERT" => shaderc::ShaderKind::Vertex,
                "FRAG" => shaderc::ShaderKind::Fragment,
                "COMP" => shaderc::ShaderKind::Compute,
                "GEOM" => shaderc::ShaderKind::Geometry,
                "TESC" => shaderc::ShaderKind::TessControl,
                "TESE" => shaderc::ShaderKind::TessEvaluation,
                _ => panic!("Unrecognized shader type: {}", shader_type),
            };

            let binary_result = compiler.compile_into_spirv(
                source.as_str(), 
                shader_type,
                "shader.glsl", 
                "main", 
                Some(&options)
            ).unwrap();

            fs::write(&destination_file, binary_result.as_binary_u8()).expect("Failed to write SPIR-V shader to destination");
            
            println!("cargo:rerun-if-changed={}",      source_file.to_string_lossy());
            println!("cargo:rerun-if-changed={}", destination_file.to_string_lossy());
        }
    });
}

fn process_directory(directory: path::PathBuf, process: &mut impl FnMut(path::PathBuf)) {

    let directory_iter = fs::read_dir(directory.clone());
    if directory_iter.is_err() {
        println!("Failed to read directory: {}", directory.to_string_lossy().to_owned());
        return;
    }

    for entry in directory_iter.unwrap() {
        let entry = entry.expect("Could not read directory contents");
        let path  = entry.path();
        let filetype = entry.file_type().expect("Failed to get filetype");

        // TODO use metadata on entry to cut down on copies
        // TODO can we move process_directory to entry?

        if filetype.is_dir() || (filetype.is_symlink() && fs::symlink_metadata(path.clone()).expect("Faild to get symlink metadata").is_dir()) {
            process_directory(path.clone(), process);
        } else {
            match path.extension().and_then(std::ffi::OsStr::to_str) {
                Some("glsl") => process(path),
                _ => {},
            }
        }
    }
}

fn reprefix_path(path: &PathBuf, prefix_cur: &PathBuf, prefix_new: &PathBuf) -> Option<PathBuf> {
    let mut result = prefix_new.clone();
    result.push(path.strip_prefix(prefix_cur).ok()?);
    return Some(result);
}