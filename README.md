# ESP32 Combination Generator Firmware

An optimized **ESP32 Arduino firmware** that generates all possible combinations of a given character set for a specified code length. Each combination is **calculated and printed dynamically** to the Serial Monitor without storing all combinations in memory, making it ideal for ESP32's limited RAM environment.

---

## Features

* Accepts **code length** from the Serial Monitor (1–10 characters).
* Calculates and displays **total possible combinations** instantly.
* Generates combinations **on-the-fly**, one by one, without using RAM for storage.
* Streams combinations directly to the **Serial Monitor**.
* Safe and optimized for ESP32 microcontrollers.
* Optional: **limit the number of generated codes** for testing extremely large lengths.

---

## Supported Character Set

The default character set includes **94 printable characters**:

```
ABCDEFGHIJKLMNOPQRSTUVWXYZ
abcdefghijklmnopqrstuvwxyz
0123456789
!@#$%^&*()-_=+[]{};:'",.<>/?\|
```

You can customize this character set in the firmware if desired.

---

## Hardware Requirements

* ESP32 Development Board
* USB cable for Serial connection
* Arduino IDE (or PlatformIO)

---

## Installation & Setup

1. **Install Arduino IDE** (or use PlatformIO).
2. **Select ESP32 board**: Tools → Board → ESP32 Dev Module.
3. **Connect ESP32** via USB.
4. **Open the firmware sketch** in Arduino IDE.
5. **Upload the code** to the ESP32.

---

## Usage

1. Open **Serial Monitor** (set baud rate to 115200).
2. You will see the welcome message:

```
=== ESP32 Combination Generator ===
Enter code length (1–10):
```

3. Enter a number for the code length and press **Enter**.
4. The ESP32 prints:

```
Length received: 3
Total combinations: 830584
Generating (streaming one by one)...
AAA
AAB
AAC
...
```

5. After completion (or reaching the optional limit), it prints:

```
✅ Done generating all combinations.
Enter next code length:
```

6. You can now enter a new length to generate another set of combinations.

---

## Important Notes

* **Extremely large code lengths (≥8)** produce trillions of combinations; generating all is **practically impossible**.
* For testing large lengths, use the optional **limit** feature to print only the first few thousand combinations.
* The firmware **does not store combinations**; it calculates each one dynamically and discards it immediately to save RAM.
* Use a **small delay (1–2ms)** if Serial printing causes buffer overflow.

---

## Customization

### Change Character Set

Modify the `charset` array in the firmware:

```cpp
const char charset[] = "ABC123"; // Example custom charset
```

### Limit Generated Combinations

For testing purposes, you can limit the number of codes generated:

```cpp
if (index >= 1000) break; // Stop after first 1000 combinations
```

### Adjust Serial Delay

Prevent Serial buffer overflow during fast printing:

```cpp
delay(1); // optional
```

---

## Performance Considerations

| Code Length | Total Combinations       | Practical to Print            |
| ----------- | ------------------------ | ----------------------------- |
| 1–5         | 94 – 7,338,136           | ✅ Fast                        |
| 6–7         | 689 million – 64 billion | ⚠️ Slow                       |
| 8–10        | 6 trillion+              | 🚫 Impractical to fully print |

Use the **limit** feature to generate a subset for testing long lengths.

---

## License

MIT License – free to use, modify, and distribute.

---

## Summary

This firmware provides a **reliable, memory-efficient solution** for generating all possible combinations of a character set on an ESP32. It's ideal for **learning, testing, and research** purposes where streaming output is required without overwhelming the device's memory.
