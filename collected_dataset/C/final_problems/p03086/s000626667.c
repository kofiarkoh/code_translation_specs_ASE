#include<stdio.h>

int main()
{
  char S[11];
  int i;
  int cnt=0;
  int max=0;
  
  for(i=0;i<11;i++){
    S[i]='\0';
  }
  
  fgets(S,11,stdin);
  
  for(i=0;S[i]!='\0';i++){
    if(S[i]=='A'||S[i]=='T'||S[i]=='G'||S[i]=='C'){
      cnt++;
    }else{
      if(max<cnt) max=cnt;
      cnt=0;
    }
    if(max<cnt) max=cnt;
  }
  printf("%d",max);
  
  return 0;
  
}