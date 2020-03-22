#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct SimpleDist {
    params: Arc<SimpleDistParams>,
}

struct SimpleDistParams {
    threshold: AtomicFloat
}

impl Default for SimpleDist {
    fn default() -> SimpleDist {
        SimpleDist {
            params: Arc::new(SimpleDistParams::default()),
        }
    }
}

impl Default for SimpleDistParams {
    fn default() -> SimpleDistParams {
	SimpleDistParams {
	    threshold: AtomicFloat::new(0.5),
	}
    }
}

impl Plugin for SimpleDist {
    fn get_info(&self) -> Info {
        Info {
            name: "SimpleDist".to_string(),
            vendor: "Topi Kettunen".to_string(),
            unique_id: 22032020,
	    version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 1,
	    category: Category::Effect,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
	let threshold = self.params.threshold.get();
	
        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(threshold) / threshold;
                } else {
                    *output_sample = input_sample.max(-threshold) / threshold;
                }
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
	Arc::clone(&self.params) as Arc <dyn PluginParameters>
    }
}

impl PluginParameters for SimpleDistParams {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.threshold.set(value),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold",
            _ => "",
        }.to_string()
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.threshold.get() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            _ => "".to_string(),
        }
    }
}

plugin_main!(SimpleDist);
