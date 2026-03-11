#include<stdio.h>
#include<string.h>
int main(){
  char s[33],d[1000][33];
  int r=0,i,ml,mi,l=0,max=0,b[1000]={0};
  while(~scanf("%s",s)){
    for(i=0;i<r;i++){
      if(strcmp(s,d[i])==0)break;
    }
    if(i-r){
      b[i]++;
      if(max<b[i])max=b[mi=i];
    }
    else{
      b[r]=1;
      strcpy(d[r++],s);
      if(strlen(s)>l)l=strlen(d[ml=r-1]);
    }
  }
  printf("%s %s\n",d[mi],d[ml]);
  return 0;
}