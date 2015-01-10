use common::*;
use gl;
use interval_timer::IntervalTimer;
use process_event::process_event;
use render::render;
use sdl2;
use sdl2::event::Event;
use state::App;
use std::mem;
use std::time::duration::Duration;
use std::io::timer;
use stopwatch::TimerSet;
use time;
use update::update;
use yaglw::gl_context::GLContext;

pub const FRAMES_PER_SECOND: u64 = 30;
pub const UPDATES_PER_SECOND: u64 = 30;

pub fn main() {
  debug!("starting");

  sdl2::init(sdl2::INIT_EVERYTHING);

  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3);
  sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
  sdl2::video::gl_set_attribute(
    sdl2::video::GLAttr::GLContextProfileMask,
    sdl2::video::GLProfile::GLCoreProfile as isize
  );

  let mut window =
    sdl2::video::Window::new(
      "playform",
      sdl2::video::WindowPos::PosCentered,
      sdl2::video::WindowPos::PosCentered,
      WINDOW_WIDTH as isize,
      WINDOW_HEIGHT as isize,
      sdl2::video::OPENGL,
    ).unwrap();

  // Send text input events.
  sdl2::keyboard::start_text_input();

  {
    let _sdl_gl_context = window.gl_create_context().unwrap();

    // Load the OpenGL function pointers.
    gl::load_with(|s| unsafe {
      mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    let (gl, mut gl_context) = unsafe {
      GLContext::new()
    };

    let timers = TimerSet::new();
    let mut app = App::new(&gl, &mut gl_context, &timers);

    let mut render_timer;
    let mut update_timer;
    {
      let now = time::precise_time_ns();
      let nanoseconds_per_second = 1000000000;
      render_timer = IntervalTimer::new(nanoseconds_per_second / FRAMES_PER_SECOND, now);
      update_timer = IntervalTimer::new(nanoseconds_per_second / UPDATES_PER_SECOND, now);
    }

    let mut has_focus = true;

    loop {
      let updates = update_timer.update(time::precise_time_ns());
      if updates > 0 {
        update(&mut app);
      }

      let renders = render_timer.update(time::precise_time_ns());
      if renders > 0 {
        render(&mut app);
        // swap buffers
        window.gl_swap_window();
      }

      loop {
        match sdl2::event::poll_event() {
          Event::None => {
            break;
          },
          Event::Quit(_) => {
            return;
          }
          Event::AppTerminating(_) => {
            return;
          }
          Event::Window(_, _, event_id, _, _) => {
            // Manage has_focus so that we don't capture the cursor when the
            // window is in the background
            match event_id {
              sdl2::event::WindowEventId::FocusGained => {
                has_focus = true;
                sdl2::mouse::show_cursor(false);
              }
              sdl2::event::WindowEventId::FocusLost => {
                has_focus = false;
                sdl2::mouse::show_cursor(true);
              }
              _ => {}
            }
          }
          event => {
            if has_focus {
              process_event(&mut app, &mut window, event);
            }
          },
        }
      }
      timer::sleep(Duration::microseconds(10));
    }
  }

  debug!("finished");
}
