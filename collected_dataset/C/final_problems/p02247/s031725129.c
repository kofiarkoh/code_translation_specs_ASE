#include <stdio.h>
#include <string.h>
int equal(char *t,char *p,int i,int plen){
  int j;
  for(j=0;j<plen;j++){
    if(t[i+j]!=p[j]) return 0;
  }
   
  return 1;
}
int main(){
  char t[1000];
  char p[1000];
  int i,j;
  int tlen;
  int plen;
  scanf("%s",t);
  scanf("%s",p);
  tlen=strlen(t);
  plen=strlen(p);
  for(i=0;i<tlen;i++){
    if(equal(t,p,i,plen))printf("%d\n",i);
     
  }
  return 0;
}
