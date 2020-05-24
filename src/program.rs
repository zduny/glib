use glium::backend::Facade;
use glium::Program;
use include_dir::*;
use lazy_static::*;
use linked_hash_set::LinkedHashSet;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct ShaderFile {
    required: Vec<String>,
    rest: String,
}

impl ShaderFile {
    fn new(contents: &str) -> ShaderFile {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?m)^#require <(?P<path>[a-zA-Z0-9/\-_]+)>\r?$").unwrap();
        }

        let required = RE
            .captures_iter(contents)
            .map(|capture| capture[1].to_string())
            .collect();

        let rest = RE.replace_all(contents, "").trim().to_string();

        ShaderFile { required, rest }
    }
}

fn get_file_from_dir<'a>(dir: &'a Dir<'a>, try_paths_in_order: &[&str]) -> Option<File<'a>> {
    for path in try_paths_in_order {
        if let Some(file) = dir.get_file(path) {
            return Some(file);
        }
    }

    None
}

struct ChunksCache<'a> {
    chunks_directory: &'a Dir<'a>,
    chunks_cache: HashMap<String, ShaderFile>,
    glsl_version: String,
}

impl<'a> ChunksCache<'a> {
    fn new(chunks_directory: &'a Dir, glsl_version: &'a str) -> ChunksCache<'a> {
        ChunksCache {
            chunks_directory,
            chunks_cache: HashMap::new(),
            glsl_version: glsl_version.replace(" ", ""),
        }
    }

    fn get_chunk(&mut self, chunk_path: String) -> ShaderFile {
        let chunks_directory = self.chunks_directory;
        let glsl_version = &self.glsl_version;

        self.chunks_cache
            .entry(chunk_path.to_string())
            .or_insert_with(|| {
                let full_name = format!("{}.{}.glsl", chunk_path, glsl_version);
                let full_name_no_version = format!("{}.glsl", chunk_path);
                let full_name_functions = format!("functions/{}.{}.glsl", chunk_path, glsl_version);
                let full_name_functions_no_version = format!("functions/{}.glsl", chunk_path);

                let file = get_file_from_dir(
                    chunks_directory,
                    &[
                        &full_name,
                        &full_name_no_version,
                        &full_name_functions,
                        &full_name_functions_no_version,
                    ],
                );

                match file {
                    None => panic!("Chunk \"{}\" not found!", chunk_path),
                    Some(file) => ShaderFile::new(file.contents_utf8().unwrap()),
                }
            })
            .clone()
    }
}

fn collect_requirements<'a>(
    chunks_cache: &mut ChunksCache,
    required: &mut LinkedHashSet<String>,
    shader_file: &'a ShaderFile,
) {
    shader_file.required.iter().for_each(|requirement| {
        if !required.contains(requirement) {
            required.insert(requirement.to_string());
            let required_chunk = chunks_cache.get_chunk(requirement.to_string());
            collect_requirements(chunks_cache, required, &required_chunk);
        }
    });
}

fn append_chunks(chunks_cache: &mut ChunksCache, program: &mut String, chunks: &[String]) {
    for chunk_path in chunks.iter() {
        let chunk_file = chunks_cache.get_chunk(chunk_path.to_string());

        program.push_str(&chunk_file.rest);
        program.push_str(&"\n\n");
    }
}

fn create_program_part(
    chunks_cache: &mut ChunksCache,
    default_requirements: &[&str],
    shader_file: &ShaderFile,
    glsl_version: &str,
) -> String {
    let mut requirements = LinkedHashSet::new();
    for requirement in default_requirements {
        requirements.insert(requirement.to_string());
    }
    collect_requirements(chunks_cache, &mut requirements, shader_file);

    lazy_static! {
        static ref INCLUDE_ORDER: Vec<&'static str> = vec!["attributes", "uniforms", "structs"];
    }

    let mut program = String::new();
    program.push_str(&format!("#version {}\n\n", glsl_version));

    let mut required: Vec<String> = requirements.iter().map(|r| r.to_string()).collect();
    for group in INCLUDE_ORDER.iter() {
        let (current, rest): (Vec<String>, Vec<String>) = required
            .drain(..)
            .partition(|requirement| requirement.starts_with(&(group.to_string() + "/")));

        append_chunks(chunks_cache, &mut program, &current);
        required = rest;
    }
    append_chunks(chunks_cache, &mut program, &required);

    program.push_str(&shader_file.rest);

    program
}

fn load_program<F: Facade>(
    facade: &F,
    materials_directory: &Dir,
    chunks_cache: &mut ChunksCache,
    material_name: &str,
    glsl_version: &str,
) -> Program {
    let vertex_file_path = format!("{}/vert.{}.glsl", material_name, glsl_version);
    let vertex_file_path_no_version = format!("{}/vert.glsl", material_name);

    let vertex_file = get_file_from_dir(
        materials_directory,
        &[&vertex_file_path, &vertex_file_path_no_version],
    );
    if vertex_file == None {
        panic!(
            "No vertex shader file found for \"{}\" material!",
            material_name
        );
    }

    let vertex_file_contents = vertex_file.unwrap().contents_utf8().unwrap();

    let vertex_file = ShaderFile::new(vertex_file_contents);
    let vertex_part = create_program_part(
        chunks_cache,
        &["attributes/common", "uniforms/common"],
        &vertex_file,
        glsl_version,
    );

    let fragment_file_path = format!("{}/frag.{}.glsl", material_name, glsl_version);
    let fragment_file_path_no_version = format!("{}/frag.glsl", material_name);

    let fragment_file = get_file_from_dir(
        materials_directory,
        &[&fragment_file_path, &fragment_file_path_no_version],
    );

    if fragment_file == None {
        panic!(
            "No fragment shader file found for \"{}\" material!",
            material_name
        );
    }
    let fragment_file_contents = fragment_file.unwrap().contents_utf8().unwrap();

    let fragment_file = ShaderFile::new(fragment_file_contents);
    let fragment_part = create_program_part(
        chunks_cache,
        &["uniforms/common"],
        &fragment_file,
        glsl_version,
    );

    let program = Program::from_source(facade, &vertex_part, &fragment_part, None);

    match program {
        Ok(program) => program,
        Err(error) => panic!(
            "{}\nSource code:\n\nVertex:\n{}\n\n\nFragment:\n{}\n\n",
            error, vertex_part, fragment_part
        ),
    }
}

pub struct ProgramsCache {
    programs: HashMap<String, Rc<Program>>,
}

impl ProgramsCache {
    pub fn new<F: Facade>(
        facade: &F,
        glsl_version: &str,
        chunks_directory: &Dir,
        material_directory: &Dir,
    ) -> ProgramsCache {
        let mut chunks_cache = ChunksCache::new(&chunks_directory, glsl_version);

        let mut cache = HashMap::new();
        for directory in material_directory.dirs().iter() {
            let program_name = directory.path().file_name().unwrap().to_str().unwrap();

            let program = load_program(
                facade,
                &material_directory,
                &mut chunks_cache,
                program_name,
                glsl_version,
            );

            cache.insert(program_name.to_string(), Rc::new(program));
        }

        ProgramsCache { programs: cache }
    }

    #[allow(dead_code)]
    pub fn get_program_names(&self) -> Vec<&String> {
        self.programs.keys().collect()
    }

    pub fn get_program(&self, name: &str) -> Option<Rc<Program>> {
        match self.programs.get(name) {
            Some(program) => Some(Rc::clone(program)),
            None => None,
        }
    }
}
