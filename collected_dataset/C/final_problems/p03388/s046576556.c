#include<stdio.h>
#include<math.h>

int main(){
  int q,i;
  long long int a,b,t,s,an;
  scanf("%d",&q);
  for(i=0;i<q;i++){
    scanf("%ld %ld",&a,&b);
    t=a*b;
    s=sqrt(t);
    if(s*s==t) an=2*s-2;
    else if(s*(s+1)>=t) an=2*s-1;
    else an=2*s;
    if(a!=b) an--;
    printf("%ld\n",an);
  }
}