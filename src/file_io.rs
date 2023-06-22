use hound::*;

#[derive(Clone)]
pub struct Buffer {
	prev_path: Option<String>,
	spec: Option<WavSpec>,
	pub channels: Vec<Vec<f32>>,
}

impl Default for Buffer {
	fn default() -> Self {
		Self {
			prev_path: None,
			spec: None,
			channels: Default::default(),
		}
	}
}

impl Buffer {
	pub fn normalize(&mut self) {
		let mut peak: f32 = 0f32;
		for channel in &self.channels {
			for sample in channel {
				peak = peak.max(sample.abs());
			}
		}

		for channel in 0..self.channels.len() {
			for sample in 0..self.channels[channel].len() {
				self.channels[channel][sample] /= peak;
			}
		}
	}
}

pub fn wav_import(path: &str) -> Option<Buffer> {
	let mut result: Buffer = Buffer::default();
	result.prev_path = Some(path.to_string());
	result.spec = Some(WavReader::open(path).unwrap().spec());
	let reader = WavReader::open(path).unwrap();

	let mut raw_samples: Vec<f32> = Vec::<f32>::new();
	match result.spec.unwrap().sample_format {
		SampleFormat::Float => {
			for sample_opt in reader.into_samples::<f32>() {
				let sample = match sample_opt {
					Ok(smp) => smp,
					Err(_) => return None,
				};
				raw_samples.push(sample);
			}
		}
		SampleFormat::Int => {
			for sample_opt in reader.into_samples::<i32>() {
				let sample = match sample_opt {
					Ok(smp) => smp,
					Err(_) => return None,
				};
				raw_samples.push(
					sample as f32
						//scale int samples between -1.0 and 1.0
						/ (i32::MAX >> (32 - result.spec.unwrap().bits_per_sample)) as f32,
				);
			}
		}
	}
	result.spec = Some(WavSpec {
		channels: result.spec.unwrap().channels,
		sample_rate: result.spec.unwrap().sample_rate,
		bits_per_sample: 32,
		sample_format: SampleFormat::Float,
	});

	let channels: usize = result.spec.unwrap().channels as usize;
	for channel in 0..channels {
		let mut new_channel = Vec::<f32>::new();
		for i in 0..(raw_samples.len() / channels) {
			new_channel.push(raw_samples[(i * channels) + channel]);
		}
		result.channels.push(new_channel);
	}

	Some(result)
}

// i'm sorry popbot
// fn flac_import(path: &str) -> Option<Buffer> {
// 	let mut result: Buffer = Buffer::default();
// 	result.prev_path = Some(path.to_string());

// 	let mut reader = match claxon::FlacReader::open(path) {
// 		Ok(unwrapped_reader) => unwrapped_reader,
// 		Err(_) => return None,
// 	};

// 	for sample in reader.samples() {

// 	}
// }

pub fn audio_import(path: &str) -> Option<Buffer> {
	match path.split_once(".") {
		Some((_, extension)) => match extension.to_lowercase().as_str() {
			"wav" => wav_import(path),
			_ => None,
		},
		None => None,
	}
}

pub fn wav_export(suffix: &String, buffer: &Buffer) {
	let buffer_path = buffer.prev_path.as_ref().unwrap();

	let filename = format!(
		"{}{}{}",
		match buffer_path.split_once(".") {
			Some(pathparts) => pathparts.0,
			None => buffer_path.as_str(),
		},
		suffix,
		".wav"
	);

	let mut writer = match hound::WavWriter::create(filename, buffer.spec.unwrap()) {
		Ok(writer) => writer,
		Err(_) => return,
	};
	for sample in 0..buffer.channels[0].len() {
		for channel in 0..buffer.channels.len() {
			writer
				.write_sample(buffer.channels[channel][sample])
				.unwrap();
		}
	}
}
