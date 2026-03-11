#include <stdio.h>

int main(){
  int i,n,d=0,c[2]={},f;
  char t[1000005];
  long x=0;
  scanf("%s",&t);
  for(n=0;t[n]!=0;n++){
    if(t[n]=='1'){
      d++;
      x+=(long)((d+1)>>1);
    }
  }
  f=d&1;
  for(i=n-1;i>=0;i--){
    f^=1;
    if(t[i]=='1'){
      d--;
      c[f]++;
    }else{
      x+=(long)((d+1)>>1);
      x+=(long)(c[f]);
    }
  }
  printf("%ld",x);
  return(0);
}