#include<stdio.h>
int main(){
  int n,m,s,i,j,t,d[101];
  while(scanf("%d",&n)!=EOF){
    for(i=0;i<n;i++){
      scanf("%d",&d[i]);
      for(j=i;j;j--){
	if(d[j-1]>d[j]){
	  t=d[j-1];
	  d[j-1]=d[j];
	  d[j]=t;
	}
      }
    }
    s=0;
    for(i=0;i<n;i++){
      s+=d[i]*(n-i);
    }
    printf("%d\n",s);
  }
  return 0;
}