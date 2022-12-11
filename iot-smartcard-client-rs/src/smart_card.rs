use pcsc::{Card, Context, MAX_BUFFER_SIZE, Protocols, Scope, ShareMode};
use thiserror::Error;

#[allow(dead_code)]
pub struct SmartCard {
    pcsc_card: Card,
}

#[allow(dead_code)]
impl SmartCard {
    /// Create a new smartcard object, connects to the first smartcard reader
    pub fn new() -> Result<SmartCard, anyhow::Error> {
        // We assume here there is only 1 smartcard reader connected to the computer.
        // Establish a PC/SC context.
        let ctx = Context::establish(Scope::User)
            .map_err(|err| anyhow::anyhow!("Failed to establish context: {}", err))?;

        // List available readers.
        let mut readers_buf = [0; 2048];
        let mut readers = ctx.list_readers(&mut readers_buf)
            .map_err(|err| anyhow::anyhow!("Failed to list readers: {}", err))?;

        // Use the first reader.
        let reader = readers.next()
            .ok_or_else(|| anyhow::anyhow!("No readers are connected."))?;

        let card = ctx.connect(reader, ShareMode::Shared, Protocols::ANY)
            .map_err(|err| anyhow::anyhow!("Failed to connect to card: {}", err))?;

        Ok(Self {
            pcsc_card: card
        })
    }

    /// Sends an APDU command to the smartcard and returns the response.
    pub fn send_apdu_command(&self, apdu_command: apdu::Command) -> Result<apdu::Response, anyhow::Error> {
        let mut rapdu_buf = [0; MAX_BUFFER_SIZE];

        let rapdu = self.pcsc_card.transmit(<apdu::Command as Into<Vec<u8>>>::into(apdu_command).as_slice(), &mut rapdu_buf)
            .map_err(|err| anyhow::anyhow!("Failed to transmit APDU command to card: {}", err))?;

        Ok(rapdu.to_vec().into())
    }
}

/// Represents an error that can occur when communicating with a smartcard.
#[derive(Error, Debug)]
pub enum SmartCardError {
    #[error("Failed to establish context: {0}")]
    FailedToEstablishContext(String),
    #[error("Failed to list readers: {0}")]
    FailedToListReaders(String),
    #[error("No readers are connected.")]
    NoReadersConnected,
    #[error("Failed to connect to card: {0}")]
    FailedToConnectToCard(String),
    #[error("Failed to transmit APDU command to card: {0}")]
    FailedToTransmitApduCommandToCard(String),
}