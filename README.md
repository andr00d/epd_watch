# epd watch
A low power epaper smartwatch running on an NRF52832.  
It has the basic functionalities like telling time, a stopwatch, and an alarm. But also has some fun features like snake.

## running on the watch
A few steps are needed to be able to compile and run the code on the watch. running the following commands installs cargo embed and the correct target:  
```rustup target add thumbv7em-none-eabihf```  
```rustup component add llvm-tools```  
```cargo install cargo-binutils```  
```cargo install cargo-embed```

After this is finished you can hook up your preferred swd programmer to the watch and run ```cargo embed``` to flash the watch. 

## running on your pc
There is a pc_test folder that allows additions to the watch to be tested a bit more easily than having to constantly wait for the watch to be programmed. simply go into the pc_test folder, type ```cargo run``` and you can test if everything is as you expect.