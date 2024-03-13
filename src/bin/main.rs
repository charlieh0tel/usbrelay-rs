use anyhow::Result;
use std::{thread, time};
use usbrelay_rs::sainsmart::SainSmartFourChannelRelay;

fn main() -> Result<()> {
    let relay = SainSmartFourChannelRelay::new("i:0x0403:0x6001")?;
    let pins = relay.read()?;
    println!("Original pins: 0x{pins:02X}");

    for new_pins in 0..=15 {
        println!("{new_pins:04b}");
        relay.set(new_pins)?;

        thread::sleep(time::Duration::from_secs(1));
    }
    relay.set(0)?;

    let pins = relay.read()?;
    println!("Now pins: 0x{pins:02X}");
    Ok(())
}
