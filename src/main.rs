mod dto;
mod utils;
mod visitors;

use clap::Parser;
use std::{fs::File, path::Path};

use crate::{
    dto::e1pmodule::E1pModule,
    visitors::{
        luawriter::LuaWriter, sexpressionwriter::SExpressionWriter,
        translationreplacer::TranslationReplacer, translationwriter::TranslationWriter,
    },
};

#[derive(clap::ValueEnum, Clone, Debug)]
enum ProgramMode {
    Disassemble,
    Assemble,
    ExtractTranslations,
    PackageTranslations,
    Info,
    Transpile,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Program mode
    #[clap(value_enum)]
    mode: ProgramMode,

    /// Directory to read/write the files from/to
    #[arg(short, long)]
    directory: String,

    /// Name of the module
    #[arg(short, long)]
    name: String,

    /// Output file for the disassembly output or input file for the assembly input
    #[arg(short, long)]
    file: Option<String>,

    /// Flag indicating whether the loaded module is a main module or a mod module
    #[arg(short, long, action)]
    main: bool,
}

fn main() {
    let args = Args::parse();
    let source_dir = Path::new(&args.directory);

    let module_type_str = if args.main { "main" } else { "mod" };
    println!(
        "Reading {} module from \"{}/{}.*\"",
        module_type_str, args.directory, args.name
    );

    match args.mode {
        ProgramMode::Transpile => {
            let mut module = E1pModule::load_from_directory(source_dir, &args.name, args.main);

            let outfile = args.file.unwrap_or("./transpilation.lua".into());
            let file = File::create(&outfile).expect("Failed to create output file");

            LuaWriter::write(file, &mut module);

            println!("Successfully transpiled to Lua \"{}\"", outfile);
        }
        ProgramMode::ExtractTranslations => {
            let mut module = E1pModule::load_from_directory(source_dir, &args.name, args.main);

            let outfile = args.file.unwrap_or("./strings.txt".into());
            let file = File::create(&outfile).expect("Failed to create output file");

            let mut extractor = TranslationWriter::new(file);
            module
                .get_root_seq_mut()
                .visit(&mut extractor)
                .expect("Failed to write translations table");

            println!("Successfully extracted translations \"{}\"", outfile);
        }
        ProgramMode::PackageTranslations => {
            let mut module = E1pModule::load_from_directory(source_dir, &args.name, args.main);

            let outfile = args.file.unwrap_or("./strings.txt".into());
            let file = File::open(&outfile).expect("Failed to create output file");

            let mut replacer = TranslationReplacer::new(file);
            module
                .get_root_seq_mut()
                .visit(&mut replacer)
                .expect("Failed to replace translations");

            module.write_to_directory(Path::new("."), &args.name);

            println!("Successfully replaced translations using \"{}\"", outfile);
        }
        ProgramMode::Disassemble => {
            let mut module = E1pModule::load_from_directory(source_dir, &args.name, args.main);

            let outfile = args.file.unwrap_or("./disassembly.e1d".into());
            let file = File::create(&outfile).expect("Failed to create output file");

            SExpressionWriter::new(file, &module)
                .write(&mut module)
                .expect("Failed to write disassembly to file");

            println!("Successfully wrote disassembly to \"{}\"", outfile);
        }
        ProgramMode::Assemble => {
            todo!()
        }
        ProgramMode::Info => {
            let mut module = E1pModule::load_from_directory(source_dir, &args.name, args.main);

            println!("Replacements:");
            if module.replacements.is_some() {
                for r in module.replacements.as_ref().unwrap().replacements.iter() {
                    println!("  {} => {}", r.old_id, r.new_id);
                }
            } else {
                println!("  No replacements table");
            }

            println!("Rooms:");
            for r in module.rooms.rooms.iter() {
                println!("  {:?}", r);
            }

            println!("Conversations:");
            for c in module.conversations.conversations.iter() {
                match c.ctype {
                    dto::cht::ConversationType::Conversation => {
                        println!(
                            "  Conv id={}, filepos={}, checklist={:?}",
                            c.id, c.filepos, c.checklist
                        );
                    }
                    dto::cht::ConversationType::Room => {
                        println!(
                            "  Room id={}, filepos={}, checklist={:?}",
                            c.id, c.filepos, c.checklist
                        );
                    }
                }
            }

            print!("Phrases:");
            let mut pidx = 0;
            for p in module.phrases.indices.iter() {
                if pidx % 10 == 0 {
                    print!("\n  ");
                }
                pidx += 1;
                print!("{} ", p);
            }
            println!("");

            println!("Gen:");
            let g = &module.generated;
            println!(
                "  name=\"{}\", ver=\"{}\", flags={}, strflags={}, objs={}, objrefs={}, lists={}",
                g.name,
                g.version,
                g.flags_count,
                g.string_flags_count,
                g.objects_count,
                g.object_refs_count,
                g.list_count,
            );

            module.write_to_directory(Path::new("."), &args.name);
        }
    }
}
