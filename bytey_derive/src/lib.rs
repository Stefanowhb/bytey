#![feature(proc_macro_diagnostic)]

mod byte_buffer_read;
mod byte_buffer_write;
mod field_wrapper;
mod source;
mod symbols;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(ByteBufferWrite, attributes(bytey))]
pub fn derive_byte_buffer_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    byte_buffer_write::expand_derive_byte_buffer_write(&input).into()
}

#[proc_macro_derive(ByteBufferRead, attributes(bytey))]
pub fn derive_byte_buffer_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    byte_buffer_read::expand_derive_byte_buffer_read(&input).into()
}
