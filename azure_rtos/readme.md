## how to build


### build azure_rtos for cortex a9
- install [prerequisites](./threadx/README.md#Prerequisites)
- copy threadx_build/cortex_a9.cmake to /threadx/cmake/
- copy threadx_build/CMakeLists.txt to  /threadx/ports/cortex_a9/gnu/
- copy threadx/ports/cortex_a9/gnu/inc/tx_port.h tp threadx/common/
- in the /threadx/ build as [static library](./threadx/README.md#Building-as-a-static-library) (On Windows use `Developper Command prompt for VS` to execute these ) :
    ```cmd
    cmake -Bbuild -DCMAKE_TOOLCHAIN_FILE=cmake/cortex_a9.cmake -GNinja .

    cmake --build ./build
    ```
- in threadx/build, rename libthreadx.a to libthreadx.lib
- run cargo build



### some docs
https://docs.rust-embedded.org/book/c-tips/index.html