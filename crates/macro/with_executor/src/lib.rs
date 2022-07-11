extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{fold::Fold, parse_macro_input, parse_quote, Expr, ItemFn, Path};

struct Args {}

impl Args {
    fn new() -> Self {
        Args {}
    }
}

fn path_to_string(path: &Path) -> String {
    let parts: Vec<String> = path
        .segments
        .pairs()
        .map(|pair| pair.value().ident.to_string())
        .collect();
    parts.join("::")
}

impl Fold for Args {
    fn fold_expr_call(&mut self, i: syn::ExprCall) -> syn::ExprCall {
        let mut out = i.clone();
        let index_to_replace = i
            .args
            .into_iter()
            .enumerate()
            .find(|(_, value)| match value {
                Expr::Path(path) => path_to_string(&path.path) == *"executor",
                _ => false,
            });
        if let Some((index, _)) = index_to_replace {
            out.args[index] = parse_quote!(unsafe { executor.get().read() })
        }
        out
    }
}

#[proc_macro_attribute]
pub fn with_executor(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let mut args = Args::new();

    let output = args.fold_item_fn(input);
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = output;
    sig.generics.params.push(parse_quote!(EXECUTOR));
    sig.inputs.insert(0, parse_quote!(executor: EXECUTOR));

    let stmts = &block.stmts;
    TokenStream::from(quote! {
        #(#attrs)* #vis #sig {
            let executor = std::cell::UnsafeCell::new(executor);
            #(#stmts)*
        }
    })
}
