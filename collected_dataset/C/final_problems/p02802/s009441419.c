#include <stdio.h>
int main(void){
  int N,M;
  if(scanf("%d%d",&N,&M)<2) return 1;
  int i,p[M];
  char S[M][2];
  for(i=0;i<M;i++){
    if(scanf("%d%s",&p[i],&S[i])<2) return 1;
  }
  int j,flag=0,ac=0,pn=0,temp=0;
  for(j=1;j<=N;j++){
    for(i=0;i<M;i++){
      if(p[i]==j){
        if(S[i][0]=='W'){
          temp++;
        }
        else{
          if(!flag){
            flag=1;
            ac++;
            pn += temp;
          }
        }
      }
    }
    temp=0;
    flag=0;
  }
  printf("%d %d\n",ac,pn);      
  return 0;
}