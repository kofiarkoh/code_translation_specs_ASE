#include <stdio.h>

int i;
int main(void){
  
  
  int now=0;
  long ans=1;
  int terget;
  int n,m;//top,breaking
  int breaker;
  int max=0;
  scanf("%d %d",&n, &m);
  int inter[m+1];
  for(i=0;i<m;i++){
    scanf("%d",&breaker);
    inter[i]=breaker - now-1;
    //printf("%d\n",inter[i]);
    if(inter[i]==-1){
        ans=0;

    }
    if(inter[i] >max){
      max=inter[i];
    }
    now=breaker+1;
  }
  inter[m]=n-now;
  if(inter[m] >max){
      max=inter[m];
    }
     if(inter[i]==-1){
        ans=0;
        
    }
//printf("%d\n",inter[i]);
  int fer[max+1];
  for (i=0;i<max+1;i++){
    switch (i){
      case 0:
        fer[i]=1;
        break;
      case 1:
        fer[i]=1;
        break;
      case 2:
       fer[i]=2;
       break;
      default:
        fer[i]=fer[i-2]+fer[i-1];
      break; 
     
    }
  //  printf("%d;%d\n",i,fer[i]);
  }
  for(i=0;i<m+1;i++){
    if(inter[i]==-1){
        ans=0;
    }else{
    ans*=fer[inter[i]];
        
    }
    ans%=1000000007;
   //  printf("%ld\n",ans);
  }
  printf("%ld\n",ans);
  return 0;
}