#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use syn::export::{TokenStream, Span};
use syn::{Attribute, Visibility, Type, Stmt, ReturnType, Block, FnArg, Pat, Ident, Error, Token, braced, parenthesized, Field, ItemEnum, parse_macro_input, Expr, PathSegment};
use syn::parse::{ParseStream, Parse, Parser};
use syn::token::{Token, Mut, Brace};
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;
use std::marker::PhantomData;
use std::future::Future;
use quote::quote;

#[proc_macro_attribute]
pub fn show_streams(attributes: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attributes.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn my_macro(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let t = item.clone();
    let input = item;
    let tokens = input.clone();
    let parser = Punctuated::<PathSegment, Token![::]>::parse_separated_nonempty;
    let _path = parser.parse(tokens);

    // Parse a possibly empty sequence of expressions terminated by commas with
    // an optional trailing punctuation.
    let tokens = input.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let _args = parser.parse(tokens);

    // Parse zero or more outer attributes but not inner attributes.
    let tokens = input.clone();
    let parser = Attribute::parse_outer;
    let _attrs = parser.parse(tokens);
    println!("{:?}, {:?}, {:?}", _path, _args, _attrs);
    // let input = parse_macro_input!(attributes as MyMacroInput);

    t
}

struct MyMacroInput {
    /* ... */
}

// impl Parse for MyMacroInput {
//     fn parse(input: TokenStream) -> Result<Self, Error> {
//
//         Ok(Self)
        // Parse a nonempty sequence of path segments separated by `::` punctuation
        // with no trailing punctuation.

    // }
// }

#[derive(Debug)]
enum Item {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

#[derive(Debug)]
struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: Brace,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            input.parse().map(Item::Struct)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Item::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
        })
    }
}


async fn test(testing: &str) {
    println!("{}", testing)
}

#[derive(Debug)]
struct CommandProperties {
    // other attributes applied to it
    // pub attributes: Vec<Attribute>,
    // function name
    pub name: Ident,
    // arguments the function takes
    //type the function returns
    // pub returntype: Type,
    // the code of the function
    // pub code: Vec<Stmt>,
}

impl Parse for CommandProperties {
    fn parse(input: ParseStream<'_>) -> Result<Self, Error> {
        // let attributes = input.call(Attribute::parse_outer)?;




        let name = input.parse()?;

        // let Parenthesised(arguments) = input.parse::<Parenthesised<FnArg>>()?;

        // let returntype = match input.parse::<ReturnType>()? {
        //     ReturnType::Type(_, t) => (*t).clone(),
        //     ReturnType::Default => {
        //         return Err(input
        //             .error("expected a result type of either `CommandResult` or `CheckResult`"));
        //     }
        // };

        // { ... }
        // let bcont;
        // braced!(bcont in input);
        // let code = bcont.call(Block::parse_within)?;


        Ok(Self {
            // attributes,
            name,
        })
    }
}