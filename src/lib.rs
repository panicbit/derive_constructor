use proc_macro2::{Ident, TokenStream};
use quote::quote;
use synstructure::{Structure, decl_derive};
use syn::{ConstParam, GenericParam, LifetimeDef, TypeParam, parse_str};

fn constructor_derive(s: Structure) -> TokenStream {
    let self_ident = &s.ast().ident;
    let generics_decl = &s.ast().generics;
    let generics = s.ast().generics.params.iter().map(|g| match g {
            GenericParam::Type(TypeParam { ident, .. }) => quote! { #ident },
            GenericParam::Lifetime(LifetimeDef { lifetime, .. }) => quote! { #lifetime },
            GenericParam::Const(ConstParam { ident, .. }) => quote! { #ident },
    });
    let generics = quote! { <#(#generics),*> };
    let where_clause = &s.ast().generics.where_clause;
    let trait_ident = format!("{}Constructor", self_ident);
    let trait_ident = parse_str::<Ident>(&trait_ident).unwrap();

    let trait_decl_items = s.variants().iter().map(|v| {
        let ident = &v.ast().ident;
        
        if v.bindings().is_empty() {
            return quote! {
                const #ident: Self;
            }
        }

        let arg_decls = v.bindings().iter().map(|b| {
            let ident = b.ast().ident.as_ref().unwrap_or(&b.binding);
            let ty = &b.ast().ty;

            quote! {
                #ident: #ty
            }
        });

        quote! {
            #[allow(non_snake_case)]
            fn #ident(#(#arg_decls),*) -> Self;
        }
    });
    let trait_decl = quote! {
        pub trait #trait_ident #generics_decl #where_clause {
            #(#trait_decl_items)*
        }
    };

    let trait_impl_items = s.variants().iter().map(|v| {
        let ident = &v.ast().ident;

        if v.bindings().is_empty() {
            return quote! {
                const #ident: Self = Self::#ident;
            }
        }

        let arg_decls = v.bindings().iter().map(|b| {
            let ident = b.ast().ident.as_ref().unwrap_or(&b.binding);
            let ty = &b.ast().ty;

            quote! {
                #ident: #ty
            }
        });

        let construct = v.construct(|_, i| {
            let b = &v.bindings()[i];
            let ident = b.ast().ident.as_ref().unwrap_or(&b.binding);

            ident
        });

        quote! {
            fn #ident(#(#arg_decls),*) -> Self {
                #construct
            }
        }
    });
    let trait_impl = quote! {
        impl #generics_decl #trait_ident #generics for #self_ident #generics #where_clause {
            #(#trait_impl_items)*
        }
    };

    quote! {
        #trait_decl
        #trait_impl
    }
}

decl_derive!([Constructor] => constructor_derive);

#[cfg(test)]
mod tests {
    #![allow(clippy::blacklisted_name, clippy::redundant_field_names)]
    use super::*;
    use synstructure::test_derive;

    #[test]
    fn test() {
        test_derive! {
            constructor_derive {
                enum Foo<'a, T: Clone> where T: Copy {
                    A,
                    B,
                    X(i32, &'a str, T),
                    Y { foo: i32, bar: &'a str },
                }
            }

            expands to {
                pub trait FooConstructor<'a, T: Clone> where T: Copy {
                    const A: Self;
                    const B: Self;

                    #[allow(non_snake_case)]
                    fn X(__binding_0: i32, __binding_1: &'a str, __binding_2: T) -> Self;
                    #[allow(non_snake_case)]
                    fn Y(foo: i32, bar: &'a str) -> Self;
                }

                impl<'a, T: Clone> FooConstructor<'a, T> for Foo<'a, T> where T: Copy {
                    const A: Self = Self::A;
                    const B: Self = Self::B;

                    fn X(__binding_0: i32, __binding_1: &'a str, __binding_2: T) -> Self {
                        Foo::X(__binding_0, __binding_1, __binding_2,)
                    }

                    fn Y(foo: i32, bar: &'a str) -> Self {
                        Foo::Y {
                            foo: foo,
                            bar: bar,
                        }
                    }
                }
            }
        }
    }
}
