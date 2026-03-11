#include<stdio.h>
#include<stdlib.h>

int main(void)
{
  int n,d;
  int x[20][20];
  int count=0;
  scanf("%d %d",&n,&d);
  for(int i=0;i<n;i++){
    for(int j=0;j<d;j++){
      scanf("%d",&x[i][j]);
    }
  }

  for(int i=0;i<n;i++){
    for(int j=i+1;j<n;j++){
      int sum=0;
      for(int k=0;k<d;k++){
        sum+=abs(x[i][k]-x[j][k])*abs(x[i][k]-x[j][k]);
      }
      for(int l=1;l<=20;l++){
        if(l*l==sum){
          count++;
        }
      }
    }
  }

  printf("%d\n",count);
  return 0;
}
