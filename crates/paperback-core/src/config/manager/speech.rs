use super::ConfigManager;
use crate::speech::SpeechRule;

impl ConfigManager {
	/// The speech dictionary, in the order its rules apply.
	#[must_use]
	pub fn get_speech_rules(&self) -> Vec<SpeechRule> {
		if !self.initialized {
			return Vec::new();
		}
		self.data.borrow().speech_rules.clone()
	}

	pub fn set_speech_rules(&self, rules: Vec<SpeechRule>) {
		if !self.initialized {
			return;
		}
		self.data.borrow_mut().speech_rules = rules;
		self.dirty.set(true);
	}
}
