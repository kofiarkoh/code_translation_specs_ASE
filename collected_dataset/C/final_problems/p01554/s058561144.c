#include<stdio.h>
#include<string.h>
int main(){
  int n,d=1,m,i;
  char s[300][20],c[20],w[][20]={"Opened by","Closed by","Unknown"};
  scanf("%d",&n);
  for(i=0;i<n;i++)scanf("%s",s[i]);
  scanf("%d",&m);
  while(m--){
    scanf("%s",c);
    for(i=0;i<n&&strcmp(s[i],c);i++);
    printf("%s %s\n",i-n?w[d=1-d]:w[2],c);
  }
  return 0;
}