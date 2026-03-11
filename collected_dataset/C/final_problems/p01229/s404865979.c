#include<stdio.h>
int main(){
  int n,m,s,d[100],i;
  scanf("%d",&m);
  while(m--){
    scanf("%d",&n);
    for(i=s=0;i<n;i++)scanf("%d",&d[i]);
    for(i=n;i--;){
      if(i%2-n%2)s+=d[i];
      else if(d[i]==0){
	if(i<2||d[i-2]==0||d[i-1]==0)break;
	d[i-2]--;
	d[i-1]--;
      }
    }
    printf("%d\n",s);
  }
  return 0;
}