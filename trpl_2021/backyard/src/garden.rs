// Pull src/garden/vegetables.rs into scope
pub mod vegetables;


/*

Note when we declare a module, rust will look for it in the following places:

1. Inline within curly brackets that replace the semicoloon following mod garden
2. In the file src/garden.rs
3. In the file src/garden/mod.rs 

In this particular case we are declaring a sub-module, vegetables, within the garden module, 
so rust will look for it in the following places:

1. Inline within curly brackets that replace the semicoloon following mod vegetables
2. In the file src/garden/vegetables.rs
3. In the file src/garden/vegetables/mod.rs 

*/
