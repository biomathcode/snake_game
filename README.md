## Snake game build with Rust, WASM and Javascript

How does it works?

- We are creating a pkg file that is generated from the rust code inside src/lib.rs 
- Then we are adding the pkg as npm dependency inside the view package.json. Please see the file to get better understanding
- We are call the rust function compiled into wasm inside the `view/index.ts`
- for deployment root file index.js and public folders are used. 
- root public folder and view/public folders are same



