use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(stream: TokenStream) -> TokenStream {
    let Some(struct_name) = get_struct_name(stream) else {
        panic!("The name of the struct is not found.");
    };

    format!("impl Component for {struct_name} {{ }}").parse().unwrap()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(stream: TokenStream) -> TokenStream {
    let Some(struct_name) = get_struct_name(stream) else {
        panic!("The name of the struct is not found.");
    };

    format!("impl Resource for {struct_name} {{ }}").parse().unwrap()
}

#[proc_macro_derive(SystemResource)]
pub fn derive_system_resource(stream: TokenStream) -> TokenStream {
    let Some(struct_name) = get_struct_name(stream) else {
        panic!("The name of the struct is not found.");
    };

    format!("impl SystemResource for {struct_name} {{ }}").parse().unwrap()
}

fn get_struct_name(stream: TokenStream) -> Option<String> {
    let mut iter = stream.into_iter();

    while let Some(tree) = iter.next() {
        if let proc_macro::TokenTree::Ident(ident) = tree {
            if ident.to_string() == "struct" {
                break;
            }
        }
    }

    let Some(name_tree) = iter.next() else {
        return None;
    };

    let proc_macro::TokenTree::Ident(name_ident) = name_tree else {
        return None;
    };

    Some(name_ident.to_string())
}
