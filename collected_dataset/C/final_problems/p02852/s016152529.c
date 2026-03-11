#include<stdio.h>
int main(){
  int n,m,i,j,p,np,d[100010];
  char s[100010];
  scanf("%d %d %s",&n,&m,s);
  for(i=0;i<n;i++)d[i]=1e9;
  d[n]=0;
  p=np=n;
  while(p){
    for(i=1;i<=m;i++){
      if(p-i<0)break;
      if(s[p-i]=='0'){
	d[p-i]=d[p]+1;
	np=p-i;
      }
    }
    if(np==p){
      printf("-1\n");
      return 0;
    }
    p=np;
  }
  //for(i=0;i<n;i++)printf("%d ",d[i]);printf("\n");
  while(p!=n){
    if(p)printf(" ");
    for(i=1;i<=m;i++){
      if(d[p]==d[p+i]+1)break;
    }
    printf("%d",i);
    p+=i;
  }
  printf("\n");
  return 0;
}
