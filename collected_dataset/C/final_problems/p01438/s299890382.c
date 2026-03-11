#include<stdio.h>
int n,m,i,a,b,c,s,d[(1<<23)+10];
int main(){
  while(scanf("%d",&n),n){  
    for(i=0;i<(1<<17);i++)d[i<<6]=0;
    while(n--){
      scanf("%d %d",&m,&c);
      while(m--){
	s=0;
	scanf("%d %d",&a,&b);
	for(i=a;i<b;i++)s+=1<<i;
      }
      for(i=(1<<23);i>=s;i--){
	if((i&s)==s)d[i]=d[i]>d[i-s]+c?d[i]:d[i-s]+c;
      }
    }
    printf("%d\n",d[(1<<23)-1]);
  }
  return 0;
}
  