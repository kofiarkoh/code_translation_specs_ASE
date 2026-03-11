#include <stdio.h>
typedef struct{
  int p;
  double t;
}run;
int main(void){
  run g[3][8],tmp,min1,min2;
  int i,j,k;
  for(i=0;i<3;i++){
    for(j=0;j<8;j++)
      scanf("%d %lf",&g[i][j].p,&g[i][j].t);
  }
  for(i=0;i<3;i++){
    for(j=0;j<7;j++){
      for(k=7;k>j;k--){
	if(g[i][k-1].t>g[i][k].t){
	  tmp=g[i][k-1];
	  g[i][k-1]=g[i][k];
	  g[i][k]=tmp;
	}
      }
    }
  }
  for(i=0;i<3;i++){
    for(j=0;j<2;j++)
      printf("%d %.2f\n",g[i][j].p,g[i][j].t);
  }
  min1.t=100;
  for(i=0;i<3;i++){
    for(j=2;j<8;j++){
      if(g[i][j].t<min1.t){
	min2=min1;
	min1=g[i][j];
      }
    }
  }
  printf("%d %.2f\n",min1.p,min1.t);
  printf("%d %.2f\n",min2.p,min2.t);
  return 0;
}