mod bus;
mod cpu;
mod decode;
mod error;
mod execute;
mod isa;
mod memory;
mod recorder;
mod replayer;
mod snapshot;
mod trace;

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;

use clap::{Args, Parser, Subcommand};

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::error::CpuError;
use crate::recorder::SnapshotRecorder;
use crate::replayer::TraceReplayer;

const DEFAULT_RAM_MB: u64 = 128;

#[derive(Debug, Parser)]
#[command(author, version, about = "RISC-V emulator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run(RunArgs),
    Record(RecordArgs),
    Replay(ReplayArgs),
}

#[derive(Debug, Args)]
struct RunArgs {
    #[arg(long, value_name = "PATH")]
    binary: PathBuf,
    #[arg(long, default_value = "0x80000000", value_parser = parse_hex_u64)]
    entry: u64,
    #[arg(long, default_value_t = DEFAULT_RAM_MB, value_name = "MB")]
    ram_size: u64,
    #[arg(long)]
    trace: bool,
}

#[derive(Debug, Args)]
struct RecordArgs {
    #[arg(long, value_name = "PATH")]
    binary: PathBuf,
    #[arg(long, default_value = "0x80000000", value_parser = parse_hex_u64)]
    entry: u64,
    #[arg(long, default_value = "out.trace", value_name = "PATH")]
    output: PathBuf,
    #[arg(long)]
    trace: bool,
}

#[derive(Debug, Args)]
struct ReplayArgs {
    #[arg(long, value_name = "PATH")]
    trace: PathBuf,
    #[arg(long, value_name = "CYCLE")]
    goto: Option<u64>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run(args) => {
            if args.trace {
                init_tracing();
            }

            let mut cpu = match load_cpu(&args.binary, args.entry, args.ram_size) {
                Ok(cpu) => cpu,
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };

            match cpu.run() {
                Ok(()) | Err(CpuError::Ebreak) => {
                    println!("Simulation complete");
                    exit(0);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        }
        Command::Record(args) => {
            if args.trace {
                init_tracing();
            }

            let cpu = match load_cpu(&args.binary, args.entry, DEFAULT_RAM_MB) {
                Ok(cpu) => cpu,
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };

            let mut recorder = SnapshotRecorder::new(cpu);
            match recorder.run() {
                Ok(()) => match recorder.save(&args.output) {
                    Ok(()) => {
                        println!("Trace saved to {}", args.output.display());
                        exit(0);
                    }
                    Err(err) => {
                        eprintln!("Failed to save trace {}: {}", args.output.display(), err);
                        exit(1);
                    }
                },
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        }
        Command::Replay(args) => {
            init_tracing();

            let mut replayer = match TraceReplayer::load(&args.trace) {
                Ok(replayer) => replayer,
                Err(err) => {
                    eprintln!("Failed to load trace {}: {}", args.trace.display(), err);
                    exit(1);
                }
            };

            if let Some(cycle) = args.goto {
                match replayer.goto(cycle) {
                    Ok(_) => {
                        replayer.print_state();
                        exit(0);
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                        exit(1);
                    }
                }
            }

            if let Err(err) = run_repl(&mut replayer) {
                eprintln!("{}", err);
                exit(1);
            }
        }
    }
}

fn parse_hex_u64(input: &str) -> Result<u64, String> {
    let trimmed = input.trim();
    let normalized = trimmed.strip_prefix("0x").unwrap_or(trimmed);
    u64::from_str_radix(normalized, 16)
        .map_err(|err| format!("invalid hex address '{}': {}", input, err))
}

fn load_cpu(binary: &PathBuf, entry: u64, ram_size_mb: u64) -> Result<Cpu, String> {
    let ram_bytes = ram_size_mb
        .checked_mul(1024 * 1024)
        .ok_or_else(|| format!("RAM size overflow: {} MB", ram_size_mb))?;
    let ram_bytes = usize::try_from(ram_bytes)
        .map_err(|_| format!("RAM size too large: {} MB", ram_size_mb))?;

    let data = std::fs::read(binary)
        .map_err(|err| format!("Failed to read binary {}: {}", binary.display(), err))?;

    let mut bus = Bus::new(ram_bytes);
    bus.load_bytes(entry, &data)
        .map_err(|err| format!("Failed to load binary into RAM: {}", err))?;

    Ok(Cpu::new(entry, bus))
}

fn run_repl(replayer: &mut TraceReplayer) -> Result<(), String> {
    let mut input = String::new();

    loop {
        input.clear();
        print!("replay> ");
        io::stdout().flush().map_err(|err| err.to_string())?;

        if io::stdin().read_line(&mut input).map_err(|err| err.to_string())? == 0 {
            return Ok(());
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            replayer.print_state();
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        match parts.next() {
            Some("q") => return Ok(()),
            Some("n") => match replayer.step_forward() {
                Ok(_) => replayer.print_state(),
                Err(err) => eprintln!("{}", err),
            },
            Some("p") => match replayer.step_back() {
                Ok(_) => replayer.print_state(),
                Err(err) => eprintln!("{}", err),
            },
            Some("g") => {
                let cycle = match parts.next() {
                    Some(value) => value.parse::<u64>().map_err(|err| err.to_string())?,
                    None => {
                        eprintln!("missing cycle number");
                        continue;
                    }
                };
                match replayer.goto(cycle) {
                    Ok(_) => replayer.print_state(),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Some(other) => {
                eprintln!("unknown command '{}': use n, p, g <N>, q, or empty line", other);
            }
            None => {}
        }
    }
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt().with_target(false).try_init();
}
