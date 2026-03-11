#include<stdio.h>
int main(){
  int K,A,B,flag=0;
  scanf("%d",&K);
  scanf("%d %d",&A,&B);
  for(int i=1;K*i<=B;i++){
    if(K*i>=A && K*i<=B){printf("OK"); flag=1; break;}
  }
  if(flag==0) printf("NG");
}