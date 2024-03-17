use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use clap_num::maybe_hex;
use usbrelay_rs::sainsmart::SainSmartFourChannelRelay;

/// Simple CLI to control SainSmart Four Channel USB Relay
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// libftdi device string.  (Can be:
    ///   d:<device node>
    ///   i:<vendor>:<product>
    ///   i:<vendor>:<product>:<index>
    ///   s:<vendor>:<product>:<serial>
    /// )
    #[arg(short, long, default_value = "i:0x0403:0x6001", verbatim_doc_comment)]
    device: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Turns on channels.
    On { channels: Vec<u8> },

    /// Turns off channels.
    Off { channels: Vec<u8> },

    /// Get channels as hex value.
    Get,

    /// Set channels with hex value.
    Set {
        #[arg(value_parser=maybe_hex::<u8>)]
        next_pins: u8,
    },
}

fn set_and_check_pins(relay: &mut SainSmartFourChannelRelay, next_pins: u8) -> Result<()> {
    relay.set(next_pins)?;
    let pins = relay.read()?;
    if pins != next_pins {
        bail!("Failed to set pins, expected 0x{next_pins:02X}, got 0x{pins:02X}");
    }
    Ok(())
}

struct ClearPins(u8);
struct SetPins(u8);

fn frob_pins(relay: &mut SainSmartFourChannelRelay, clear: ClearPins, set: SetPins) -> Result<()> {
    let pins = relay.read()?;
    let mut next_pins = pins;
    next_pins &= !clear.0;
    next_pins |= set.0;
    next_pins &= SainSmartFourChannelRelay::MASK;
    set_and_check_pins(relay, next_pins)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut relay = SainSmartFourChannelRelay::new(&cli.device)?;

    match &cli.command {
        Commands::On { channels } => {
            let mut set_pins = SetPins(0);
            let clear_pins = ClearPins(0);
            for channel in channels.iter() {
                // TODO(ch): check if channel is  valid.
                set_pins.0 |= 1 << channel;
            }
            frob_pins(&mut relay, clear_pins, set_pins)?;
        }
        Commands::Off { channels } => {
            let set_pins = SetPins(0);
            let mut clear_pins = ClearPins(0);
            for channel in channels.iter() {
                // TODO(ch): check if channel is valid.
                clear_pins.0 &= !(1 << channel);
            }
            frob_pins(&mut relay, clear_pins, set_pins)?;
        }
        Commands::Get => {
            let pins = relay.read()?;
            println!("0x{pins:02X}");
        }
        Commands::Set { mut next_pins } => {
            next_pins &= SainSmartFourChannelRelay::MASK;
            set_and_check_pins(&mut relay, next_pins)?;
        }
    }

    Ok(())
}
