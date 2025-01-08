use anyhow::Result;
use onnxruntime::{
    // We import Environment, Session, and the OrtOwnedTensor type
    environment::Environment,
    session::Session,
    OrtOwnedTensor,
};
use ndarray::{Array, IxDyn}; // from the separate `ndarray` crate
use std::collections::HashSet;

// ----------------------------------------------------------------
// 1. Encode the password as integers (same logic as in training)
// ----------------------------------------------------------------
fn encode_password(password: &str, max_length: usize) -> Vec<i64> {
    // Example character set used during training
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>?/"
        .chars()
        .collect::<Vec<_>>();

    // Build a lookup map: char -> integer index
    let char_to_idx = chars
        .into_iter()
        .enumerate()
        .map(|(i, c)| (c, i as i64 + 1))  // +1 so 0 can be "unknown"/padding
        .collect::<std::collections::HashMap<_, _>>();

    // Convert each character of the password
    let mut encoded: Vec<i64> = password
        .chars()
        .map(|c| *char_to_idx.get(&c).unwrap_or(&0))
        .collect();

    // Truncate or pad
    if encoded.len() > max_length {
        encoded.truncate(max_length);
    } else {
        encoded.resize(max_length, 0);
    }
    encoded
}

// ----------------------------------------------------------------
// 2. Extract complexity features (again, must match training code)
// ----------------------------------------------------------------
fn extract_features(password: &str) -> Vec<f32> {
    let length = password.len() as f32;
    let num_upper = password.chars().filter(|c| c.is_uppercase()).count() as f32;
    let num_lower = password.chars().filter(|c| c.is_lowercase()).count() as f32;
    let num_digits = password.chars().filter(|c| c.is_ascii_digit()).count() as f32;
    let num_special = password.chars().filter(|c| !c.is_alphanumeric()).count() as f32;
    let unique_chars = password.chars().collect::<HashSet<_>>().len() as f32;

    // Avoid division by zero
    if length > 0.0 {
        vec![
            length,
            num_upper / length,
            num_lower / length,
            num_digits / length,
            num_special / length,
            unique_chars,
        ]
    } else {
        // If empty password, just return zeros
        vec![0.0; 6]
    }
}

// ----------------------------------------------------------------
// 3. Public function to run inference
//    Must be pub so main.rs can call it
// ----------------------------------------------------------------
pub fn prediction(password: &str) -> Result<i64> {
    // Adjust constants to your training setup
    let max_length = 16;
    let num_features = 6;

    // 1. Create ONNX environment
    let environment = Environment::builder()
        .with_name("rust-onnx")
        .build()?;

    // 2. Create session (SessionBuilder is private in 0.0.14)
    //    Make sure your model path is correct
    let session = Session::new(&environment, "files/password_strength_model.onnx")?;

    // 3. Preprocess the password
    let sequence_encoded = encode_password(password, max_length);
    let features = extract_features(password);

    // 4. Create input arrays (batch_size=1)
    let sequence_input = Array::from_shape_vec(IxDyn(&[1, max_length]), sequence_encoded)?;
    let complexity_input = Array::from_shape_vec(IxDyn(&[1, num_features]), features)?;

    // 5. Run inference
    //    The input names must match your ONNX model's input node names.
    //    The returned value is a Vec of OrtOwnedTensor (one per output).
    let outputs: Vec<OrtOwnedTensor<f32, IxDyn>> = session.run(vec![
        ("password_sequence", &sequence_input),
        ("complexity_features", &complexity_input),
    ])?;

    // 6. Process the output
    //    Typically, the model outputs a shape [batch_size, num_classes] with probabilities.
    //    We'll find the index with the highest probability.
    //    For a single batch, outputs[0] is the output for that batch.
    let predictions = outputs[0].view(); // an ndarray::ArrayViewD<f32>
    // predictions shape might be [1, 5] if you have 5 strength classes (0..4).
    // We'll take row 0:
    let row = predictions.index_axis(ndarray::Axis(0), 0);

    // Find the index of the maximum value
    let (max_idx, _max_val) = row
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    // Return the predicted class as i64
    Ok(max_idx as i64)
}
