#include<stdio.h>
int main(){
  int n,s,i,j;
  while(scanf("%d %d",&n,&s),n||s){
    int c=0,d[20001];
    for(i=0;i<n;i++){
      scanf("%d",&d[i]);
      for(j=i;j;j--){
	if(d[i]+d[j-1]>s)c++;
      }
    }
    printf("%d\n",c);
  }
  return 0;
}