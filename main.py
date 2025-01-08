import pandas as pd
import numpy as np
import tensorflow as tf
from tensorflow import keras
from keras import layers
from sklearn.model_selection import train_test_split
import string
import re
from math import log2
from zxcvbn import zxcvbn

csv_path = "data/processed/processedinfo.csv"

df = pd.read_csv(csv_path, delimiter="\t", header=1, names=["password", "score", "crack_time"])

def time_to_seconds(time_str):
    """Convert time strings into seconds."""
    if "less than a second" in time_str:
        return 1
    elif "second" in time_str or "seconds" in time_str:
        return int(time_str.split()[0])
    elif "minute" in time_str or "minutes" in time_str:
        return int(time_str.split()[0]) * 60
    elif "hour" in time_str or "hours" in time_str:
        return int(time_str.split()[0]) * 3600
    elif "day" in time_str or "days" in time_str:
        return int(time_str.split()[0]) * 86400
    elif "month" in time_str or "months" in time_str:
        return int(time_str.split()[0]) * 2592000
    elif "year" in time_str or "years" in time_str:
        return int(time_str.split()[0]) * 31536000
    elif "centuries" in time_str:
       
        return 31536000 * 999
    else:
        return 0

df["crack_time_seconds"] = df["crack_time"].apply(time_to_seconds)
df["score"] = df["score"].astype(int)


df.drop(columns=["crack_time"], inplace=True)


chars = list(string.ascii_letters + string.digits + string.punctuation)
char_to_idx = {c: i + 1 for i, c in enumerate(chars)} 
MAX_LENGTH = 16  

def encode_password(pwd):
    """Convert each password into a sequence of character indices."""
    encoded = [char_to_idx.get(c, 0) for c in pwd[:MAX_LENGTH]]
    while len(encoded) < MAX_LENGTH:
        encoded.append(0)
    return encoded

def extract_features(password):
 
    length = len(password)

 
    num_upper = sum(1 for c in password if c.isupper())
    num_lower = sum(1 for c in password if c.islower())
    num_digits = sum(1 for c in password if c.isdigit())
    num_special = sum(1 for c in password if not c.isalnum())


    upper_ratio = num_upper / length if length > 0 else 0
    lower_ratio = num_lower / length if length > 0 else 0
    digit_ratio = num_digits / length if length > 0 else 0
    special_ratio = num_special / length if length > 0 else 0


    unique_chars = len(set(password))


    if length > 0:
        char_counts = [password.count(c) for c in set(password)]
        entropy = -sum((count/length)*log2(count/length) for count in char_counts)
    else:
        entropy = 0


    dictionary_words = ["password", "admin", "qwerty", "letmein", "123456"]
    contains_dict_word = any(dw in password.lower() for dw in dictionary_words)
    dict_word_flag = 1 if contains_dict_word else 0

    triple_repeat = 1 if re.search(r"(.)\1\1", password) else 0

    return [
        length,
        upper_ratio,
        lower_ratio,
        digit_ratio,
        special_ratio,
        unique_chars,
        entropy,
        dict_word_flag,
        triple_repeat
    ]

df = df.dropna(subset=["password"])

df["password"] = df["password"].astype(str)

df["password_encoded"] = df["password"].apply(encode_password)

df["complexity_features"] = df["password"].apply(extract_features)

X_seq = np.array(df["password_encoded"].tolist())
X_feats = np.array(df["complexity_features"].tolist())
y = df["score"].values.astype(int)

X_train_seq, X_temp_seq, y_train, y_temp = train_test_split(X_seq, y, test_size=0.3, random_state=42)
X_train_feats, X_temp_feats = X_feats[:len(X_train_seq)], X_feats[len(X_train_seq):]

X_val_seq, X_test_seq, y_val, y_test = train_test_split(X_temp_seq, y_temp, test_size=0.5, random_state=42)
X_val_feats, X_test_feats = X_temp_feats[:len(X_val_seq)], X_temp_feats[len(X_val_seq):]

print(f"Training set size: {len(X_train_seq)}")
print(f"Validation set size: {len(X_val_seq)}")
print(f"Test set size: {len(X_test_seq)}")

vocab_size = len(chars) + 1
num_classes = len(np.unique(y))

sequence_input = keras.Input(shape=(MAX_LENGTH,), name="password_sequence")
features_input = keras.Input(shape=(X_feats.shape[1],), name="complexity_features")

x = layers.Embedding(input_dim=vocab_size, output_dim=16, input_length=MAX_LENGTH)(sequence_input)
x = layers.Bidirectional(layers.LSTM(32))(x)
#x = layers.Dropout(0.2)(x)

f = layers.Dense(16, activation='relu')(features_input)
#f = layers.Dropout(0.2)(f) 

combined = layers.Concatenate()([x, f])
combined = layers.Dense(32, activation='relu')(combined)
#combined = layers.Dropout(0.2)(combined) 
output = layers.Dense(num_classes, activation='softmax')(combined)

model = keras.Model(inputs=[sequence_input, features_input], outputs=output)

model.compile(
    loss='sparse_categorical_crossentropy',
    optimizer='adam',
    metrics=['accuracy']
)

model.summary()


history = model.fit(
    [X_train_seq, X_train_feats], y_train,
    validation_data=([X_val_seq, X_val_feats], y_val),
    epochs=25,
    batch_size=256,
    verbose=2
)

test_loss, test_acc = model.evaluate([X_test_seq, X_test_feats], y_test, verbose=0)
print(f"Test Accuracy: {test_acc:.4f}")


model.save("password_strength_model_noDO2~.h5")
print("Model saved as password_strength_model_noDO2.h5")

new_passwords = ["MyNewPass!", "abc", "ThisIsVeryStrong123!!!", "Test1234", "asdfhasdlejKLFDJASDFL;KJAlkajsdflasndfghojdjfs3235efsdf", "12345678", "av"]

testingaccuracy = []
for x in new_passwords:
    results = zxcvbn(x)
    testingaccuracy.append((x, results["score"]))

X_new_seq = np.array([encode_password(p) for p in new_passwords])
X_new_feats = np.array([extract_features(p) for p in new_passwords])

predictions = model.predict([X_new_seq, X_new_feats])
predicted_scores = np.argmax(predictions, axis=1)

for p, s in zip(new_passwords, predicted_scores):
    print(f"Password: {p}, Predicted Strength Score: {s}")

print("Actual accuracies: ")
print(testingaccuracy)
