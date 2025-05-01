use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(RuneAdapter)]
pub fn rune_adapter_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Rune Adapter: Can't parse derive input!");
    impl_rune_adapter(&ast)
}

fn impl_rune_adapter(ast: &syn::DeriveInput) -> TokenStream {
    // let name = &ast.ident;

    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("Rune Adapter: Not a data struct!")
    };
    let get_members = data_struct.fields.members();
    let mod_members = data_struct.fields.members();

    let gen = quote! {
        impl World {
            #(
            #[rune::function]
            fn #get_members(&self, entity: Ent) -> Option<rune::Value> {
                let val = self.0.components.#get_members.get(entity.into())?;
                Some(rune::runtime::to_value(val.clone()).ok()?)
            }
            )*

            pub(crate) fn module() -> Result<Module, rune::ContextError> {
                let mut module = Module::new();
                module.ty::<World>()?;
                #(
                module.function_meta(World::#mod_members)?;
                )*
                module.function_meta(World::query)?;
                Ok(module)
            }
        }
    };
    gen.into()
}
