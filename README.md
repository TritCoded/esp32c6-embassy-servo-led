# ESP32-C6 Embassy Servo + LED Concurrency Test
This project was a test of concurrent programming on an ESP32-C6 using **Embassy async in Rust**. 

## Overview

The firmware runs two independent tasks in parallel:

- A **GPIO LED blink task**
- A **servo control task** that continuously sweeps the servo back and forth

## Hardware

- ESP32-C6 DevKit
- Breadboard
- LED
- 300 Ohm resistor
- Servo motor

## GPIO
Servo signal -> GPIO4

LED -> 300 Ohm resistor -> GPIO2

## Showcase
