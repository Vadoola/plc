//! Detailed test showing LD and SFC reference extraction

use plceye::analysis::analyze_plcopen_project;
use std::fs;

fn main() {
    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example test_graphical_languages <file.xml>");
        std::process::exit(1);
    });

    println!("Analyzing: {}", file_path);
    let xml_content = fs::read_to_string(&file_path).expect("Failed to read file");

    let project: plcopen::Root_project_Inline =
        quick_xml::de::from_str(&xml_content).expect("Failed to parse XML");

    let analysis = analyze_plcopen_project(&project);

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║          PLCopen Graphical Language Analysis              ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    println!("\n📊 Summary:");
    println!("   POUs: {}", analysis.stats.pous);
    println!(
        "   Bodies: ST={} FBD={} LD={} SFC={} IL={}",
        analysis.stats.st_bodies,
        analysis.stats.fbd_bodies,
        analysis.stats.ld_bodies,
        analysis.stats.sfc_bodies,
        analysis.stats.il_bodies
    );

    if analysis.stats.fbd_bodies > 0 {
        println!("\n🔷 FBD (Function Block Diagram):");
        println!("   Function blocks and variables extracted from FBD diagrams");
        println!("   - Blocks (FB/Function calls): Check 'Called POUs'");
        println!("   - Variables: inVariable, outVariable, inOutVariable");
    }

    if analysis.stats.ld_bodies > 0 {
        println!("\n🔶 LD (Ladder Diagram):");
        println!("   Contacts and coils extracted from ladder logic");
        println!("   - Contacts: Input conditions (variables)");
        println!("   - Coils: Output actions (variables)");
        println!("   - Also includes blocks if used in LD");
    }

    if analysis.stats.sfc_bodies > 0 {
        println!("\n🔵 SFC (Sequential Function Chart):");
        println!("   Steps, transitions, and actions extracted from SFC");
        println!("   - Steps: Sequential states (variables)");
        println!("   - Actions: Calls to other POUs");
        println!("   - Transitions: Condition expressions");
    }

    println!("\n📈 Reference Extraction Results:");
    println!(
        "   ✓ Defined variables: {}",
        analysis.defined_variables.len()
    );
    println!("   ✓ Used variables: {}", analysis.used_variables.len());
    println!("   ✓ Called POUs: {}", analysis.used_pous.len());

    if !analysis.used_pous.is_empty() {
        println!("\n🎯 Called POUs (Functions/FBs):");
        let mut pous: Vec<_> = analysis.used_pous.iter().collect();
        pous.sort();
        for pou in pous {
            println!("   • {}", pou);
        }
    }

    if !analysis.used_variables.is_empty() {
        println!("\n🔤 Used Variables:");
        let mut vars: Vec<_> = analysis.used_variables.iter().collect();
        vars.sort();
        for var in vars {
            println!("   • {}", var);
        }
    }

    println!("\n✨ Analysis Capabilities:");
    println!("   ✓ FBD: Blocks, variables, labels/jumps");
    println!("   ✓ LD: Contacts, coils, blocks");
    println!("   ✓ SFC: Steps, actions, transitions, jump steps");
    println!("   ✓ Cross-language: Blocks can appear in all diagrams");
}
