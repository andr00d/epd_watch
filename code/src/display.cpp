#include "display.hpp"
#include "display_data.hpp"

#include <zephyr/kernel.h>
#include <zephyr/drivers/gpio.h>
#include <zephyr/drivers/spi.h>
#include <utility>

#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(disp);

#define CONFIG  SPI_WORD_SET(8) | SPI_OP_MODE_MASTER | SPI_TRANSFER_MSB
struct spi_dt_spec spi_dev = SPI_DT_SPEC_GET(DT_NODELABEL(dispdev), CONFIG);

static const struct gpio_dt_spec pin_disp_rst = GPIO_DT_SPEC_GET(DT_NODELABEL(disp_rst), gpios);
static const struct gpio_dt_spec pin_disp_dc = GPIO_DT_SPEC_GET(DT_NODELABEL(disp_dc), gpios);
static const struct gpio_dt_spec pin_disp_cs = GPIO_DT_SPEC_GET(DT_NODELABEL(disp_cs), gpios);
static const struct gpio_dt_spec pin_disp_busy = GPIO_DT_SPEC_GET(DT_NODELABEL(disp_busy), gpios);

uint8_t Display::buffer_a[BUFFSIZE];
uint8_t Display::buffer_b[BUFFSIZE];
uint8_t* Display::buffer_curr = Display::buffer_a;
uint8_t* Display::buffer_old = Display::buffer_b;

Display::Display()//: spi_dev(dev)
{

    memset(buffer_curr, 0xFF, BUFFSIZE);
    memset(buffer_old, 0xFF, BUFFSIZE);

    sleeping = true;
    clean_update = true;
}

bool Display::bootup_check()
{
    if (!spi_is_ready_dt(&spi_dev)) 
    {
        LOG_ERR("Display SPI not ready!");
        return false;
    }

	bool ret_rst = gpio_is_ready_dt(&pin_disp_rst);
	bool ret_dc = gpio_is_ready_dt(&pin_disp_dc);
	bool ret_cs = gpio_is_ready_dt(&pin_disp_cs);
	bool ret_busy = gpio_is_ready_dt(&pin_disp_busy);

    if(!ret_rst || !ret_dc || !ret_cs || !ret_busy)
    {
        LOG_ERR("Display IO pins not ready! (rst=%i, dc=%i, cs=%i, busy=%i)",
                                        ret_rst, ret_dc, ret_cs, ret_busy);
        return false; 
    }

    int err_rst = gpio_pin_configure_dt(&pin_disp_rst, GPIO_OUTPUT_INACTIVE);
    int err_dc = gpio_pin_configure_dt(&pin_disp_dc, GPIO_OUTPUT_INACTIVE);
    int err_cs = gpio_pin_configure_dt(&pin_disp_cs, GPIO_OUTPUT_INACTIVE);
    int err_busy = gpio_pin_configure_dt(&pin_disp_busy, GPIO_INPUT);

    if(err_rst != 0 || err_dc != 0 || err_cs != 0 || err_busy != 0)
    {
        LOG_ERR("Unable to configure display IO pins! (rst=%i, dc=%i, cs=%i, busy=%i)",
                                        err_rst, err_dc, err_cs, err_busy);
        return false; 
    }

    return true;
}

/////////////////////////////////////////////////

bool Display::wake()
{
    bool init_finished = false;

    while(!init_finished)
    {
        gpio_pin_set_dt(&pin_disp_rst, 0);
        k_msleep(10);
        gpio_pin_set_dt(&pin_disp_rst, 1);
        k_msleep(10);

        send_cmd(PANEL_SETTING);
        if(clean_update) {send_data_byte(0xdf);}
        else {send_data_byte(0xff);}
        
        send_data_byte(0x0e);
        send_cmd(0x4d);
        send_data_byte(0x55);
        send_cmd(0xaa);
        send_data_byte(0x0f);
        send_cmd(0xe9);
        send_data_byte(0x02);
        send_cmd(0xb6);
        send_data_byte(0x11);
        send_cmd(0xf3);
        send_data_byte(0x0a);
        send_cmd(RESOLUTION_SETTING);
        send_data_byte(0xc8);
        send_data_byte(0x00);
        send_data_byte(0xc8);
        send_cmd(TCON_SETTING);
        send_data_byte(0x00);
        send_cmd(VCOM_DATA_INTERVAL);
        send_data_byte(0x97);
        send_cmd(POWER_SAVING);
        send_data_byte(0x00);

        if(!clean_update)
        {
            send_cmd(LUT_VCOM);
            send_data_buffer(LUT_ARR_DC, 56);
            send_cmd(LUT_WW);
            send_data_buffer(LUT_ARR_WW, 42);
            send_cmd(LUT_BW);
            send_data_buffer(LUT_ARR_BW, 56);
            send_cmd(LUT_WB);
            send_data_buffer(LUT_ARR_WB, 56);
            send_cmd(LUT_BB);
            send_data_buffer(LUT_ARR_BB, 56);
        }
        else 
        {
            clean_update = false;
        }
        
        send_cmd(POWER_ON);
        init_finished = wait_busy();
        if (!init_finished) {continue;}
        sleeping = false;
    }

    return true;
}

void Display::sleep()
{
    sleeping = true;
    send_cmd(POWER_OFF);

    wait_busy();
    k_msleep(10);
    send_cmd(DEEP_SLEEP);
    send_data_byte(0xA5);
}

bool Display::update()
{
    bool is_updated = false;

    while(!is_updated)
    {
        if(sleeping) {wake();}

        if (!send_cmd(DATA_TRANSMISSION_2) ||
            !send_data_buffer(buffer_old, BUFFSIZE))
        {
            LOG_ERR("Update error: unable to send old frame");
            return false;
        }

        if (!send_cmd(DATA_TRANSMISSION_1) ||
            !send_data_buffer(buffer_curr, BUFFSIZE))
        {
            LOG_ERR("Update error: unable to send new frame");
            return false;
        }

        send_cmd(DISPLAY_REFRESH);
        is_updated = wait_busy();
        if (!is_updated)
        {
            sleeping = true;
            clean_update = true;
            LOG_WRN("Busy wait timeout, trying again.");
            continue;
        }

        std::swap(buffer_curr, buffer_old);
        memset(buffer_curr, 0x0, BUFFSIZE);
        return true;
    }

    return false;
}

void Display::prep_clean_update()
{
    clean_update = true;
}

/////////////////////////////////////////////////

bool Display::wait_busy()
{
    uint16_t loop_cnt = 0;
    k_msleep(10);

    while(!gpio_pin_get_dt(&pin_disp_busy))
    {
        k_msleep(25);
        loop_cnt++;

        if(loop_cnt > MAX_LOOPS) {return false;}
    }

    return true;
}

bool Display::send_cmd(uint8_t cmd)
{
    gpio_pin_set_dt(&pin_disp_cs, 0);
    gpio_pin_set_dt(&pin_disp_dc, 0);

    uint8_t arr_cmd[] = {cmd};
    struct spi_buf buff_cmd = {.buf = arr_cmd, .len = 1};
    struct spi_buf_set set_cmd = {.buffers = &buff_cmd, .count = 1};

    int err = spi_write_dt(&spi_dev, &set_cmd);
    if (err) 
    {   
        LOG_ERR("SPI write error during cmd. (%i)", err);
        return false; 
    }

    gpio_pin_set_dt(&pin_disp_cs, 1);
    return true;
}

bool Display::send_data_byte(uint8_t data)
{
    uint8_t arr_data[] = {data};
    return send_data_buffer(arr_data, 1);
}

bool Display::send_data_buffer(const uint8_t* data, uint16_t size)
{
    gpio_pin_set_dt(&pin_disp_cs, 0);
    gpio_pin_set_dt(&pin_disp_dc, 1);
    
    struct spi_buf buff_data = {.buf = const_cast<uint8_t *>(data), .len = size};
    struct spi_buf_set set_data = {.buffers = &buff_data, .count = 1};

    int err = spi_write_dt(&spi_dev, &set_data);
    if (err) 
    {   
        LOG_ERR("SPI write error during ddata. (%i)", err);
        return false; 
    }

    gpio_pin_set_dt(&pin_disp_cs, 1);
    return true;
}