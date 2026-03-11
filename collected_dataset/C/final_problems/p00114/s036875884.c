#include<stdio.h>

int gcd(int a,int b){
  int c;
  while((c=a%b)!=0){
    a=b;
    b=c;
  }
  return b;
}
int main(){
  long a1,m1,a2,m2,a3,m3,count,x1,x2,x3,i,ac,bc,cc;
  long long int lc;
  do{
    scanf("%ld %ld %ld %ld %ld %ld",&a1,&m1,&a2,&m2,&a3,&m3);
    if(!a1&&!m1&&!a2&&!m2&&!a3&&!m3) break;
    for(i=a1,ac=1;i!=1;i=(a1*i)%m1,ac++);
    for(i=a2,bc=1;i!=1;i=(a2*i)%m2,bc++);
    for(i=a3,cc=1;i!=1;i=(a3*i)%m3,cc++);
    lc=ac*bc/gcd(ac,bc);
    lc=lc*cc/gcd(lc,cc);
    printf("%lld\n",lc);
    
  }while(1);
  return 0;
}