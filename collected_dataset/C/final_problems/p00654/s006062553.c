#include<stdio.h>
#include<math.h>
int main(){
  int n,c,i,m;
  double b;
  long long a,d[250];
  while(scanf("%d",&n),n){
    m=(n*(n+1))/2-1;//printf("m%d\n",m);
    for(i=0;i<n;i++)scanf("%lld",&d[i]);
    scanf("%lld",&a);
    b=(double)d[0]/a;
    c=b*d[1];
    c=sqrt(c);
    printf("%d\n",c);
    for(i=0;i<n;i++){
      if(i)printf(" ");
      printf("%lld",d[i]/c);
    }
    for(i=n;i<m;i++)scanf("%lld",&a);
    printf("\n");
  }
  return 0;    
}