#include "pch.h"
#include <stdio.h>

//#include "hello.h"
extern "C" void hello(const char *pFrom);
extern "C" void callback(void (*pCallback)(const char *pFrom));
extern "C" void cpp_hello(const char *pFrom);

int main(int argc, char **argv) {
  const char *from_context = "C++";
  hello(from_context);
  callback(cpp_hello);

  printf("\nPress [RETURN] to continue.\n");
  char buffer[80];
  fgets(buffer, sizeof(buffer), stdin);
  return 0;
}

extern "C" {
  void cpp_hello(const char *pFrom) {
    printf("Hello from %s to C++!\n", pFrom);
  }
}
