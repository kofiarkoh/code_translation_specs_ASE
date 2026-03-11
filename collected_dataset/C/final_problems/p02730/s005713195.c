#include <stdio.h>
#include <string.h>

int main(int argc, char const *argv[]) {
  char a[100];
  scanf("%s",a);
  int b=strlen(a);
  if (b<7) {
    printf("No\n");
    return 0;
  }
  int c=1;
  for (int i = 0; i < b/2; i++) {
    if (a[i]!=a[b-1-i]) {
      c=0;
    }
  }
  if (c==1) {
    printf("Yes");
  }else{
    printf("No");
  }
  return 0;
}
