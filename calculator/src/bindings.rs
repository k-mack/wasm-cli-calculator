// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod docs {
  pub mod calculator {
    
    #[allow(clippy::all)]
    pub mod add {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      #[allow(unused_unsafe, clippy::all)]
      /// Returns the sum of two integers as an integer.
      pub fn add(a: u32,b: u32,) -> u32{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        unsafe {
          
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "docs:calculator/add@0.1.0")]
          extern "C" {
            #[link_name = "add"]
            fn wit_import(_: i32, _: i32, ) -> i32;
          }
          
          #[cfg(not(target_arch = "wasm32"))]
          fn wit_import(_: i32, _: i32, ) -> i32{ unreachable!() }
          let ret = wit_import(wit_bindgen::rt::as_i32(a), wit_bindgen::rt::as_i32(b));
          ret as u32
        }
      }
      
    }
    
  }
}
pub mod exports {
  pub mod docs {
    pub mod calculator {
      
      #[allow(clippy::all)]
      pub mod calculate {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        /// Operations supported by the calculator.
        #[repr(u8)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub enum Op {
          Add,
        }
        impl ::core::fmt::Debug for Op {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Op::Add => {
                f.debug_tuple("Op::Add").finish()
              }
            }
          }
        }
        
        impl Op{
          pub(crate) unsafe fn _lift(val: u8) -> Op{
            if !cfg!(debug_assertions) {
              return ::core::mem::transmute(val);
            }
            
            match val {
              0 => Op::Add,
              
              _ => panic!("invalid enum discriminant"),
            }
          }
        }
        
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "docs:calculator/calculate@0.1.0#eval-expression"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_eval_expression(arg0: i32,arg1: i32,arg2: i32,) -> i32 {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let result0 = <_GuestImpl as Guest>::eval_expression(Op::_lift(arg0 as u8), arg1 as u32, arg2 as u32);
            wit_bindgen::rt::as_i32(result0)
          }
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          /// Evaluates an expression that applies an operation with two integers.
          fn eval_expression(op: Op,x: u32,y: u32,) -> u32;
        }
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:calculator"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1548] = [3, 0, 10, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 0, 97, 115, 109, 13, 0, 1, 0, 7, 92, 1, 65, 2, 1, 66, 4, 1, 109, 1, 3, 97, 100, 100, 4, 0, 2, 111, 112, 3, 0, 0, 1, 64, 3, 2, 111, 112, 1, 1, 120, 121, 1, 121, 121, 0, 121, 4, 0, 15, 101, 118, 97, 108, 45, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 1, 2, 4, 1, 31, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 99, 97, 108, 99, 117, 108, 97, 116, 101, 64, 48, 46, 49, 46, 48, 5, 0, 11, 15, 1, 0, 9, 99, 97, 108, 99, 117, 108, 97, 116, 101, 3, 0, 0, 7, 55, 1, 65, 2, 1, 66, 2, 1, 64, 2, 1, 97, 121, 1, 98, 121, 0, 121, 4, 0, 3, 97, 100, 100, 1, 0, 4, 1, 25, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 97, 100, 100, 64, 48, 46, 49, 46, 48, 5, 0, 11, 9, 1, 0, 3, 97, 100, 100, 3, 2, 0, 7, 90, 1, 65, 2, 1, 65, 2, 1, 66, 2, 1, 64, 2, 1, 97, 121, 1, 98, 121, 0, 121, 4, 0, 3, 97, 100, 100, 1, 0, 4, 1, 25, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 97, 100, 100, 64, 48, 46, 49, 46, 48, 5, 0, 4, 1, 27, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 97, 100, 100, 101, 114, 64, 48, 46, 49, 46, 48, 4, 0, 11, 11, 1, 0, 5, 97, 100, 100, 101, 114, 3, 4, 0, 7, 184, 1, 1, 65, 2, 1, 65, 4, 1, 66, 2, 1, 64, 2, 1, 97, 121, 1, 98, 121, 0, 121, 4, 0, 3, 97, 100, 100, 1, 0, 3, 1, 25, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 97, 100, 100, 64, 48, 46, 49, 46, 48, 5, 0, 1, 66, 4, 1, 109, 1, 3, 97, 100, 100, 4, 0, 2, 111, 112, 3, 0, 0, 1, 64, 3, 2, 111, 112, 1, 1, 120, 121, 1, 121, 121, 0, 121, 4, 0, 15, 101, 118, 97, 108, 45, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 1, 2, 4, 1, 31, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 99, 97, 108, 99, 117, 108, 97, 116, 101, 64, 48, 46, 49, 46, 48, 5, 1, 4, 1, 32, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 64, 48, 46, 49, 46, 48, 4, 0, 11, 16, 1, 0, 10, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 3, 6, 0, 7, 125, 1, 65, 2, 1, 65, 2, 1, 66, 4, 1, 109, 1, 3, 97, 100, 100, 4, 0, 2, 111, 112, 3, 0, 0, 1, 64, 3, 2, 111, 112, 1, 1, 120, 121, 1, 121, 121, 0, 121, 4, 0, 15, 101, 118, 97, 108, 45, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 1, 2, 3, 1, 31, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 99, 97, 108, 99, 117, 108, 97, 116, 101, 64, 48, 46, 49, 46, 48, 5, 0, 4, 1, 25, 100, 111, 99, 115, 58, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 47, 97, 112, 112, 64, 48, 46, 49, 46, 48, 4, 0, 11, 9, 1, 0, 3, 97, 112, 112, 3, 8, 0, 0, 185, 6, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 34, 119, 111, 114, 108, 100, 115, 34, 58, 123, 34, 97, 100, 100, 101, 114, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 68, 101, 102, 105, 110, 101, 115, 32, 116, 104, 101, 32, 99, 111, 109, 112, 111, 110, 101, 110, 116, 32, 116, 104, 97, 116, 32, 112, 114, 111, 118, 105, 100, 101, 115, 32, 116, 104, 101, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 46, 34, 125, 44, 34, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 68, 101, 102, 105, 110, 101, 115, 32, 116, 104, 101, 32, 99, 111, 109, 112, 111, 110, 101, 110, 116, 32, 116, 104, 97, 116, 32, 100, 101, 112, 101, 110, 100, 115, 32, 111, 110, 32, 97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 112, 97, 114, 116, 121, 32, 40, 99, 111, 109, 112, 111, 110, 101, 110, 116, 32, 111, 114, 32, 87, 97, 115, 109, 92, 110, 104, 111, 115, 116, 41, 32, 116, 111, 32, 112, 114, 111, 118, 105, 100, 101, 32, 116, 104, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 101, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 112, 114, 111, 118, 105, 100, 101, 115, 32, 97, 92, 110, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 46, 34, 125, 44, 34, 97, 112, 112, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 68, 101, 102, 105, 110, 101, 115, 32, 116, 104, 101, 32, 99, 111, 109, 112, 111, 110, 101, 110, 116, 32, 116, 104, 97, 116, 32, 100, 101, 112, 101, 110, 100, 115, 32, 111, 110, 32, 97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 112, 97, 114, 116, 121, 32, 40, 99, 111, 109, 112, 111, 110, 101, 110, 116, 32, 111, 114, 32, 87, 97, 115, 109, 92, 110, 104, 111, 115, 116, 41, 32, 116, 111, 32, 112, 114, 111, 118, 105, 100, 101, 32, 116, 104, 101, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110, 32, 111, 102, 32, 97, 32, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 46, 34, 125, 125, 44, 34, 105, 110, 116, 101, 114, 102, 97, 99, 101, 115, 34, 58, 123, 34, 99, 97, 108, 99, 117, 108, 97, 116, 101, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 68, 101, 115, 99, 114, 105, 98, 101, 115, 32, 116, 104, 101, 32, 99, 111, 110, 116, 114, 97, 99, 116, 32, 111, 102, 32, 97, 32, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 46, 34, 44, 34, 102, 117, 110, 99, 115, 34, 58, 123, 34, 101, 118, 97, 108, 45, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 34, 58, 34, 69, 118, 97, 108, 117, 97, 116, 101, 115, 32, 97, 110, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 116, 104, 97, 116, 32, 97, 112, 112, 108, 105, 101, 115, 32, 97, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 119, 105, 116, 104, 32, 116, 119, 111, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 34, 125, 44, 34, 116, 121, 112, 101, 115, 34, 58, 123, 34, 111, 112, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 79, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 98, 121, 32, 116, 104, 101, 32, 99, 97, 108, 99, 117, 108, 97, 116, 111, 114, 46, 34, 125, 125, 125, 44, 34, 97, 100, 100, 34, 58, 123, 34, 100, 111, 99, 115, 34, 58, 34, 68, 101, 115, 99, 114, 105, 98, 101, 115, 32, 116, 104, 101, 32, 99, 111, 110, 116, 114, 97, 99, 116, 32, 116, 111, 32, 97, 100, 100, 32, 116, 119, 111, 32, 105, 110, 116, 101, 103, 101, 114, 115, 46, 34, 44, 34, 102, 117, 110, 99, 115, 34, 58, 123, 34, 97, 100, 100, 34, 58, 34, 82, 101, 116, 117, 114, 110, 115, 32, 116, 104, 101, 32, 115, 117, 109, 32, 111, 102, 32, 116, 119, 111, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 97, 115, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114, 46, 34, 125, 125, 125, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
