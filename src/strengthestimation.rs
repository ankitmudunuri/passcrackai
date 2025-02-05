
use ndarray::prelude::*;
use ndarray::Axis;
use std::collections::HashMap;

const MAX_LENGTH: usize = 16;
//static MODEL_BYTES: &[u8] = include_bytes!("../files/passstrength_postprocessed.onnx");

/* 
pub fn estimate(input: String) -> String {
    // Load the ONNX model
    let model = tract_onnx::onnx()
        .model_for_read(&mut std::io::Cursor::new(&MODEL_BYTES[..]))
        .expect("Failed to load the ONNX model")
        .into_optimized()
        .expect("Failed to optimize the model")
        .into_runnable()
        .expect("Failed to make the model runnable");

    // Build the character-to-index mapping
    let (char_to_idx, _vocab_size) = build_char_to_idx();

    // Encode the password
    let encoded = encode_password(&input, &char_to_idx);

    // Convert the encoded password into the required input tensor format
    let input_array = Array2::<i32>::from_shape_vec((1, MAX_LENGTH), encoded.to_vec())
        .expect("Failed to create input array");
    let input_tensor = Tensor::from(input_array);

    // Run the model
    let outputs = model.run(tvec![input_tensor.into()]).expect("Failed to run the model");
    let output_tensor = &outputs[0];

    // Get the probability array
    let probs = output_tensor
        .to_array_view::<f32>()
        .expect("Failed to convert output tensor to array view");

    // Select the highest probability class
    let probs_for_one = probs.index_axis(Axis(0), 0);
    let (best_index, _) = probs_for_one
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .expect("Failed to find the best index");

    // Return the best index as a string
    best_index.to_string()
}

*/

pub fn estimate(input: String) -> String {
    return "0".to_string();
}

fn build_char_to_idx() -> (HashMap<char, i32>, i32) {
    // Replicate string.ascii_letters + string.digits + string.punctuation
    // Note: string.ascii_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    // Then digits "0123456789"
    // Then punctuation. Python’s string.punctuation = !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
    // Make sure the order is identical to your Python script’s `list(...)` ordering.
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    
    // Build the map {char -> index}, offset by 1 to keep 0 for “padding”
    let mut char_to_idx = HashMap::new();
    let mut idx = 1;
    for c in chars.chars() {
        char_to_idx.insert(c, idx);
        idx += 1;
    }
    
    // The total vocabulary size is length + 1 for the padding index
    let vocab_size = idx; // i.e. final value of idx
    (char_to_idx, vocab_size)
}

fn encode_password(pwd: &str, char_to_idx: &HashMap<char, i32>) -> [i32; MAX_LENGTH] {
    let mut encoded = [0; MAX_LENGTH];
    // Truncate or pad
    for (i, ch) in pwd.chars().take(MAX_LENGTH).enumerate() {
        // If character not in vocabulary, use 0; else use its index
        encoded[i] = *char_to_idx.get(&ch).unwrap_or(&0);
    }
    encoded
}

