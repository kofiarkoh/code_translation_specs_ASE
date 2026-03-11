#include<stdio.h>
#include <string.h>
#define N 50
#define M 100

int main(void){
  int n,i,sum=0,tmp;
  char name[N][M];
  int time[N];
  char x[M];
  
  scanf("%d",&n);
  for(i = 0;i < n;i++){
    scanf("%s %d",name[i],&time[i]);
  }
  scanf("%s",x);
  
  for(i=0;i<n;i++){
    if(strstr(name[i],x) != NULL){
    	tmp = i;
      	break;
    }
  }
  for(i=tmp+1;i<n;i++)
    sum+=time[i];
  printf("%d\n",sum);
  return 0;
}