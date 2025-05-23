use proc_macro2::TokenStream;
use quote::quote;

pub fn derive_module(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
}

pub fn derive_interface(input: TokenStream) -> TokenStream {
    todo!()
}

// struct ModuleAttr<'a> {
//     module: &'a Ident,
//     route: Option<TokenStream>,
//     handlers: Vec<PhasedHandlerMethod>,
// }

// // TODO: Add db helper and for the method handler
// pub fn derive(input: TokenStream) -> TokenStream {
//     let ast: DeriveInput = parse2(input).unwrap();
//     let module = &ast.ident;

//     let mut module_attr = ModuleAttr {
//         module,
//         route: None,
//         handlers: Vec::new(),
//     };

//     for attr in ast.attrs.iter() {
//         if attr.path().is_ident("route") {
//             module_attr.route = Some(parse_route(attr));
//             continue;
//         }

//         if let Some(ident) = attr.path().get_ident() {
//             if ["get", "post", "put", "delete"].contains(&ident.to_string().as_str()) {
//                 module_attr
//                     .handlers
//                     .push(parse_handler(&module, attr, &ident));
//             }
//         }
//     }

//     get_parsed_tokens(module_attr)
// }

// // Route
// fn parse_route(attr: &Attribute) -> TokenStream {
//     let Ok(lit) = attr.parse_args::<LitStr>() else {
//         panic!("route must be string")
//     };
//     quote!(#lit)
// }

// struct PhasedHandlerMethod {
//     add_route: TokenStream,
// }

// // MethodHandler
// fn parse_handler(module: &Ident, attr: &Attribute, method: &Ident) -> PhasedHandlerMethod {
//     let punctuated = attr
//         .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
//         .unwrap();

//     let mut punctuated_iter = punctuated.into_iter();
//     let Some(route_expr) = punctuated_iter.next() else {
//         panic!("route is missing")
//     };
//     let Some(handler_expr) = punctuated_iter.next() else {
//         panic!("handler is missing")
//     };

//     check_route(&route_expr);
//     check_handler(&handler_expr);
//     let add_route = quote! {
//         .route(#route_expr, lan_frame::axum::routing::#method(#module::#handler_expr))
//     };
//     PhasedHandlerMethod { add_route }
// }

// fn check_route(route_expr: &Expr) {
//     let Expr::Lit(ExprLit {
//         lit: Lit::Str(_), ..
//     }) = route_expr
//     else {
//         panic!("route must be string literal")
//     };
// }

// fn check_handler(handler_expr: &Expr) {
//     let Expr::Path(expr_path) = handler_expr else {
//         panic!("handler must be literal")
//     };
//     if expr_path.path.segments.len() != 1 {
//         panic!("handler must be literal")
//     }
// }

// fn get_parsed_tokens(module_attr: ModuleAttr) -> TokenStream {
//     let module = module_attr.module;
//     let name = &module.to_string();
//     let route = module_attr.route.unwrap_or(quote! {"/"});
//     let add_routes = module_attr
//         .handlers
//         .iter()
//         .fold(TokenStream::new(), |acc, handler| {
//             let add_route = &handler.add_route;
//             quote! {#acc #add_route}
//         });

//     quote! {
//         impl Module for #module {
//             fn name(&self) -> &str{
//                 #name
//             }

//             fn route(&self) -> &str {
//                 #route
//             }

//             fn router(&self) -> lan_frame::axum::Router {
//                 lan_frame::axum::Router::new()#add_routes
//             }
//         }
//     }
// }
