use anyhow::{anyhow, Result};
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (d, p) = parse_attr(attr).unwrap();
    let mut func = parse_macro_input!(item as ItemFn);
    func.vis = parse_quote!(pub);
    let gen = if p == 1 {
        func.sig.ident = parse_quote!(solve_one);
        quote! {

            mod inner_one {
                use crate::*;
                use crate::inner_two::*;
                use anyhow::Result;

                #func
            }
            fn main() -> anyhow::Result<()> {
                use utils::Parser;
                let args = utils::Args::parse();

                let input = args.get_input(#d)?;

                if args.run_one() {
                    let one = inner_one::solve_one(&input)?;
                    println!("part one:\n{}", one);
                    args.submit_one(#d, one);
                }
                if args.run_two() {
                    let two = inner_two::solve_two(&input)?;
                    println!("part two:\n{}", two);
                    args.submit_two(#d, two);
                }

                Ok(())
            }
        }
    } else {
        func.sig.ident = parse_quote!(solve_two);
        quote! {
            mod inner_two {
                use crate::*;
                use crate::inner_one::*;
                use anyhow::Result;

                #func
            }
        }
    };
    gen.into()
}

fn parse_attr(attr: TokenStream) -> Result<(i32, i32)> {
    let mut i = attr.into_iter().filter_map(|t| {
        if let TokenTree::Ident(_) = t {
            Some(t.to_string())
        } else {
            None
        }
    });
    let d = i
        .next()
        .ok_or(anyhow!("expecting 'dayX' first attribute"))
        .and_then(|d| {
            d.strip_prefix("day")
                .ok_or(anyhow!("expecting 'dayX' attribute first, got {}", d))?
                .parse::<i32>()
                .map_err(|e| {
                    anyhow!(
                        "expecting 'dayX', could not parse day number in {} - {}",
                        d,
                        e
                    )
                })
        })?;
    let p = i
        .next()
        .ok_or(anyhow!("expecting 'part<1|2>' second attribute"))
        .and_then(|p| {
            p.strip_prefix("part")
                .ok_or(anyhow!("expecting 'part<1|2>' second attribute, got {}", d))?
                .parse::<i32>()
                .map_err(|e| {
                    anyhow!(
                        "expecting 'part<1|2>', could not parse part number in {} - {}",
                        d,
                        e
                    )
                })
                .and_then(|num| {
                    if num != 1 && num != 2 {
                        Err(anyhow!("expecting part number 1 or 2, got {}", num))
                    } else {
                        Ok(num)
                    }
                })
        })?;
    if let Some(n) = i.next() {
        Err(anyhow!("unexpected attr - {}", n))
    } else {
        Ok((d, p))
    }
}
