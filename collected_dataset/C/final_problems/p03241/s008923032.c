#include<stdio.h>

int main()
{
 long N;
 long M;
 long i;
 long factor=1;
 long divisors;
 long rem;
 long commonshares;
 long out;

 scanf("%ld %ld",&N,&M);
 if((M%2)==0)
 {
  factor=2;
 }else{
  for(i=3;i<(N+M/N);i+=2)
  {
   if((M%i)==0)
   {
    factor=i;
    break;
   }
  }  
 }
 divisors=M/factor;
 commonshares=(divisors-(divisors%N))/N;
 rem=divisors%commonshares;
 if(rem==0)
 {
  out=factor*commonshares;
 }else{
  out=rem*factor;
 }
 printf("%ld\n",out);
 
 return 0;
}
