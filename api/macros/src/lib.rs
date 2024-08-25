use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse_macro_input;
use syn::spanned::Spanned;

/// Automatically generates the boilerplate necessary to register the given
/// function as an AWS Lambda entrypoint (handler), and specifically makes it
/// easier to develop API endpoint (route) handlers, by:
///
/// - Automatically treating arguments as path parameters, and as such parsing
///   them from the HTTP request which triggerred the Lambda invocation.
// TODO: more...
///
/// Inspired by the Rocket web framework's code-generation capabilities to lead
/// to a nicer DX.
#[proc_macro_attribute]
pub fn route_handler(_input: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as syn::ItemFn);

    let function_name = &function.sig.ident;
    let function_body = &function.block;
    let request_argument = quote!(request);

    let mut parse_path_parameters = function.sig.inputs
        .iter()
        .map(|input| {
            let syn::FnArg::Typed(input) = input else {
                return quote_spanned! { input.span() => compile_error!("route handler cannot take a `self` argument"); };
            };
            let name = match input.pat.as_ref() {
                syn::Pat::Ident(syn::PatIdent { ident, .. }) => ident,
                other => unimplemented!("route handler arguments of type {other:?}"),
            };
            let type_ = &input.ty;

            quote_spanned!(input.span() =>
                let parameter = path_parameters
                    .first(stringify!(#name))
                    .ok_or(format!("route handler expects a {:?} path parameter, but the request had none", stringify!(#name)))?;
                let #name = urlencoding::decode(parameter)
                    .map_err(|err| format!("path parameter {:?} is not valid UTF-8: {err}", stringify!(#name)))?
                    .to_string();
                let #input = #name.parse()
                    // TODO: This should probably be a 4xx instead of just a 500
                    .map_err(|err| format!("path parameter {:?} could not be parsed to the type {:?}: {err}", stringify!(#name), stringify!(#type_)))?;
            )
        })
        .collect::<proc_macro2::TokenStream>();
    if !parse_path_parameters.is_empty() {
        parse_path_parameters = quote! {
            let path_parameters = lambda_http::RequestExt::path_parameters(&#request_argument);
            #parse_path_parameters
        };
    }

    let return_type_declaration = match &function.sig.output {
        syn::ReturnType::Default => quote!(),
        syn::ReturnType::Type(_, type_) => quote!(: #type_),
    };

    TokenStream::from(quote!(
        async fn #function_name(
            #request_argument: lambda_http::Request,
        ) -> Result<impl lambda_http::IntoResponse, lambda_http::Error> {
            #parse_path_parameters

            let ret #return_type_declaration = #function_body;

            Ok(ret)
        }

        #[tokio::main]
        async fn main() -> Result<(), lambda_http::Error> {
            lambda_http::run(lambda_http::service_fn(#function_name)).await
        }
    ))
}
