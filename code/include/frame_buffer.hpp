#ifndef _FRAME_BUFFER_HPP_
#define _FRAME_BUFFER_HPP_

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

class FrameBuffer 
{
public:
    FrameBuffer();
    uint16_t get_text_width(const char* text, uint8_t scale);
    bool draw_text(const char* text, uint8_t x, uint8_t y, uint8_t scale, Anchor anchor);

    bool draw_line(uint8_t x1, uint8_t y1, uint8_t x2, uint8_t y2, uint8_t scale);
    bool draw_rect(uint8_t x1, uint8_t y1, uint8_t x2, uint8_t y2);
    bool draw_arrow(uint8_t x, uint8_t y, uint8_t scale, Direction arrow_dir);

    uint8_t* get_buff_curr();
    uint8_t* get_buff_old();
    void rotate_buffers();

private:

    void set_frame_bit(uint8_t x, uint8_t y, bool val);
    bool get_letter_bit(char letter, uint8_t bit);

    uint8_t get_letter_width(char letter);
    uint16_t get_letter_buff_index(char letter);
    
    static uint8_t buffer_a[BUFFSIZE];
    static uint8_t buffer_b[BUFFSIZE];
    static uint8_t* buffer_curr;
    static uint8_t* buffer_old;
};

#endif