#include<stdio.h>

int main(){
  long long tone = 0;
  long long ttwo = 0;
  long long aone = 0;
  long long atwo = 0;
  long long bone = 0;
  long long btwo = 0;
  long long disone = 0;
  long long distwo = 0;
  long long cnt = 0;
  
  scanf("%lld %lld",&tone,&ttwo);
  scanf("%lld %lld",&aone,&atwo);
  scanf("%lld %lld",&bone,&btwo);
  
  if(tone*aone+ttwo*atwo==tone*bone+ttwo*btwo){
    printf("infinity");
  }else{
    disone = tone*aone - tone*bone;
    distwo = ttwo*atwo - ttwo*btwo;
  
    //printf("%ld %ld",disone,distwo);
  
    if(disone<distwo){
      cnt = distwo / (disone+distwo);
      cnt = cnt * 2 - 1;
    }else{
      cnt = disone / (disone+distwo);
      cnt = cnt * 2;
    }
    printf("%lld",cnt);
  }
    
  return 0;
}