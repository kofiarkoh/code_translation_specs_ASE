#include <stdio.h>

int main(void){
  int N;
  scanf("%d",&N);
  long a[N];

  for(int i = 0;i < N;i++) scanf("%ld",&a[i]);

  int i = N-1;
  for(;i >= 0;i -= 2) printf("%ld ",a[i]);
  if(N%2 != 0) i += 3;
  else i++;
  for(;i < N;i += 2){
    if(i+2 >= N) printf("%ld\n",a[i]);
    else printf("%ld ",a[i]);
  }

  return 0;
}