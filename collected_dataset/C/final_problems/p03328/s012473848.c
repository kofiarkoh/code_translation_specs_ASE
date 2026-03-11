#include <stdio.h>

int main(){
  int a,b;
  scanf("%d%d",&a,&b);

  int i,sum=0;
  for(i=0;i<=b-a;i++){
    sum+=i;
  }

  printf("%d",-b+sum);
}

