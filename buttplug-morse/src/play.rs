use std::time::Duration;

use async_trait::async_trait;
use buttplug::client::{ButtplugClientDevice, ButtplugClientError, VibrateCommand};

use crate::{
    morse::{Letter, Sentence, Signal, Word},
    options::MorseOptions,
};

#[async_trait]
pub trait Play {
    /// Plays the morse representation of `self` on the provided device, using the supplied options.
    async fn play(
        &self,
        device: &ButtplugClientDevice,
        options: &MorseOptions,
    ) -> Result<(), ButtplugClientError>;
}

#[async_trait]
impl Play for Signal {
    async fn play(
        &self,
        device: &ButtplugClientDevice,
        options: &MorseOptions,
    ) -> Result<(), ButtplugClientError> {
        // Play the signal
        device
            .vibrate(VibrateCommand::Speed(*options.intensity))
            .await?;

        tokio::time::sleep(Duration::from_millis(self.duration(options))).await;
        device.stop().await?;

        // Sleep for the time between signals
        tokio::time::sleep(Duration::from_millis(options.pause_length)).await;

        Ok(())
    }
}

#[async_trait]
impl Play for Letter {
    async fn play(
        &self,
        device: &ButtplugClientDevice,
        options: &MorseOptions,
    ) -> Result<(), ButtplugClientError> {
        for signal in self.signals() {
            signal.play(device, options).await?;
        }

        // Then sleep for the time between letters
        tokio::time::sleep(Duration::from_millis(options.letter_pause_length)).await;

        Ok(())
    }
}

#[async_trait]
impl Play for Word {
    async fn play(
        &self,
        device: &ButtplugClientDevice,
        options: &MorseOptions,
    ) -> Result<(), ButtplugClientError> {
        for letter in self.letters() {
            letter.play(device, options).await?;
        }

        // Then sleep for the space time between words
        tokio::time::sleep(Duration::from_millis(options.space_length)).await;

        Ok(())
    }
}

#[async_trait]
impl Play for Sentence {
    async fn play(
        &self,
        device: &ButtplugClientDevice,
        options: &MorseOptions,
    ) -> Result<(), ButtplugClientError> {
        for word in self.words() {
            word.play(device, options).await?;
        }

        // Then we're done.
        Ok(())
    }
}

/// Plays a sentence on the provided device with the specified options. This is the bread-and-butter of this crate and
/// should typically be what you reach for first.
pub async fn play(
    device: &ButtplugClientDevice,
    options: &MorseOptions,
    sentence: &str,
) -> Result<(), ButtplugClientError> {
    Sentence::new_lossy(sentence).play(device, options).await
}
