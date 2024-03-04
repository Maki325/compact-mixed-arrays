compact_mixed_vectors::create_container! {
  mixed Name {
    chars: u8,
    lines: usize,
    new_lines: bool,
  }

  mixed Name2 {
    chars: u8,
  }
}

fn main() {
  let container = Name::new(NameLens {
    chars_len: 3,
    lines_len: 9,
    new_lines_len: 2,
  })
  .unwrap();
  println!("container: {container:#?}");

  {
    let chars = container.chars_mut();
    println!("chars: {chars:#?}");
    for c in chars.iter_mut() {
      *c = 67;
    }
    println!("chars: {chars:#?}");
  }

  {
    let lines = container.lines_mut();
    println!("lines: {lines:#?}");
    for line in lines.iter_mut() {
      *line = 32;
    }
    println!("lines: {lines:#?}");
  }

  {
    let new_lines = container.new_lines_mut();
    println!("new_lines: {new_lines:#?}");
    for (i, is_newline) in new_lines.iter_mut().enumerate() {
      *is_newline = i % 2 == 0;
    }
    println!("new_lines: {new_lines:#?}");
  }

  println!("{:#?}", container.buf_as_slice());
}
