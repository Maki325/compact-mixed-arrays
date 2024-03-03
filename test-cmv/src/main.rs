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
    let chars = container.get_mut_chars_slice();
    println!("chars: {chars:#?}");
    for c in chars.iter_mut() {
      *c = 67;
    }
    println!("chars: {chars:#?}");
  }

  {
    let lines = container.get_mut_lines_slice();
    println!("lines: {lines:#?}");
    for line in lines.iter_mut() {
      *line = 32;
    }
    println!("lines: {lines:#?}");
  }
}
