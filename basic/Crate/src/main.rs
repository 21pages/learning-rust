fn main() {
    test_lib::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    test_lib::indirect_access();
}

//rustc main.rs --extern test_lib=test\libtest_lib.rlib --edition=2018
