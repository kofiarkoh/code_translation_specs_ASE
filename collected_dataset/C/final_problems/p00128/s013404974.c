#include <stdio.h>
#include <string.h>

#define put(x, y, z) (abacus[(x)][(y)] = z)

char abacus[8][6];

int main(){
  int n, i, j, a, f = 0;
  while(scanf("%d", &n) && n){
    for(i = 0;i < 8;i++) for(j = 0;j < 5;j++) put(i, j, '*');
    for(i = 0;i < 5;i++) put(2, i, '=');
    for(i = 0;i < 5; i++){
      a = n % 10;
      n /= 10;
      if(a < 5) put(1, 4 - i, ' ');
      else if(a >= 5){
	put(0, 4 - i, ' ');
	a -= 5;
      }

      put(3 + a, 4 - i, ' ');
    }
    if(f) puts("");
    for(i = 0; i < 8;i++) puts(abacus[i]);
    f = 1;
  }
  return 0;
}