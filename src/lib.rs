use std::{
  alloc::{Layout, LayoutError},
  slice,
};

// chars: u8
// line_lens: usize
// ends_with_newline: bool
#[derive(Debug)]
struct Container {
  layout: Layout,
  chars_len: usize,
  lines_len: usize,
  ends_with_newline_len: usize,
  buf: *mut u8,
}

impl Container {
  pub fn new(
    chars_len: usize,
    lines_len: usize,
    ends_with_newline_len: usize,
  ) -> Result<Self, LayoutError> {
    // TODO: Add offsets at the end of each type
    // Such that it's aligned to the next type
    // Example: If we have 3 u8 in a row, and the next
    // Type is usize, we'll fill in until it's std::mem::align_of::<usize>()
    let len = std::mem::align_of::<u8>() * chars_len
      + std::mem::align_of::<usize>() * lines_len
      + std::mem::align_of::<bool>() * ends_with_newline_len;

    let layout = Layout::array::<u8>(len)?;
    println!("Before: {layout:#?}");

    let layout = layout.pad_to_align();
    println!("After: {layout:#?}");

    let buf = unsafe { std::alloc::alloc_zeroed(layout) };

    return Ok(Container {
      buf,
      chars_len,
      lines_len,
      ends_with_newline_len,
      layout,
    });
  }

  pub fn get_chars_slice(&self) -> &mut [u8] {
    return unsafe { slice::from_raw_parts_mut(self.buf, self.chars_len) };
  }

  pub fn get_lines_slice(&self) -> &mut [usize] {
    return unsafe {
      slice::from_raw_parts_mut(self.buf.add(self.chars_len) as *mut usize, self.lines_len)
    };
  }

  pub fn get_ends_with_newline_slice(&self) -> &mut [bool] {
    return unsafe {
      slice::from_raw_parts_mut(
        self.buf.add(self.chars_len + self.lines_len) as *mut bool,
        self.ends_with_newline_len,
      )
    };
  }
}

impl Drop for Container {
  fn drop(&mut self) {
    unsafe {
      std::alloc::dealloc(self.buf, self.layout);
    }
  }
}
