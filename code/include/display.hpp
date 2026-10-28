#ifndef _DISPLAY_HPP_
#define _DISPLAY_HPP_

#include <string.h>
#include <zephyr/types.h>

enum Anchor 
{
    AnchorLeft,
    AnchorCenter,
    AnchorRight
}; 

enum Direction 
{
    DirUp,
    DirRight,
    DirDown,
    DirLeft
}; 

#define SIZE 200
#define BUFFSIZE SIZE*((SIZE+7)/8)
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

    // uint16_t get_text_width(const char* text, uint8_t scale);
    // bool draw_text(const char* text, uint8_t x, uint8_t y, uint8_t scale, Anchor anchor);

    // bool draw_line(uint8_t x1, uint8_t y1, uint8_t x2, uint8_t y2);
    // bool draw_rect(uint8_t x, uint8_t y, uint8_t x2, uint8_t y2);
    // bool draw_arrow(uint8_t x, uint8_t y, uint8_t scale, Direction arrow_dir);


private:
    void set_bit(uint16_t index, uint8_t bit_index);
    bool get_bit(uint16_t index, uint8_t bit_index);
    bool wait_busy();

    bool send_cmd(uint8_t cmd);
    bool send_data_byte(uint8_t data);
    bool send_data_buffer(const uint8_t* data, uint16_t size);

    uint8_t get_letter_width(char letter);
    uint16_t get_letter_buff_index(char letter);
    
    static uint8_t buffer_a[BUFFSIZE];
    static uint8_t buffer_b[BUFFSIZE];
    static uint8_t* buffer_curr;
    static uint8_t* buffer_old;
    bool sleeping;
    bool initialized;
    bool clean_update;
};

#endif