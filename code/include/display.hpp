#ifndef _DISPLAY_HPP_
#define _DISPLAY_HPP_

#include "frame_buffer.hpp"

#include <string.h>
#include <zephyr/types.h>

#define MAX_LOOPS 200

class Display 
{
public:
    Display();
    bool bootup_check();

    bool wake();
    void sleep();
    bool update();

    void prep_clean_update();
    FrameBuffer* get_frame_buff();

private:
    bool wait_busy();

    bool send_cmd(uint8_t cmd);
    bool send_data_byte(uint8_t data);
    bool send_data_buffer(const uint8_t* data, uint16_t size);

    bool sleeping;
    bool initialized;
    bool clean_update;

    FrameBuffer buff;
};

#endif