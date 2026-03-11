#include<stdio.h>

int main(void){
 int m,n,tmp,i,j,k;
 short flg[100001]={0},stack[200000];
 
 scanf("%d%d",&n,&m);
 for(i=0;i<m;i++){
  scanf("%d",&tmp);
  flg[tmp]=1;
  stack[i]=tmp;
 }
 for(i=m-1;i>=0;i--){
  if(flg[stack[i]]==1) printf("%d\n",stack[i]);
  flg[stack[i]]=2;
 }
 for(i=1;i<=n;i++) if(flg[i]==0) printf("%d\n",i);
 return 0;
}