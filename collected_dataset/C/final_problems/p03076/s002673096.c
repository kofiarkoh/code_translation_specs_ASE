#include<stdio.h>

int main(){
  
  int menu[5], sum=0, div=10, i;
  
  if(scanf("%d", &menu[0])==0) return -1;
  if(scanf("%d", &menu[1])==0) return -1;
  if(scanf("%d", &menu[2])==0) return -1;
  if(scanf("%d", &menu[3])==0) return -1;
  if(scanf("%d", &menu[4])==0) return -1;

  for(i=0;i<5;i++){	
    if(menu[i]%10==0) sum+=menu[i];
  	else sum+=(menu[i]+(10-menu[i]%10));
  }
  for(i=0;i<5;i++){
    if(menu[i]%10==0) continue;
    else if(menu[i]%10<div) div=menu[i]%10;
  } 
  
  printf("%d", sum-10+div);
  
  return 0;
}