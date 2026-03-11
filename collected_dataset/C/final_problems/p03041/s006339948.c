#include <stdio.h>

int main(){

  char s[50];
  int N,k;
  scanf("%d %d",&N, &k);
  scanf("%s", &s);

  s[k-1] = s[k-1] +32;

    printf("%s\n",s);

  return 0;
}

