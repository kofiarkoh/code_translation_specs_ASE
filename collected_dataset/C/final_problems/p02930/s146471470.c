#include <stdio.h>

int main(void){
  int i,j;

  int n;
  scanf("%d",&n);

  int max=(n+1)/2;

  for(i=1;i<n;i++){
    for(j=1;i+j<n+1;j++){
      if(j<=max){
        printf("%d ",j);
      }else{
        printf("%d ",2*max-j+1);
      }
    }
    printf("\n");
  }

  return 0;
}
