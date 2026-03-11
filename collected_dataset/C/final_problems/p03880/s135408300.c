#include<stdio.h>
int f(int a){
  int i;
  for(i=0;a;i++)a/=2;
  return i;
}
int main(){
  int n,i,a=0,d,b[50]={};
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%d",&d);
    a^=d;
    b[f(d^(d-1))-1]=1;
  }
  //printf("%d\n",a);
  //for(i=0;i<5;i++)printf("%d ",b[i]);printf("\n");
  d=0;
  for(i=32;i--;){
    if(b[i]&&(a&(1<<i))){
      d++;
      a^=(1<<(i+1))-1;
    }
  }
  printf("%d\n",a?-1:d);
  return 0;
}
    
		 
