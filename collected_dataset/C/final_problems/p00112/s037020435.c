#include<stdio.h>
int main(){
  int n,i,s,j,t,m[10000];
  while(1){
  scanf("%d",&n);
  if(n==0)break;
  for(i=0;i<n;i++){
    scanf("%d",&m[i]);
  }
  for(i=0;i<n;i++){
    for(j=1;j+i<n;j++){
      if(m[j-1]>m[j]){
	t=m[j-1];
	m[j-1]=m[j];
	m[j]=t;
      }
    }
  }
  s=0;
  for(i=0;i<n;i++){
    s+=m[i]*(n-i-1);
  }
  printf("%d\n",s);
  }
  return 0;
}