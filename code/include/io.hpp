#ifndef _IO_HPP_
#define _IO_HPP_

#include "events.hpp"

class IO 
{
public:
    IO();
    ~IO();

    Event wait_for_event();
    Event wait_For_event_ms();
    bool buffer_has_ev();
private:

};

#endif