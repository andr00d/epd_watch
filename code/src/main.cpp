#include "display.hpp"
#include "io.hpp"

#include <zephyr/kernel.h>

#include <zephyr/logging/log.h>
LOG_MODULE_REGISTER(main);

int main(void)
{
    LOG_INF("Watch turned on");
    Display disp;
    if (!disp.bootup_check()) {return -1;}

    while(true)
    {
        k_msleep(1000);
        LOG_INF("Watch loop");
        disp.prep_clean_update();
        disp.wake();
        disp.update();
        disp.sleep();
        k_msleep(10000);
    }

    return 0;
}
