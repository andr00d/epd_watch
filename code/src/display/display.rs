use nrf52832_hal::gpio::Level;
use nrf52832_hal::spim::Spim;
use embedded_hal::digital::{InputPin, OutputPin, PinState};
use embedded_hal::spi::SpiBus;
// use rtt_target::rprintln;

use crate::display::{Display, BUFFSIZE};
use crate::display::DispPins;
use crate::Io;

impl Display 
{
    pub fn new(spim: nrf52832_hal::pac::SPIM2, pins: DispPins) -> Display
    {
        let buffer_curr: [u8; BUFFSIZE] = [0xFF; BUFFSIZE];
        let buffer_old: [u8; BUFFSIZE] = [0xff; BUFFSIZE];

        let _power = pins.power.into_push_pull_output(Level::High);
        let busy = pins.busy.into_floating_input();
        let res = pins.res.into_push_pull_output(Level::Low);
        let cs = pins.cs.into_push_pull_output(Level::Low);
        let dc = pins.dc.into_push_pull_output(Level::Low);
        cortex_m::asm::delay(660_000);

        let clk = pins.clk.into_push_pull_output(Level::Low);
        let mosi = pins.mosi.into_push_pull_output(Level::High);
        let miso = pins.miso.into_floating_input();

        let pins = nrf52832_hal::spim::Pins 
        {
            sck: Some(clk),
            miso: Some(miso),
            mosi: Some(mosi),
        };

        let spi = Spim::new(
            spim,
            pins,
            nrf52832_hal::spim::Frequency::M8,
            nrf52832_hal::spim::MODE_0,
            0,
        );
        
        return Display{
            buffer_curr: buffer_curr,
            buffer_old: buffer_old,
            // power: power,
            spi: spi,
            busy: busy, 
            res: res,
            cs: cs,
            dc: dc,
            sleeping: true,
            clean_update: true,
        };
    } 

    pub fn init(&mut self, io: &mut Io)
    {        
        // Module reset (At least 10ms delay between)
        _ = self.res.set_state(PinState::Low);
        io.waitms(10);
        _ = self.res.set_state(PinState::High);
        io.waitms(10);

        // startup sequence
        // raw hex commands are undocumented but needed.
        self.send_cmd(Self::PANEL_SETTING);
        if self.clean_update {self.send_data(&[0xdf]);}
        else {self.send_data(&[0xff]);}
        self.send_data(&[0x0e]);
        self.send_cmd(0x4d);
        self.send_data(&[0x55]);
        self.send_cmd(0xaa);
        self.send_data(&[0x0f]);
        self.send_cmd(0xe9);
        self.send_data(&[0x02]);
        self.send_cmd(0xb6);
        self.send_data(&[0x11]);
        self.send_cmd(0xf3);
        self.send_data(&[0x0a]);
        self.send_cmd(Self::RESOLUTION_SETTING);
        self.send_data(&[0xc8]);
        self.send_data(&[0x00]);
        self.send_data(&[0xc8]);
        self.send_cmd(Self::TCON_SETTING);
        self.send_data(&[0x00]);
        self.send_cmd(Self::VCOM_DATA_INTERVAL);
        self.send_data(&[0x97]);
        self.send_cmd(Self::POWER_SAVING);
        self.send_data(&[0x00]);

        if !self.clean_update
        {
            self.send_cmd(Self::LUT_VCOM);
            self.send_data(&Self::LUT_ARR_DC);
            self.send_cmd(Self::LUT_WW);
            self.send_data(&Self::LUT_ARR_WW);
            self.send_cmd(Self::LUT_BW);
            self.send_data(&Self::LUT_ARR_BW);
            self.send_cmd(Self::LUT_WB);
            self.send_data(&Self::LUT_ARR_WB);
            self.send_cmd(Self::LUT_BB);
            self.send_data(&Self::LUT_ARR_BB);
        }
        else {self.clean_update = false;}

        // if it hangs here, good luck debugging!
        self.send_cmd(Self::POWER_ON);
        self.wait_busy(io);
        self.sleeping = false;
    }

    pub fn sleep(&mut self, io: &mut Io)
    {
        self.sleeping = true;
        self.send_cmd(Self::POWER_OFF);
        
        // with the normal busy loop, the display goes fucky
        // (and somehow, only when rtt is not attached)
        // TODO: find out why
        cortex_m::asm::delay(64_000);
        while self.busy.is_low().unwrap()
        {
            cortex_m::asm::delay(64_000);
        }
        io.waitms(5);
        
        self.send_cmd(Self::DEEP_SLEEP);
        self.send_data(&[0xA5]); // default
    }

    pub fn update(&mut self, io: &mut Io)
    {
        if self.sleeping {self.init(io);}

        self.send_cmd(Self::DATA_TRANSMISSION_2);
        
        _ = self.cs.set_state(PinState::Low);
        _ = self.dc.set_state(PinState::High);
        _ = SpiBus::write(&mut self.spi, &self.buffer_curr);
        _ = self.cs.set_state(PinState::High);

        self.send_cmd(Self::DATA_TRANSMISSION_1);
        
        _ = self.cs.set_state(PinState::Low);
        _ = self.dc.set_state(PinState::High);
        _ = SpiBus::write(&mut self.spi, &self.buffer_old);
        _ = self.cs.set_state(PinState::High);

        self.send_cmd(Self::DISPLAY_REFRESH);
        self.wait_busy(io);

        (self.buffer_curr, self.buffer_old) = (self.buffer_old, self.buffer_curr);
        self.buffer_curr.fill(0xff);
    }

    pub fn set_clean_update(&mut self)
    {
        self.clean_update = true;
    }

    ////////////////////////////////////
    // used a lot by other display functions.

    pub(super) fn set_bit(&mut self, index: usize, bit_index: u8, value: bool) 
    {
        let clr_mask = 0xff ^ (0x80 >> bit_index);
        let set_mask = ((value as u8) << 7) >> bit_index;
    
        self.buffer_curr[index] &= clr_mask;
        self.buffer_curr[index] |= set_mask;
    }

    pub(super) fn get_bit(&mut self, arr: &[u8], index: usize, bit_index: u8) -> bool
    {
        let mask = 0x80 >> bit_index;
        return (arr[index] & mask) > 0;
    }

    ////////////////////////////////////

    fn send_cmd(&mut self, cmd: u8)
    {
        _ = self.cs.set_state(PinState::Low);
        _ = self.dc.set_state(PinState::Low);
        cortex_m::interrupt::free(|_| {  _ = SpiBus::write(&mut self.spi, &[cmd]); });
        _ = self.cs.set_state(PinState::High);
    }

    fn send_data(&mut self, data: &[u8])
    {
        _ = self.cs.set_state(PinState::Low);
        _ = self.dc.set_state(PinState::High);
        cortex_m::interrupt::free(|_| { _ = SpiBus::write(&mut self.spi, data); });
        _ = self.cs.set_state(PinState::High);
    }

    fn wait_busy(&mut self, io: &mut Io)
    {
        io.waitms(10);
        while self.busy.is_low().unwrap()
        {
            io.waitms(10);
        }
    }

    ////////////////////////////////////
    
    // GDEW0154M09 commands
    const PANEL_SETTING: u8 =               0x00;
    // const POWER_SETTING: u8 =               0x01;
    const POWER_OFF: u8 =                   0x02;
    // const POWER_OFF_SEQ_SETTING: u8 =       0x03;
    const POWER_ON: u8 =                    0x04;
    // const POWER_ON_MEASURE: u8 =            0x05;
    // const BOOSTER_SOFT_START: u8 =          0x06;
    const DEEP_SLEEP: u8 =                  0x07;
    const DATA_TRANSMISSION_1: u8 =         0x10;
    // const DATA_STOP: u8 =                   0x11;
    const DISPLAY_REFRESH: u8 =             0x12;
    const DATA_TRANSMISSION_2: u8 =         0x13;
    // const AUTO_SEQUENCE: u8 =               0x17;
    const LUT_VCOM: u8 =                    0x20;
    const LUT_WW: u8 =                      0x21;
    const LUT_BW: u8 =                      0x22;
    const LUT_WB: u8 =                      0x23;
    const LUT_BB: u8 =                      0x24;
    // const LUT_OPTION: u8 =                  0x2A;
    // const PLL_CONTROL: u8 =                 0x30;
    // const TEMP_SENSOR_CALLIBRATION: u8 =    0x40;
    // const TEMP_SENSOR_SELECT: u8 =          0x41;
    // const TEMP_SENSOR_WRITE: u8 =           0x42;
    // const TEMP_SENSOR_READ: u8 =            0x43;
    // const PANEL_BREAK_CHECK: u8 =           0x44;
    const VCOM_DATA_INTERVAL: u8 =          0x50;
    // const LOWER_POWER_DETECT: u8 =          0x51;
    const TCON_SETTING: u8 =                0x60;
    const RESOLUTION_SETTING: u8 =          0x61;
    // const GATE_SOURCE_START: u8 =           0x65;
    // const REVISION: u8 =                    0x70;
    // const GET_STATUS: u8 =                  0x71;
    // const AUTO_MEASURE_VCOM: u8 =           0x80;
    // const VCOM_VALUE: u8 =                  0x81;
    // const VCOM_DC_SETTINGS: u8 =            0x82;
    // const PARTIAL_WINDOW: u8 =              0x90;
    // const PARTIAL_IN: u8 =                  0x91;
    // const PARTIAL_OUT: u8 =                 0x92;
    // const PROGRAM_MODE: u8 =                0xA0;
    // const ACTIVE_PROGRAMMING: u8 =          0xA1;
    // const READ_OTP_DATA: u8 =               0xA2;
    // const CASCADE_SETTING: u8 =             0xE0;
    const POWER_SAVING: u8 =                0xE3;
    // const LVD_VOLTAGE_SELECT: u8 =          0xE4;
    // const FORCE_TEMP: u8 =                  0xE5;


    // LUTs from the STM32 sample on the GDEW0154m09 gooddisplay page
    const LUT_ARR_DC: [u8; 56] =
    [
        0x01, 0x04, 0x04, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    const LUT_ARR_WW: [u8; 42] =
    [
        0x01, 0x04, 0x04, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ];
    
    const LUT_ARR_BW: [u8; 56] =
    [
        0x01, 0x84, 0x84, 0x83, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ];
    
    const LUT_ARR_WB: [u8; 56] =
    [
        0x01, 0x44, 0x44, 0x43, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ];
    
    const LUT_ARR_BB: [u8; 56] =
    [
        0x01, 0x04, 0x04, 0x03, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
    ];
}