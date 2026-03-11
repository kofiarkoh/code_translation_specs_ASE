#include <stdio.h>

int main(){
  int N,a[110],i,num[10],temp,sum,max,min;
  
  //Nとa[i]を入力
  scanf("%d",&N);
  for(i=0; i<N; i++){
  	scanf("%d",&a[i]);
  }
  
  //num[i],sum,max,minを初期化
  for(i=0; i<9; i++){num[i]=0;}
  sum=0; max=-1; min=10;
  
  //a[i]がどのクラスnum[i]に入るか分別
  //num[i]該当者がいればnum[i]をインクリ
  for(i=0; i<N; i++){
  	temp=a[i]/400;
  	  if(temp>=8){temp=8;}
    num[temp]++;
  }
  
  for(i=0; i<8; i++){
  	if(num[i]!=0){sum++;}
  }
  
  min=sum;
  max=sum+num[8];
  	if(max>8){max=8;}
  	if(min==0){min=1;}
  
  printf("%d %d",min,max);
  
  return(0);
}
