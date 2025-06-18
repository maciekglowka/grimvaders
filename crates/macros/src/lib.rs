use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(RuneAdapter)]
pub fn rune_adapter_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Rune Adapter: Can't parse derive input!");
    impl_rune_adapter(&ast)
}

fn impl_rune_adapter(ast: &syn::DeriveInput) -> TokenStream {
    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("Rune Adapter: Not a data struct!")
    };
    let members = data_struct.fields.members();

    let gen = quote! {
        impl World {
            #[rune::function]
            fn get(&self, component: String, entity: &Ent) -> Option<rune::Value> {
                match component.as_str() {
                    #(stringify!(#members) => Some(
                            rune::runtime::to_value(self.0.components.#members.get(entity.into())?.clone()).ok()?
                        ),
                    )*
                    _ => None
                }
            }

            pub(crate) fn module() -> Result<Module, rune::ContextError> {
                let mut module = Module::new();
                module.ty::<World>()?;
                module.function_meta(World::get)?;
                module.function_meta(World::get_unit_at)?;
                module.function_meta(World::get_tile_at)?;
                module.function_meta(World::get_player_in_front)?;
                module.function_meta(World::get_adjacent_players)?;
                module.function_meta(World::get_players_in_column)?;
                module.function_meta(World::get_players_with_tag)?;
                module.function_meta(World::is_in_front)?;
                module.function_meta(World::is_adjacent)?;
                module.function_meta(World::query)?;
                module.function_meta(World::get_current_food)?;
                Ok(module)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(ComponentGen)]
pub fn component_gen_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_component_gen(&ast)
}

fn impl_component_gen(ast: &syn::DeriveInput) -> TokenStream {
    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("Not a data struct!")
    };
    let members = data_struct.fields.members();

    let gen = quote! {
        impl Components {
            pub(crate) fn insert_from_yaml(entity: Entity, component: &str, data: &serde_yaml::Value, world: &mut World)  {
                match component {
                    #(stringify!(#members) =>
                        world.0.components.#members.insert(
                            entity,
                            serde_yaml::from_value(data.clone()).expect("Can't deserialize component data!")
                        ),
                    )*
                    _ => ()
                };
            }
        }
    };
    gen.into()
}
