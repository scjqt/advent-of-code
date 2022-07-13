use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Expr, ExprLit, ExprRange, Lit, RangeLimits, Result,
};

struct Values {
    values: Vec<u8>,
}

impl Parse for Values {
    fn parse(input: ParseStream) -> Result<Self> {
        let exprs: Punctuated<Expr, Comma> = input.parse_terminated(Expr::parse)?;
        let mut values: Vec<u8> = Vec::new();

        for expr in exprs {
            match expr {
                Expr::Lit(_) => {
                    values.push(extract_int(expr)?);
                }
                Expr::Range(ExprRange {
                    from, limits, to, ..
                }) => {
                    let start = from.map_or(Ok(1), |e| extract_int(*e))?;
                    let mut end = to.map_or(Ok(26), |e| extract_int(*e))?;
                    if let RangeLimits::Closed(_) = limits {
                        end += 1;
                    }
                    for day in start..end {
                        values.push(day);
                    }
                }
                _ => panic!(),
            }
        }
        Ok(Values { values })
    }
}

fn extract_int(expr: Expr) -> Result<u8> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Int(day), ..
    }) = expr
    {
        return day.base10_parse();
    }
    panic!()
}

fn day_ident(day: u8) -> Ident {
    if day < 10 {
        format_ident!("day0{}", day)
    } else {
        format_ident!("day{}", day)
    }
}

#[proc_macro]
pub fn part1(item: TokenStream) -> TokenStream {
    let Values { values: days } = parse_macro_input!(item as Values);
    expand(days.into_iter().map(|day| (day, 1)).collect())
}

#[proc_macro]
pub fn complete(item: TokenStream) -> TokenStream {
    let Values { values: days } = parse_macro_input!(item as Values);

    let mut parts = Vec::new();
    for day in days {
        parts.push((day, 1));
        if day != 25 {
            parts.push((day, 2));
        }
    }

    expand(parts)
}

fn expand(parts: Vec<(u8, u8)>) -> TokenStream {
    let parts = parts.into_iter().map(|(day, part)| {
        let day_ident = day_ident(day);
        let part_ident = format_ident!("part{}", part);
        quote! {
            if (day, part) == (#day, #part) {
                return Some(#day_ident::#part_ident(input).to_string());
            }
        }
    });

    quote! {
        #(#parts)*
    }
    .into()
}

#[proc_macro]
pub fn days(item: TokenStream) -> TokenStream {
    let Values { values: days } = parse_macro_input!(item as Values);
    let days = days.into_iter().map(|day| {
        let ident = day_ident(day);
        quote! {
            mod #ident;
        }
    });
    quote! {
        #(#days)*
    }
    .into()
}

#[proc_macro]
pub fn years(item: TokenStream) -> TokenStream {
    let Values { values: years } = parse_macro_input!(item as Values);
    let years = years.into_iter().map(|year| {
        let ident = format_ident!("year20{}", year);
        quote! {
            if year == #year {
                return #ident::run(day, part, input);
            }
        }
    });
    quote! {
        #(#years)*
    }
    .into()
}
