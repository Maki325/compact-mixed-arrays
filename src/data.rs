use quote::{format_ident, quote, ToTokens};
use syn::{
  braced,
  parse::{Parse, ParseStream},
  Ident, Token, Type, Visibility,
};

pub struct TypeInfo {
  name: Ident,
  ty: Type,
}

impl Parse for TypeInfo {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let name: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let ty: Type = input.parse()?;

    return Ok(TypeInfo { name, ty });
  }
}

pub struct Data {
  visibility: Visibility,
  name: Ident,
  types: Vec<TypeInfo>,
}

impl Parse for Data {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let visibility: Visibility = input.parse()?;

    let mixed: Ident = input.parse()?;

    if mixed != "mixed" {
      return Err(syn::Error::new_spanned(
        mixed,
        "Not starting with type mixed!",
      ));
    }

    let name: Ident = input.parse()?;

    let content;
    braced!(content in input);

    let types: Vec<TypeInfo> = content
      .parse_terminated(TypeInfo::parse, Token![,])?
      .into_iter()
      .collect();

    return Ok(Self {
      name,
      types,
      visibility,
    });
  }
}

impl ToTokens for Data {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let Data {
      name,
      types,
      visibility,
    } = self;

    let mut len_names = Vec::with_capacity(types.len());
    let mut offset_names = Vec::with_capacity(types.len());

    let mut slice_getters = Vec::with_capacity(types.len());
    let mut sizes = Vec::with_capacity(types.len());

    for TypeInfo { name, ty } in types {
      let len_name = format_ident!("{}_len", name);
      let offset_name = format_ident!("{}_offset", name);
      let fn_mut_name = format_ident!("{}_mut", name);

      sizes.push(quote! {
        let align_of = std::mem::align_of::<#ty>();
        let rest = size % align_of;
        let size = size + if rest > 0 { align_of - rest } else { 0 };

        let #offset_name = size;
        let size = size + std::mem::size_of::<#ty>() * #len_name;
      });

      slice_getters.push(quote! {
        pub fn #fn_mut_name(&self) -> &mut [#ty] {
          return unsafe {
            std::slice::from_raw_parts_mut(
              self.buf.add(self.#offset_name) as *mut #ty,
              self.#len_name,
            )
          };
        }

        pub fn #name(&self) -> &[#ty] {
          return unsafe {
            std::slice::from_raw_parts(
              self.buf.add(self.#offset_name) as *const #ty,
              self.#len_name,
            )
          };
        }
      });

      len_names.push(len_name);
      offset_names.push(offset_name);
    }

    let len_names_struct = format_ident!("{name}Lens");
    let offsets_struct_name = format_ident!("{name}Offsets");

    let data = quote! {
      #visibility struct #offsets_struct_name {
        pub size: usize,
        #(pub #offset_names: usize),*
      }

      #[derive(Clone)]
      #visibility struct #len_names_struct {
        #(pub #len_names: usize),*
      }

      #[derive(Debug)]
      #visibility struct #name {
        layout: std::alloc::Layout,
        size: usize,
        buf: *mut u8,

        #(#len_names: usize,)*
        #(#offset_names: usize,)*
      }

      impl Drop for #name {
        fn drop(&mut self) {
          unsafe {
            std::alloc::dealloc(self.buf, self.layout);
          }
        }
      }

      impl #name {
        #[inline]
        pub const fn calculate_offsets_and_size(#len_names_struct {#(#len_names),*}: #len_names_struct) -> #offsets_struct_name {
          let size = 0;
          #(#sizes)*

          return #offsets_struct_name {size, #(#offset_names),*};
        }

        pub fn new(lens: #len_names_struct) -> Result<Self, std::alloc::LayoutError> {
          let #offsets_struct_name {size, #(#offset_names),*} = Self::calculate_offsets_and_size(lens.clone());
          let #len_names_struct {#(#len_names),*} = lens;

          let layout = std::alloc::Layout::array::<u8>(size)?.pad_to_align();

          let buf = unsafe { std::alloc::alloc_zeroed(layout) };

          return Ok(#name {
            buf,
            layout,
            size,
            #(#len_names,)*
            #(#offset_names),*
          });
        }

        pub fn size(&self) -> usize {
          return self.size;
        }

        pub fn buf(&self) -> *const u8 {
          return self.buf;
        }

        pub fn buf_as_slice(&self) -> &[u8] {
          return unsafe {
            std::slice::from_raw_parts(self.buf, self.size)
          };
        }

        pub fn buf_mut(&self) -> *mut u8 {
          return self.buf;
        }

        pub fn buf_as_slice_mut(&self) -> &mut [u8] {
          return unsafe {
            std::slice::from_raw_parts_mut(self.buf, self.size)
          };
        }

        #(#slice_getters)*
      }
    };

    tokens.extend(data);
  }
}
