#include<stdio.h>
int main(){
  int a,b=0,c=0;
  scanf("%d",&a);
  int d[a];
  for(int i=0; i<a; i++){scanf("%d",&d[i]);}
  for(int i=a-1; 0<i; i--){
    for(int j=0; j<i; j++){
      if(d[j]<d[j+1]){
        int e=d[j];
        d[j]=d[j+1];
        d[j+1]=e;
      }
    }
  }
  for(int i=0; i<a; i++){
    if(i%2){c+=d[i];}else{b+=d[i];}
  }
  printf("%d",b-c);
  return 0;}
