# epd watch
A low power epaper smartwatch running on an NRF52832.  
It has the basic functionalities like telling time, a stopwatch, and an alarm. But also has some fun features like snake.

## hardware
The kicad files for both the charger and the watch are in the hardware folder. The components in the watch already include LCSC values to order from JLCPCB, but its good to check and make sure every component is still available. Several components will also need to be rotated or moved a bit to ensure correct placement. 

Several extra components are also needed, for the display, the GDEW0154M09 is used. And 10x10x1mm magnets are used to keep the watch and charger connected. 

The charger board is a simple pcb to easily connect 4 pogo pins to a pi pico which is flashed with [debugprobe](github.com/raspberrypi/debugprobe).

## running on the watch
A few steps are needed to be able to compile and run the code on the watch. running the following commands installs cargo embed and the correct target:  
```rustup target add thumbv7em-none-eabihf```  
```rustup component add llvm-tools```  
```cargo install cargo-binutils```  
```cargo install cargo-embed```

After this is finished, just place the watch on the charger and run ```cargo embed``` to flash the watch. 

## running on your pc
There is a pc_test folder that allows additions to the watch to be tested a bit more easily than having to constantly wait for the watch to be programmed. simply go into the pc_test folder, type ```cargo run``` and you can test if everything is as you expect.