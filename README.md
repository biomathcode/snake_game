There is a glue code between webassembly and javascript
allocation, rust is using the global and to reduce the size we will be using wee_alloc which will create a smaller allocation and smaller webassembly size

Things not to do for invalid pointers
Pointer will change if you set new data 