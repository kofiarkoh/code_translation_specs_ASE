#include <stdio.h>

int main(void){
  int d,n,data=1;
  if(scanf("%d%d",&d,&n)==1);
    for(int i=0;i < d;i++){
      data *= 100;
    }
    if(n<=99){
      data *= n;
    }else{
      data *= 101;
    }

    printf("%d\n",data);

  return 0;
}
