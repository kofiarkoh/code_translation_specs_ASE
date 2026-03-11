#include<stdio.h>

int main(void){
  int N,T,i,b;
  int max=0;
  int l[100000]={};
  int r[100000]={};
  int time[100000]={};
  scanf("%d %d",&N,&T);
  i=0;
  while(i<=N-1){
  scanf("%d %d",&l[i],&r[i]);
  b=l[i];
 
  for(b;b<r[i];b++)
  time[b]+=1;
  i++;
  }
 
  i=0;
  while(i<T){ 
    if(max<time[i])
      max=time[i];
      i++;
  }

  printf("%d\n",max);


}