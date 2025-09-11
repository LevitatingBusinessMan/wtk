require 'ffi'

module WTK
  extend FFI::Library
  
  # Load the shared library (adjust path as needed)
  ffi_lib './libwtk.so'  # or 'libdylib' on macOS, 'dll' on Windows
  
  # Define the C types
  typedef :pointer, :wtk_app_t
  typedef :pointer, :wtk_widget_t  
  typedef :pointer, :wtk_button_t
  
  # Define the callback type
  callback :wtk_button_callback_t, [:wtk_button_t], :void
  
  # Attach the C functions
  attach_function :wtk_app_sdl_new, [:string], :wtk_app_t
  attach_function :wtk_app_sdl_destroy, [:wtk_app_t], :void
  attach_function :wtk_app_run, [:wtk_app_t], :void
  attach_function :wtk_button_new, [:string, :wtk_button_callback_t], :wtk_button_t
  attach_function :wtk_button_share, [:wtk_button_t], :wtk_widget_t
  attach_function :wtk_button_set_text, [:wtk_button_t, :string], :void
  attach_function :wtk_app_sdl_add_widget, [:wtk_app_t, :wtk_widget_t], :void
end

def main
  include WTK
  on_click = proc do |button| 
    wtk_button_set_text(button, "clicked")
  end
  app = wtk_app_sdl_new("WTK button example")
  button = wtk_button_new("clickme", on_click)
  widget = wtk_button_share(button)
  wtk_app_sdl_add_widget(app, widget)
  wtk_app_run(app)
  wtk_app_sdl_destroy(app)
end

# Run the program
main if __FILE__ == $0

