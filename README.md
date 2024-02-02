# Project Title: Collecting Temperature and Humidity data from SHT31 sensor using micro:bit V2 microcontroller.

## Description:

   This project demonstrates the collection of temperature and humidity data from a SHT31 sensor using the micro:bit V2 microcontroller and the Kitronik Breakout Board Edge Connector.

## Project's Hardware:

- 1 Micro:bit V2 microcontroler
- 1 Kitronik Breakout Board Edge Connector

   - Note: If you don't have pin headers on the 19 or 20 through-hole,  you need to solder pin headers to those through-holes.

- 1 SHT31 Groove Temperature and Humidity Sensor

- 1 830 hole breadboard
- 2 4.7k Ohm Resistors
- 1 micro-USB to USB-A Cable
- 1 M/M 6" Premium Jumper Cables (Green) 
    - Note: If you can find a White Premium Jumper Cable, it will color match with the sensor's cable. If that's the case, substitue the Green for the White for this project. 
- 1 M/F 6" Premium Jumper Cables (Green) 
    - Note: If you can find a White Premium Jumper Cable, it will color match with the sensor's cable. If that's the case, substitue the Green for the White for this project. 
- 1 M/M 6" Premium Jumper Cables (Yellow) 
- 1 M/F 6" Premium Jumper Cables (Yellow) 
- 1 M/M 6" Premium Jumper Cable (Red)
- 1 M/F 6" Premium Jumper Cable (Red) 
- 1 M/M 6" Premium Jumper Cable (Black)
- 1 M/F 6" Premmium Jumper Cable (Black)

## Instructions:

1. Clone this repository and change/move into the cloned directory in your terminal window

2. Prepare the micro:bit V2 and Kitronik Edge Connector Breakout Board:
Attach the Kitronik Edge Connector Breakout Board to the micro:bit V2. 

3. Take out the 830 point breadboard and lay it on the worktable.

4. Take one M/F 6" Premium Jumper Cable (Yellow) and connect the female end to pin 19 on the Kitronik Breakout Board Edge Connector. 

5. You now have the male end of the Yellow M/F 6" Premium Jumper Cable available. Plug the male end into hole a27 on the breadboard.

6. Take one 4.7k Ohm resistor, and plug it into hole c27 on the breadboard, which is just after the Yellow cable you pluged into breadboard. 

7. Take the other end of the 4.7k Ohm resistor, and plug it into the positive rail of the breadboard that is denoted by (+) on the board with a red line that runs along the entire board. Plug it into the rail to the left of the Yellow wire, for example in (+) on row 23.

8. Take one M/F 6" Premium Jumper Cable (Green) and connect the female end to pin 20 on the Kitronik Breakout Board Edge Connector. 

9. You now have the male end of the Green M/F 6" Premium Jumper Cable available. Plug the male end into hole a29 on the breadboard.

10. Take one 4.7k Ohm resistor, and plug it into hole c29 on the breadboard, which is just after the Yellow cable you pluged into breadboard. 

11. Take the other end of the 4.7k Ohm resistor, and plug it into the positive rail of the breadboard that is denoted by (+) on the board with a red line that runs along the entire board. Plug it into the rail to the right of the White wire, for example in (+) on row 34.

12. Take one M/M 6" Premium Jumper Cable (Yellow) and connect one male end to hole d27 on the breadboard.

13. Take the other end of the M/M 6" Premium Jumper Cable (Yellow) and connect it to the SCL wire (Yellow) of the SHT31 temperature and humidity sensor.

14. Take one M/M 6" Premium Jumper Cable (Green) and connect one male end to hole d27 on the breadboard.

15. Take the other end of the M/M 6" Premium Jumper Cable (Green) and connect it to the SDA wire (White) of the SHT31 temperature and humidity sensor.

16. Take one M/F 6" Premium Jumper Cable (Red) and connect the female end to the 3V pin on the Kitronik Breakout Board Edge Connector.

17. Take the male end of the same M/F 6" Premium Jumper Cable (Red) and connect it to the positive rail (+) at row 3 or the first spot available to the left on the breadboard.

18. Take one M/F 6" Premium Jumper Cable (Black) and connect the female end to the 0V pin on the Kitronik Breakout Board Edge Connector.

19. Take the male end of the same M/F 6" Premium Jumper Cable (Black) and connect it to the negative rail (-) at row 7, or the fifth spot available to the left on the breadboard.

20. Take one M/M 6" Premium Jumper Cable (Red) and connect one male end to the positive (+) rail of the breadboard at row 15. 

21. Take the other male end of the M/M 6" Premium Jumper Cable (Red) and connect it to the VCC (Red Cable) terminal of the SHT31 temperature and humidity sensor.

22. Take a M/M 6" Premium Jumper Cable (Black) and connect one male end to the negative (-) raile off the breadboard at row 40.

23. Take the other end of the M/M 6" Premium Jumper Cable (Black) and connect it to the GND (Black Cable) terminal of the SHT31 temperature and humidity sensor.

24. Connect your micro:bit V2 to your computer using the micro-USB to USB-A cable.

25. Build and Flash your micro:bit V2:

> Run this command from the root cloned directory:

```rust
  - cargo build --features v2 --target thumbv7em-none-eabihf
```

> If you have problems with this, check out the Rust Discovery Book:
- https://docs.rust-embedded.org/discovery/microbit/index.html

> Then run this command from the root cloned directory:

```rust
  - cargo embed --features v2 --target thumbv7em-none-eabihf
```

> If you have problems with this, check out the Rust Discovery Book:
- https://docs.rust-embedded.org/discovery/microbit/index.html

26. The terminal should display the initilization messages and you should see periodic updates of the temperature and humidity data.

27. Do you see the data being collected?
