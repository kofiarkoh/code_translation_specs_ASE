#include<stdio.h>

int main()
{
  int i,j,N,Q,M,date,max,ii;
  int Date[101];

  while(1){
    scanf("%d%d",&N,&Q);
    if(N==0&&Q==0)break;
    for(i=0;i<101;i++)Date[i]=0;
    for(i=0;i<N;i++){
      scanf("%d",&M);
      for(j=0;j<M;j++){
	scanf("%d",&date);
        Date[date]++;
      }
    }
    max=Date[0];
    for(i=1;i<101;i++){
      if(max<Date[i]){
        max=Date[i];
	ii=i;
      }
    }
    if(max>=Q)printf("%d\n",ii);
    else printf("0\n");
  }
  return 0;
}