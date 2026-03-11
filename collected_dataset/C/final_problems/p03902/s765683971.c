#include<stdio.h>
int h,w,d[1010][1010];
int f(int n){
  int i;
  for(i=0;i<w;i++){
    if(d[n][i]==d[n-1][i])continue;
    if(d[n][i]<d[n-1][i])return 1;
    return 0;
  }
  return 1;
}
int main(){
  int i,j,c=0;
  scanf("%d %d",&h,&w);
  for(i=0;i<h;i++){
    for(j=0;j<w;j++)scanf("%d",&d[i][j]);
  }
  for(i=1;i<h;i++){
    while(f(i)){
      for(j=1;j<w;j++)d[i][j]+=d[i][j-1];
      c++;
      if(c>10000010){
	c=-1;
	goto END;
      }
    }
  }
 END:;
  printf("%d\n",c);
  return 0;
}
