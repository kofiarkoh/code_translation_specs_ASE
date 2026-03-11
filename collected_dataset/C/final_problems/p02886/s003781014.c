#include <stdio.h>

int main(void)
{
 int N,d[100],s,i,j;

 scanf("%d",&N);

 for(i=0;i<N;i++){
  scanf("%d",&d[i]);
 }
 s=0;
 for(i=0;i<N;i++){
  for(j=i+1;j<N;j++){
   s=s+d[i]*d[j];
  }
 }

  printf("%d",s);

 return 0;
}
