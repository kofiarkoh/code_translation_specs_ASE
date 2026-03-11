#include <stdio.h>

void swap(int *a, int *b){
  int t=*a;*a=*b;*b=t;
}

int main(void){
  int w,n,i,l[31];
  scanf("%d\n",&w);
  scanf("%d\n",&n);
  for(i=1;i<=w;i++){
    l[i]=i;
  }
  for(i=0;i<n;i++) {
    int a,b;
    scanf("%d,%d\n",&a,&b);
    swap(&l[a],&l[b]);
  }
  for(i=1;i<=w;i++){
    printf("%d\n", l[i]);
  }

  return 0;
}