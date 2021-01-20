use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn;

#[proc_macro]
pub fn replace_status(input: TokenStream) -> TokenStream {
    // Grab fn name identifier
    let ident = syn::parse_macro_input!(input as syn::Ident);

    // Paste call to internal function
    let add_replace_status_fn = quote::format_ident!("{}_status_internal_add_replace_fn", ident);
    quote!(
        unsafe {
            /*
             * e.g. marth_turnaround_status_internal_replace_fn()
             *
             * This will call a function generated in the "status_script" proc macro,
             * which will call the actual replace_status_func() function in status_hook
             */
            #add_replace_status_fn();
        }
    ).into()
}

/*
 * This is for the attribute above the replacement func
 * Should be: #[status_func(fighter_str, status_kind_luaconst, lua_script_luaconst)]
 *
 * attr: fighter_str, ...
 * item: the function and body attached
 */
#[proc_macro_attribute]
pub fn status_script(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: StatusAttrs = syn::parse_macro_input!(attr as StatusAttrs);
    let mut usr_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let mut output = TokenStream2::new();

    let usr_fn_name = &usr_fn.sig.ident;
    let _fighter = attr.fighter;
    let _status_kind = attr.status_kind;
    let _lua_script = attr.lua_script;

    // These are used to hold onto the l2c_fighter and l2c_agent the trampoline receives
    let _smash_l2c_fighter = quote::format_ident!("{}_status_l2c_fighter", usr_fn_name);
    let _smash_l2c_agent = quote::format_ident!("{}_status_l2c_agent", usr_fn_name);

    // Modified original!() from skyline-rs
    //https://github.com/ultimate-research/skyline-rs/blob/7f1403fc19e1bf91a3cbe729686898278d3b91b8/skyline_macro/src/lib.rs#L84-L109
    let orig_stmt: syn::Stmt = syn::parse_quote! {
        macro_rules! original {
            () => {
                {
                    #[allow(unused_unsafe)]
                    if true {
                        unsafe {
                            skyline_status::call_original(#_fighter, #_status_kind.into(), #_lua_script.into(), #_smash_l2c_fighter, #_smash_l2c_agent);
                        }
                    }
                }
            }
        }
    };
    usr_fn.block.stmts.insert(0, orig_stmt);
    usr_fn.to_tokens(&mut output);

    let add_replace_fn = quote::format_ident!("{}_status_internal_add_replace_fn", usr_fn_name);
    let trampoline = quote::format_ident!("{}_status_internal_trampoline_fn", usr_fn_name);
    quote!(
        /*
         * Here is the function that is called through replace_status!()
         */
        #[allow(non_upper_case_globals)]
        pub unsafe fn #add_replace_fn() {
            skyline_status::replace_status_func(#_fighter, #_status_kind.into(), #_lua_script.into(), #trampoline);
        }
        /*
         * Trampoline to call user func with boma instead of l2c_fighter and l2c_agent
         * It also sets the l2c_fighter and l2c_agent statics for use by original!()
         */
        #[allow(non_upper_case_globals)]
        pub unsafe fn #trampoline(l2c_fighter: *mut smash::lua2cpp::L2CFighterCommon, l2c_agent: u64) -> smash::lib::L2CValue {
            let _boma = smash::app::sv_system::battle_object_module_accessor((*l2c_fighter).lua_state_agent);
            #_smash_l2c_fighter = l2c_fighter;
            #_smash_l2c_agent = l2c_agent;
            #usr_fn_name(_boma)
        }

        #[allow(non_upper_case_globals)]
        static mut #_smash_l2c_fighter: *mut smash::lua2cpp::L2CFighterCommon = 0 as *mut smash::lua2cpp::L2CFighterCommon;
        #[allow(non_upper_case_globals)]
        static mut #_smash_l2c_agent: u64 = 0 as u64;
    ).to_tokens(&mut output);

    output.into()
}

#[derive(Debug)]
struct StatusAttrs {
    pub fighter: syn::LitStr,   // fighter name
    pub status_kind: syn::Path, // status_kind LuaConst
    pub lua_script: syn::Path,  // lua_kind LuaConst
}

impl syn::parse::Parse for StatusAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fighter = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let status_kind = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let lua_script = input.parse()?;
        Ok(StatusAttrs {
            fighter: fighter,
            status_kind: status_kind,
            lua_script: lua_script,
        })
    }
}
/*
    syn::Path = "A path at which a named item is exported (e.g. std::collections::HashMap)"
    syn::LitStr = "A UTF-8 string literal: "foo""
    syn::ItemFn = "A free-standing function: fn process(n: usize) -> Result<()> { ... }" (needs "full" feature)
*/
