use std::{env, fs::read_to_string, sync::atomic};
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::macros::command,
    framework::standard::{Args, CommandResult},
    json::json,
    model::channel::Message,
};

#[derive(Debug, Deserialize)]
struct Element {
    name: String,
    appearance: Option<String>,
    atomic_mass: f64,
    boil: Option<f64>,
    category: String,
    density: Option<f64>,
    discovered_by: Option<String>,
    melt: Option<f64>,
    molar_heat: Option<f64>,
    named_by: Option<String>,
    number: u32,
    period: u32,
    group: u32,
    phase: String,
    source: String,
    bohr_model_image: Option<String>,
    bohr_model_3d: Option<String>,
    spectral_img: Option<String>,
    summary: String,
    symbol: String,
    xpos: u32,
    ypos: u32,
    wxpos: u32,
    wypos: u32,
    shells: Vec<u32>,
    electron_configuration: String,
    electron_configuration_semantic: String,
    electron_affinity: Option<f64>,
    electronegativity_pauling: Option<f64>,
    ionization_energies: Vec<f64>,
    cpk_hex: Option<String>,
    image: Image,
    block: String,
}

#[derive(Debug, Deserialize)]
struct Image {
    title: String,
    url: String,
    attribution: String,
}

#[derive(Debug, Deserialize)]
struct Elements {
    elements: Vec<Element>,
}

#[command]
pub async fn element(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("src/commands/chemistry/periodic_table.json");

    let json_contents = read_to_string(file_path).expect("Failed to read JSON");
    let Elements { elements } = serde_json::from_str(&json_contents).expect("Failed to parse JSON");

    let atomic_number: u32 = args.single()?;
    let Element {
        name,
        electron_configuration,
        discovered_by,
        symbol,
        number,
        named_by,
        ..
    } = &elements[(atomic_number - 1) as usize];

    let discovered_by_str = discovered_by.to_owned().unwrap_or("null".to_string());
    let named_by_str = named_by.to_owned().unwrap_or("null".to_string());

    let content = format!(
        "**Name:** {}\n**Symbol:** {}\n**Atomic number:** {}\n**Discovered by:** {}\n**Named by**: {}\n**Electron Configuration:** {}",
        name, symbol, number, discovered_by_str, named_by_str, electron_configuration
    );

    msg.reply(ctx, content).await?;

    Ok(())
}

// Deprecated
#[command]
pub async fn econfig(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let sub = [
        "s", "s", "p", "s", "p", "s", "d", "p", "s", "d", "p", "s", "f", "d", "p", "s", "f", "d",
        "p", "f", "d", "f",
    ];
    let principal_quantum_number = [
        1, 2, 2, 3, 3, 4, 3, 4, 5, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 6, 7, 7,
    ];
    let diff: i32;
    let mut electrons = [0; 22];
    let mut current_electron = 0;
    let mut l: usize = 0;
    let mut answer = String::new();

    let mut args = args;
    let atomic_number: i32 = args.single()?;

    while atomic_number != current_electron {
        match sub[l] {
            "s" => {
                current_electron += 2;
                electrons[l] += 2;
            }
            "p" => {
                current_electron += 6;
                electrons[l] += 6;
            }
            "d" => {
                current_electron += 10;
                electrons[l] += 10;
            }
            _ => {
                current_electron += 14;
                electrons[l] += 14;
            }
        }

        if current_electron > atomic_number {
            diff = current_electron - atomic_number;
            electrons[l] -= diff;
            break;
        }

        l += 1;
    }

    for i in 0..l + 1 {
        let electrons_string = electrons[i].to_string();
        let principal_quantum_string = principal_quantum_number[i].to_string();

        answer.push_str(&format!(
            "{}{}{} ",
            principal_quantum_string, sub[i], electrons_string
        ));
    }

    msg.reply(ctx, answer).await?;

    Ok(())
}
