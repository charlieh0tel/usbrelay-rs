use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use usbrelay_rs::sainsmart::SainSmartFourChannelRelay;

/// Simple CLI to control SainSmart Four Channel USB Relay
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// FTDI device string.
    #[arg(short, long, default_value = "i:0x0403:0x6001")]
    device: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Turns on channels.
    On(Channels),

    /// Turns off channels.
    Off(Channels),
}

#[derive(Args)]
struct Channels {
    channels: Vec<u8>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let relay = SainSmartFourChannelRelay::new(&cli.device)?;
    let pins = relay.read()?;

    let mut next_pins = pins;
    match &cli.command {
        Commands::On(channels) => {
            for channel in channels.channels.iter() {
                // TODO(ch): check channel is  valid
                next_pins |= 1 << channel;
            }
        }
        Commands::Off(channels) => {
            for channel in channels.channels.iter() {
                // TODO(ch): check channel is valid
                next_pins &= !(1 << channel);
            }
        }
    }

    relay.set(next_pins)?;

    let pins = relay.read()?;

    if pins != next_pins {
        bail!("Failed to set pins correctly, expected 0x{next_pins:02X}, got 0x{pins:02X}");
    }

    Ok(())
}
