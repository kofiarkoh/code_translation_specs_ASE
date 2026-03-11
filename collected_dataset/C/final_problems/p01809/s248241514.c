/*
  AOJ #2706
  @kankichi573
 */
#include <stdio.h>
#include <float.h>
#include <math.h>

int gcd(int a, int b)
{
        int c;

        while (b > 0) {
                c = a % b;
                a = b;
                b = c;
        }

        return a;
}

void factorize(int number,int fact[],int *nfact)
{
  int d;

  *nfact=0;
  d=2;
  while(number>1)
    {
      if(d > sqrt(number))
	{
	  fact[(*nfact)++]=number;
	  break;
	}
      if((number % d)==0)
	{
	  fact[(*nfact)++]=d;
	  while((number % d)==0)
	    number /= d;
	}

	if(d==2)
	  d=3;
	else
	  d+=2;
    }
}

int main()
{
  int i;
  int p,q,g,fact[1000],nf,ret;
  
  scanf("%d %d",&p,&q);
  g = gcd(p,q);
  q /= g;
  
  factorize(q,fact,&nf);
  //printf("nf=%d\n",nf);
  for(i=0,ret=1;i<nf;i++)
    ret *= fact[i];

  printf("%d\n",ret);

  return(0);

}