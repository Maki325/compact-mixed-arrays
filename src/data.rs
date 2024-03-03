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

    let mut variable_names = Vec::with_capacity(types.len() * 2);
    let mut len_names = Vec::with_capacity(types.len());
    let mut lens = Vec::with_capacity(types.len());
    let mut variables = Vec::with_capacity(types.len() * 2);

    let mut slice_getters = Vec::with_capacity(types.len());
    let mut sizes = Vec::with_capacity(types.len());

    for TypeInfo { name, ty } in types {
      let len_name = format_ident!("{}_len", name);
      let offset_name = format_ident!("{}_offset", name);
      let fn_mut_name = format_ident!("get_mut_{}_slice", name);
      let fn_name = format_ident!("get_{}_slice", name);

      lens.push(quote! {#len_name: usize});
      len_names.push(len_name.clone());
      variables.push(quote! {#len_name: usize});
      variables.push(quote! {#offset_name: usize});
      sizes.push(
        quote! {let #offset_name = size; let size = size + std::mem::align_of::<#ty>() * #len_name;},
      );
      variable_names.push(len_name.clone());
      variable_names.push(offset_name.clone());

      slice_getters.push(quote! {
        pub fn #fn_mut_name(&self) -> &mut [#ty] {
          return unsafe {
            std::slice::from_raw_parts_mut(
              self.buf.add(self.#offset_name) as *mut #ty,
              self.#len_name,
            )
          };
        }

        pub fn #fn_name(&self) -> &[#ty] {
          return unsafe {
            std::slice::from_raw_parts(
              self.buf.add(self.#offset_name) as *mut #ty,
              self.#len_name,
            )
          };
        }
      });
    }

    let len_names_struct = format_ident!("{name}Lens");

    let data = quote! {
      #visibility struct #len_names_struct {
        #(#lens),*
      }

      #[derive(Debug)]
      #visibility struct #name {
        layout: std::alloc::Layout,
        buf: *mut u8,

        #(#variables),*
      }

      impl Drop for #name {
        fn drop(&mut self) {
          unsafe {
            std::alloc::dealloc(self.buf, self.layout);
          }
        }
      }

      impl #name {
        pub fn new(#len_names_struct {#(#len_names),*}: #len_names_struct) -> Result<Self, std::alloc::LayoutError> {
          let size = 0;
          #(#sizes)*
          let layout = std::alloc::Layout::array::<u8>(size)?.pad_to_align();

          let buf = unsafe { std::alloc::alloc_zeroed(layout) };

          return Ok(#name {
            buf,
            layout,
            #(#variable_names),*
          });
        }

        #(#slice_getters)*
      }
    };

    tokens.extend(data);
  }
}
