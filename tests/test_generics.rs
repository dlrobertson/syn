extern crate syn;
use syn::*;

#[macro_use]
extern crate quote;

#[test]
fn test_split_for_impl() {
    // <'a, 'b: 'a, #[may_dangle] T: 'a = ()> where T: Debug
    let generics = Generics {
        lifetimes: vec![LifetimeDef {
                            attrs: Vec::new(),
                            lifetime: Lifetime::new("'a"),
                            bounds: Vec::new(),
                        },
                        LifetimeDef {
                            attrs: Vec::new(),
                            lifetime: Lifetime::new("'b"),
                            bounds: vec![Lifetime::new("'a")],
                        }],
        ty_params: vec![TyParam {
                            attrs: vec![Attribute {
                                            style: AttrStyle::Outer,
                                            path: "may_dangle".into(),
                                            tts: vec![],
                                            is_sugared_doc: false,
                                        }],
                            ident: Ident::new("T"),
                            bounds: vec![TyParamBound::Region(Lifetime::new("'a"))],
                            default: Some(TyTup { tys: Vec::new() }.into()),
                        }],
        where_clause: WhereClause {
            predicates: vec![WherePredicate::BoundPredicate(WhereBoundPredicate {
                                                                bound_lifetimes: Vec::new(),
                                                                bounded_ty: TyPath {
                                                                    qself: None,
                                                                    path: "T".into(),
                                                                }.into(),
                                                                bounds: vec![
                        TyParamBound::Trait(
                            PolyTraitRef {
                                bound_lifetimes: Vec::new(),
                                trait_ref: "Debug".into(),
                            },
                            TraitBoundModifier::None,
                        ),
                    ],
                                                            })],
        },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let tokens = quote! {
        impl #impl_generics MyTrait for Test #ty_generics #where_clause {}
    };
    let expected = concat!("impl < 'a , 'b : 'a , # [ may_dangle ] T : 'a > ",
                           "MyTrait for Test < 'a , 'b , T > ",
                           "where T : Debug { }");
    assert_eq!(expected, tokens.to_string());

    let turbofish = ty_generics.as_turbofish();
    let tokens = quote! {
        Test #turbofish
    };
    let expected = "Test :: < 'a , 'b , T >";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_ty_param_bound() {
    let tokens = quote!('a);
    let expected = TyParamBound::Region(Lifetime::new("'a"));
    assert_eq!(expected, parse_ty_param_bound(tokens.as_str()).unwrap());

    let tokens = quote!(Debug);
    let expected = TyParamBound::Trait(PolyTraitRef {
                                           bound_lifetimes: Vec::new(),
                                           trait_ref: "Debug".into(),
                                       },
                                       TraitBoundModifier::None);
    assert_eq!(expected, parse_ty_param_bound(tokens.as_str()).unwrap());

    let tokens = quote!(?Sized);
    let expected = TyParamBound::Trait(PolyTraitRef {
                                           bound_lifetimes: Vec::new(),
                                           trait_ref: "Sized".into(),
                                       },
                                       TraitBoundModifier::Maybe);
    assert_eq!(expected, parse_ty_param_bound(tokens.as_str()).unwrap());
}
