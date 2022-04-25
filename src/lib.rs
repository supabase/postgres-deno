// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.
//!  This example shows you how to define ops in Rust and then call them from
//!  JavaScript.
// https://github.com/denoland/deno/blob/main/core/examples/hello_world.rs
// https://github.com/denoland/deno/blob/main/core/examples/http_bench_json_ops.rs

use deno_core::v8;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use pgx::*;

pg_module_magic!();

// https://github.com/denoland/deno/blob/main/core/examples/eval_js_value.rs
fn eval(context: &mut JsRuntime, code: &str) -> Result<serde_json::Value, String> {
    let res = context.execute_script("<anon>", code);
    match res {
        Ok(global) => {
            let scope = &mut context.handle_scope();
            let local = v8::Local::new(scope, global);
            // Deserialize a `v8` object into a Rust type using `serde_v8`,
            // in this case deserialize to a JSON `Value`.
            let deserialized_value = serde_v8::from_v8::<serde_json::Value>(scope, local);

            match deserialized_value {
                Ok(value) => Ok(value),
                Err(err) => Err(format!("Cannot deserialize value: {:?}", err)),
            }
        }
        Err(err) => Err(format!("Evaling error: {:?}", err)),
    }
}

#[pg_extern]
fn pldeno_execute(code: &str) -> pgx::JsonB {
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    // Evaluate some code
    let output: serde_json::Value = eval(&mut runtime, code).expect("Eval failed");

    pgx::JsonB(output)
}

// #[pg_extern]
// fn hello_pldeno() {
// // fn hello_pldeno() ->&'static str {

//     let mut js_runtime = create_js_runtime();
//     let runtime = tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();
//     let future = async move {
//         js_runtime
//             .execute_script(
//                 "<usage>",
//                 r#"
// // Print helper function, calling Deno.core.print()
// function main() {
//   return { "hello": "value" };
// }
// main();
//                 "#,
//             )
//             .unwrap();
//         js_runtime.run_event_loop(false).await
//     };
//     let res = runtime.block_on(future).unwrap();
//     return res
//     // return "hello_pldeno";

//     //   runtime
//     //     .execute_script(
//     //       "<usage>",
//     //       r#"
//     // function print(value) {
//     //   Deno.core.print(value.toString()+"\n");
//     // }
//     // // Print helper function, calling Deno.core.print()
//     // const arr = [1, 2, 3];
//     // print("The sum of");
//     // "#,
//     //     )
//     //     .unwrap();
//     // return "hello_pldeno";
// }

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    // use pgx::*;

    // #[pg_test]
    // fn test_hello_pldeno() {
    //     assert_eq!("Hello, pldeno", crate::hello_pldeno());
    // }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
