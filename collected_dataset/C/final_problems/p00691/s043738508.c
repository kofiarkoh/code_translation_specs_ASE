#include<stdio.h>
int main(){
  int n,i,j,max,d[1500];
  for(i=0;i<1200;i++)d[i]=i*i*i;
  while(scanf("%d",&n),n){
    for(i=max=1;i<n;i++){
      for(j=1;d[i]+d[j]<d[n];j++){
	if(max<d[i]+d[j])max=d[i]+d[j];
      }
    }
    printf("%d\n",d[n]-max);
  }
  return 0;
}