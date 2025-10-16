#include <Arduino.h>

// Full printable character set (94 characters)
const char charset[] =
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
  "abcdefghijklmnopqrstuvwxyz"
  "0123456789"
  "!@#$%^&*()-_=+[]{};:'\",.<>/?\\|";
const int charsetLen = sizeof(charset) - 1;

unsigned long long powerCalc(int base, int exp) {
  unsigned long long result = 1;
  for (int i = 0; i < exp; i++) result *= base;
  return result;
}

void generateCombinations(int length) {
  char buffer[20]; // enough for up to 19 chars + null terminator
  unsigned long long total = powerCalc(charsetLen, length);

  Serial.print("\nTotal combinations: ");
  Serial.println(total);
  Serial.println("Generating (streaming one by one)...\n");

  // Generate combinations dynamically
  for (unsigned long long index = 0; index < total; index++) {
    unsigned long long temp = index;

    // Build the combination in reverse order
    for (int pos = length - 1; pos >= 0; pos--) {
      buffer[pos] = charset[temp % charsetLen];
      temp /= charsetLen;
    }
    buffer[length] = '\0';

    // Print and forget immediately
    Serial.println(buffer);
    delay(1); // slight delay for serial stability (optional)
  }

  Serial.println("\n✅ Done generating all combinations.");
}

void setup() {
  Serial.begin(115200);
  delay(2000);

  Serial.println("=== ESP32 Combination Generator ===");
  Serial.println("Enter code length (1–10):");
}

void loop() {
  static String input = "";
  if (Serial.available()) {
    char c = Serial.read();

    if (c == '\n' || c == '\r') {
      if (input.length() > 0) {
        int len = input.toInt();
        Serial.print("Length received: ");
        Serial.println(len);

        if (len >= 1 && len <= 10) {
          generateCombinations(len);
        } else {
          Serial.println("⚠️ Please enter a length between 1 and 10.");
        }

        Serial.println("\nEnter next code length:");
        input = "";
      }
    } else {
      input += c;
    }
  }
}
