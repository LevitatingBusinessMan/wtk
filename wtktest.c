#include "src/ffi/libwtk.h"

void on_click(wtk_button_t button) {
    wtk_button_set_text(button, "clicked");
}

int main(void) {
    wtk_app_t app = wtk_app_sdl_new("WTK button example");
    wtk_button_t button = wtk_button_new("clickme", on_click);
    wtk_app_sdl_add_widget(app, wtk_button_share(button));
    wtk_app_run(app);
    wtk_app_sdl_destroy(app);
}
