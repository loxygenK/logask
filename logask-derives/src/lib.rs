#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(WithId)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let gen = quote! {
        impl crate::model::id::WithId for #ident {
            fn id(&self) -> &Id<Self> {
                &self.id
            }

            fn take_id(self) -> Id<Self> {
                self.id
            }
        }
    };

    TokenStream::from(gen)
}
