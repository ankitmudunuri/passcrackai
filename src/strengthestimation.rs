use std::collections::HashMap;
use tract_onnx::prelude::*;
use ndarray::Array2;
use ndarray::Axis;

const MAX_LENGTH: usize = 16;

static MODEL_BYTES: &[u8] = include_bytes!("../files/password_strength_model_cnn.onnx");

pub fn estimate(input: String) -> String {
    let model = tract_onnx::onnx()
        .model_for_read(&mut std::io::Cursor::new(&MODEL_BYTES[..]))
        .expect("Failed to load the ONNX model")
        .into_optimized()
        .expect("Failed to optimize the model")
        .into_runnable()
        .expect("Failed to make the model runnable");

    let (char_to_idx, _vocab_size) = build_char_to_idx();

    let encoded = encode_password(&input, &char_to_idx);

    let input_array = Array2::<i32>::from_shape_vec((1, MAX_LENGTH), encoded.to_vec())
        .expect("Failed to create input array");
    let input_tensor = Tensor::from(input_array);

    let outputs = model
        .run(tvec![input_tensor.into()])
        .expect("Failed to run the model");

    let output_tensor = &outputs[0];

    let probs = output_tensor
        .to_array_view::<f32>()
        .expect("Failed to convert output tensor to array view");

    let probs_for_one = probs.index_axis(Axis(0), 0);
    let (best_index, _) = probs_for_one
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .expect("Failed to find the best index");

    best_index.to_string()
}


fn build_char_to_idx() -> (HashMap<char, i32>, i32) {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    
    let mut char_to_idx = HashMap::new();
    let mut idx = 1;
    for c in chars.chars() {
        char_to_idx.insert(c, idx);
        idx += 1;
    }
    
    let vocab_size = idx;
    (char_to_idx, vocab_size)
}

fn encode_password(pwd: &str, char_to_idx: &HashMap<char, i32>) -> [i32; MAX_LENGTH] {
    let mut encoded = [0; MAX_LENGTH];
    for (i, ch) in pwd.chars().take(MAX_LENGTH).enumerate() {
        encoded[i] = *char_to_idx.get(&ch).unwrap_or(&0);
    }
    encoded
}
