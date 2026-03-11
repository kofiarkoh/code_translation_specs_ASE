#include<stdio.h>
#include<string.h>
#define N 10000

int main(){
  int i,tlen,plen;
  char T[N*100],P[N];

  scanf("%s%s",T,P);
  tlen=strlen(T);
  plen=strlen(P);

  for(i=0;i<tlen;i++){
    if(strncmp(&T[i],P,plen)==0){
      printf("%d\n",i);
    }
  }
 
  return 0;
}