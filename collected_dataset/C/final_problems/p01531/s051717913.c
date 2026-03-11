#include <stdio.h>

int main() {
  int n, cn;
  char c;
  char x[10] = {'w', 0, 'k', 's', 't', 'n', 'h', 'm', 'y', 'r'};
  char f[5] = {'a', 'i', 'u', 'e', 'o'};

  while(scanf("%d %c", &n, &c) != EOF) {
    if(c == 'T') cn = 0;
    else if(c == 'L') cn = 1;
    else if(c == 'U') cn = 2;
    else if(c == 'R') cn = 3;
    else cn = 4;

    if(n == 0 && cn == 2) printf("nn");
    else if(x[n] == 0) printf("%c", f[cn]);
    else printf("%c%c", x[n], f[cn]);
  }

  printf("\n");

  return 0;
}