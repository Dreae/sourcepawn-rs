// Line comment
/*
 * This is a block comment
 */
public void SayHello() {
       PrintToServer("Hello, world!");
       if (foo == 'c' || foo == '\n') {
          return;
       } else {
          int foo = 123;
          PrintToServer("Foo: %d", foo);
       }
}
