use std::io;
use std::ops::Range;
use std::os::unix::io::RawFd;
use std::time::Duration;
use std::mem;
use std::ptr;
use std::os::raw::c_long;
use buffer::Buffer;
use super::{ffi, to_result};
#[repr(C)]
#[derive(Default)]
pub struct Event {
  event: ffi::io_event
}

#[repr(C)]
pub struct Operation {
  iocb: ffi::iocb,
  buffer: Buffer
}

impl Operation {

  pub fn create_read(fd: RawFd, range: Range<usize>, mut buffer: Buffer) -> Operation {
    let buffer_ptr = buffer.as_mut_slice().as_mut_ptr();
    let len = range.end - range.start;
    assert!(len <= buffer.capacity());
    Operation {
      iocb: ffi::iocb {
        aio_fildes:     fd as u32,
        aio_lio_opcode: ffi::IOCB_CMD_PREAD as u16,
        aio_buf:        buffer_ptr as u64,
        aio_nbytes:     len as u64,
        aio_offset:     range.start as i64,
        .. Default::default()
      },
      buffer
    }
  }

  pub fn set_event_fd(&mut self, event_fd: RawFd) {
    self.iocb.aio_flags |= ffi::IOCB_FLAG_RESFD;
    self.iocb.aio_resfd = event_fd as u32;
  }

  pub fn into_read_result(mut self, event: &Event) -> io::Result<Buffer> {
    //check event matches this operation?
    if event.event.res < 0 {
      Err(io::Error::from_raw_os_error( - event.event.res as i32))
    }
    else {
      unsafe { self.buffer.set_len(event.event.res as usize) };
      Ok(self.buffer)
    }
  }

  pub fn as_iocb<'a>(&'a self) -> &'a ffi::iocb {
    &self.iocb
  }
}

pub struct Context {
  ctxp: ffi::aio_context_t
}

impl Context {

  pub fn setup(max_operations: u32) -> io::Result<Context> {
    let mut ctxp: ffi::aio_context_t = 0;
    let setup_result = to_result(unsafe { ffi::io_setup(max_operations, &mut ctxp) });
    setup_result.map(|_| Context {ctxp})
  }

  pub fn submit(&self, control_blocks: &[&ffi::iocb]) -> io::Result<usize> {
    let control_blocks_ptr = 
      unsafe { 
        mem::transmute::<*const &ffi::iocb, *mut *mut ffi::iocb>(control_blocks.as_ptr())
      };
    let amount_submitted = to_result(unsafe {
      ffi::io_submit(
        self.ctxp,
        control_blocks.len() as c_long,
        control_blocks_ptr
      )
    });
    amount_submitted.map(|n| n as usize)
  }

  /*
  pub fn cancel(&self, operation: &Operation) -> Result<Event> {
    io_cancel(self.ctxp, operation)
    //
  }
  */

  pub fn get_events<'a>(&self, min_events: usize, events: &'a mut [Event], timeout: Option<Duration>) -> &'a [Event] {
    
    let timeout = timeout.map(|t| {
      ffi::timespec {
        tv_sec:  t.as_secs() as c_long,
        tv_nsec: t.subsec_nanos() as c_long
      }
    });
    let timeout_ptr = timeout
      .map(|ref mut t| t as *mut ffi::timespec)
      .unwrap_or(ptr::null_mut());
    let events_ptr = unsafe {
      mem::transmute::<*mut Event, *mut ffi::io_event>(events.as_mut_ptr())
    };

    let count = unsafe {
      ffi::io_getevents(
        self.ctxp,
        min_events as c_long,
        events.len() as c_long,
        events_ptr,
        timeout_ptr) as usize
    };
    &events[.. count]
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { ffi::io_destroy(self.ctxp) };
  }
}
