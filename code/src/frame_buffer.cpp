#include "frame_buffer.hpp"
#include "font.hpp"

#include <zephyr/kernel.h>
#include <stdlib.h>
#include <utility>

#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(frame_buff);

uint8_t FrameBuffer::buffer_a[BUFFSIZE];
uint8_t FrameBuffer::buffer_b[BUFFSIZE];
uint8_t* FrameBuffer::buffer_curr = FrameBuffer::buffer_a;
uint8_t* FrameBuffer::buffer_old = FrameBuffer::buffer_b;

FrameBuffer::FrameBuffer()
{
    memset(buffer_curr, 0xFF, BUFFSIZE);
    memset(buffer_old, 0xFF, BUFFSIZE);
}


/////////////////////////////////////////////////

uint16_t FrameBuffer::get_text_width(const char* text, uint8_t scale)
{
    uint16_t total = 0;
    uint16_t padding = scale/2;
    if (padding == 0) {padding = 1;}

    for (size_t i = 0; i < strlen(text); i++)
    {
        char letter = text[i];
        // if (letter == 0) {total += 4;}
        // else
        // {
            total += get_letter_width(letter) * scale;
            total += padding;
        // }
    }

    return total;
}

bool FrameBuffer::draw_text(const char* text, uint8_t x, uint8_t y, uint8_t scale, Anchor anchor)
{
    if (text == "") {return true;}
    uint16_t width = get_text_width(text, scale);

    if (anchor != AnchorLeft)
    {
        uint16_t offset = width;
        if (anchor == AnchorCenter) {offset /= 2;}

        if (offset > x)
        {
            LOG_INF("text too much too the left!");
            return false;
        }
        
        x -= offset;
    }

    if (x + width > SIZE)
    {
        LOG_INF("Text too much to the right. max = 200, actual= x:%i", x + width);
        return false;
    }

    if (y + scale*5 > SIZE)
    {
        LOG_INF("Text too far down, max = 200, actual= y:%i", y + scale*5);
        return false;
    }

    uint8_t x_offset = 0;
    uint8_t padding = scale/2;
    if (padding == 0) {padding = 1;}

    for (size_t i = 0; i < strlen(text); i++)
    {
        uint8_t char_width = get_letter_width(text[i]);
        uint8_t ltr_x_loc = x + x_offset;
        uint8_t ltr_y_loc = y;
        uint8_t ltr_bit = 0;

        for (size_t h = 0; h < 5; h++)
        {
            for (size_t b = 0; b < char_width; b++)
            {
                bool bit_val = get_letter_bit(text[i], ltr_bit);
                if (!bit_val) 
                {
                    uint8_t x1 = ltr_x_loc + (b * scale);
                    uint8_t y1 = ltr_y_loc + (h * scale);
                    draw_rect(x1, y1, x1 + scale, y1 + scale);
                }
                ltr_bit++;
            }
        }

        x_offset += padding + (char_width * scale);
    }

    return true;
}

bool FrameBuffer::draw_line(uint8_t x1, uint8_t y1, uint8_t x2, uint8_t y2, uint8_t scale)
{
    if (x2 < x1 || y2 < y1)
    {
        LOG_INF("X2 is smaller that X1, or Y2 is smaller than Y1.");
        return false;  
    }

    if (x1 < scale || y1 < scale)
    {
        LOG_INF("Line extends too far to the top/left!");
        return false;
    }

    if (x2 + scale > SIZE || y2 + scale > SIZE)
    {
        LOG_INF("Line extends too far to the bottom/right!");
        return false;
    }

    int16_t delta_x = x2 - x1;
    int16_t delta_y = y2 - y1;
    if (delta_x < 0) {delta_x *= -1;}
    if (delta_y > 0) {delta_y *= -1;}

    int16_t step_x = x1 < x2 ? 1 : -1;
    int16_t step_y = y1 < y2 ? 1 : -1;
    int16_t error = delta_y + delta_x;

    while (true)
    {
        draw_rect(x1 - scale, y1 - scale, x1 + scale, y1 + scale);
        int16_t error2 = 2*error;
        
        
        if (error2 >= delta_y)
        {
            if (x1 == x2) {break;}
            error += delta_y;
            x1 += step_x;
        }
        
        if (error2 <= delta_x)
        {
            if (y1 == y2) {break;}
            error += delta_x;
            y1 += step_y;
        }
    }

    return true;
}

bool FrameBuffer::draw_rect(uint8_t x1, uint8_t y1, uint8_t x2, uint8_t y2)
{
    if (x2 < x1 || y2 < y1)
    {
        LOG_INF("X2 is smaller that X1, or Y2 is smaller than Y1.");
        return false;  
    }

    if (x2 > SIZE)
    {
        LOG_INF("Rectangle extends too far to the left!");
        return false;
    }

    if (y2 > SIZE)
    {
        LOG_INF("Rectangle extends too far down!");
        return false;
    }

    uint8_t start_byte = x1 / 8;
    uint8_t end_byte = x2 / 8;

    uint8_t start_bit = x1 % 8;
    uint8_t end_bit = x2 % 8;
    uint8_t start_mask = 0xFF >> start_bit;
    uint8_t end_mask = 0xFF << (7 - end_bit);

    if (start_byte == end_byte)
    {
        uint8_t line_mask = start_mask & end_mask;

        for (int y = y1; y <= y2; y++)
        {
            uint8_t *row = buffer_curr + y * (SIZE / 8);
            row[start_byte] &= ~line_mask;
            continue;
        }

        return true;
    }

    if (end_byte - start_byte == 1)
    {
        for (int y = y1; y <= y2; y++)
        {
            uint8_t *row = buffer_curr + y * (SIZE / 8);
            row[start_byte] &= ~start_mask;
            row[end_byte] &= ~end_mask;
        }

        return true;
    } 


    for (int y = y1; y <= y2; y++)
    {
        uint8_t *row = buffer_curr + y * (SIZE / 8);

        memset(&row[start_byte + 1], 0x0, (end_byte - start_byte) - 1);
        row[start_byte] &= ~start_mask;
        row[end_byte] &= ~end_mask;
    }

    return true;
}

bool FrameBuffer::draw_arrow(uint8_t x, uint8_t y, uint8_t scale, Direction arrow_dir)
{
    if (x < scale || y < scale)
    {
        LOG_INF("Arrow extends too far to the top/left!");
        return false;
    }

    if (x + scale > SIZE || y + scale > SIZE)
    {
        LOG_INF("Arrow extends too far to the bottom/right!");
        return false;
    }

    uint8_t height = scale;
    if (arrow_dir != DirDown) {y -= scale;}
    if (arrow_dir == DirLeft || arrow_dir == DirRight) {height = height*2 - 1;}
    

    for (int h = 0; h <= height; h++)
    {
        uint8_t row_start = 0;
        uint8_t row_end = 0;

        if (arrow_dir == DirUp)
        {
            row_start = x - h;
            row_end = x + h;
        }

        else if (arrow_dir == DirDown)
        {
            row_start = x - scale + h;
            row_end = x + scale - h;
        }

        else if (arrow_dir == DirLeft)
        {
            row_start = x - scale + abs(h - scale);
            row_end = x;
        }

        else if (arrow_dir == DirRight)
        {
            row_start = x;
            row_end = x + scale - abs(h - scale);
        }

        draw_rect(row_start, y + h, row_end, y + h);
    }



    return true;
}

uint8_t* FrameBuffer::get_buff_curr()
{
    return buffer_curr;
}

uint8_t* FrameBuffer::get_buff_old()
{
    return buffer_old;
}

void FrameBuffer::rotate_buffers()
{
    std::swap(buffer_curr, buffer_old);
    memset(buffer_curr, 0xFF, BUFFSIZE);
}


/////////////////////////////////////////////////

void FrameBuffer::set_frame_bit(uint8_t x, uint8_t y, bool val)
{
    uint8_t byte_index = (x / 8) + (y * (SIZE / 8));
    uint8_t bit_index = x % 8;
    uint8_t mask = 0x80 >> bit_index;

    if (val)
    {
        buffer_curr[byte_index] |= mask;
    }
    else
    {
        buffer_curr[byte_index] &= ~mask;
    }
}

bool FrameBuffer::get_letter_bit(char letter, uint8_t bit)
{
    uint16_t index = get_letter_buff_index(letter);
    index += (bit / 8);

    uint8_t mask = 0x80 >> (bit % 8);
    return (FONT[index] & mask) > 0;
}

uint8_t FrameBuffer::get_letter_width(char letter)
{
    uint8_t width = 3;
    if (letter >= 'a' && letter <= 'z') {letter -= 32;}
    
    if (letter == 'I' || letter == ':' || letter == '\'' || letter == '1') {width = 1;}
    else if (letter == 'J' || letter == '`') {width = 2;}
    else if (letter == 'W' || letter == 'Q' || letter == 'M') {width = 4;}

    return width;
}

uint16_t FrameBuffer::get_letter_buff_index(char letter)
{
    uint16_t index = 0;
    if (letter >= 'a' && letter <= 'z') {letter -= 32;}

    if (letter >= ' '  && letter <= '`')
    {
        index = (letter - 31) * 2;

        if (letter > 'M') {index += 1;}
        if (letter > 'Q') {index += 1;}
        if (letter > 'W') {index += 1;}
    }

    return index;
}
