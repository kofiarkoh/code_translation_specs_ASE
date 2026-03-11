#include<stdio.h>
int main(){
  char s[1027];
  int n;
  int i,j,g;
  scanf("%d\n",&n);
  for(i=0;i<n;i++){
    char t[55]={0};
    g=25;
    fgets(s,1026,stdin);
    for(j=0;1;j+=3){
      t[g]=s[j];
      if     (s[j+1]=='-')g++;
      else if(s[j+1]=='<')g--;
      else    break;
    }
    for(j=0;t[j]==0;j++);
    for(;t[j]!=0;j++)printf("%c",t[j]);
    printf("\n");
  }
  return 0;
}