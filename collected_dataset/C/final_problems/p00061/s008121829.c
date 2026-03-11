#include<stdio.h>
int main(){
  int n,c=1,s,t,d[101]={0};
  int i,j,r[101];
  while(1){
    scanf("%d,%d",&n,&t);
    if(n==0&&t==0)break;
    d[n]=t;
  }
  for(i=30;i>=0;i--){
    s=0;
    for(j=1;j<101;j++){
      if(d[j]==i){
	r[j]=c;
	s=1;
      }
    }
    if(s==1)c++;
  }
  while(scanf("%d",&n)!=EOF){
    printf("%d\n",r[n]);
  }
  return 0;
}