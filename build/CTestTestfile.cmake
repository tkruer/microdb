# CMake generated Testfile for 
# Source directory: /Users/tylerkruer/code/microdb
# Build directory: /Users/tylerkruer/code/microdb/build
# 
# This file includes the relevant testing commands required for 
# testing this directory and lists subdirectories to be tested as well.
add_test(dbTests "/Users/tylerkruer/code/microdb/build/tests")
set_tests_properties(dbTests PROPERTIES  _BACKTRACE_TRIPLES "/Users/tylerkruer/code/microdb/CMakeLists.txt;22;add_test;/Users/tylerkruer/code/microdb/CMakeLists.txt;0;")
subdirs("_deps/catch2-build")
