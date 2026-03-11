#include<stdio.h>
long long  n,m,i,a,b,c,s,d[(1<<17)+10];
int main(){
  while(scanf("%lld",&n),n){  
    for(i=0;i<(1<<17);i++)d[i]=0;
    while(n--){
      scanf("%lld %lld",&m,&c);
      while(m--){
	s=0;
	scanf("%lld %lld",&a,&b);
	for(i=a;i<b;i++)s+=1<<(i-6);
      }
      for(i=(1<<17);i>=s;i--){
	if((i&s)==s)d[i]=d[i]>d[i-s]+c?d[i]:d[i-s]+c;
      }
    }
    printf("%lld\n",d[(1<<17)-1]);
  }
  return 0;
}
  