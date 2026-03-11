#include<stdio.h>
#include<string.h>
int main(){
  char s[100][101];
  int d[100];
  char w[101];
  char c[101];
  int i,j,k,n,m,o,p;
  while(1){
    scanf("%d",&n);
    if(!n)break;
    for(i=0;i<n;i++)scanf("%s %d",s[i],&d[i]);
    scanf("%d",&m);
    for(i=0;i<m;i++){
      scanf("%s %d",w,&o);
      p=0;
      for(j=0;j<o;j++){
	scanf("%s",c);
	for(k=0;k<n;k++)if(strcmp(s[k],c)==0)break;
	p+=d[k];
      }
      for(j=0;j<n;j++)if(strcmp(s[j],w)==0)break;
      if(p<d[j])d[j]=p;
    }
    scanf("%s",w);
    for(i=0;i<n;i++)if(strcmp(s[i],w)==0)break;
    printf("%d\n",d[i]);
  }
  return 0;
}