/*
  AOJ #1531 
  title:Remainder Zero
  @kankichi57301
 */
#include <stdio.h>
long long a[5],b[5],gcd_a,qb[5],gcd_qb;
int f_expt[14];
int m,n,ret;

long long gcd(long long a, long long b)
{
        long long c;

        while (b > 0) {
                c = a % b;
                a = b;
                b = c;
        }

        return a;
}

void factorize(long long num){
  int i,f,f1,fc;
    
    f=1,fc=-1;
    
    for (i = 2; i <= num; i++) // 他の素数で割っていく
    {
	f=1;
        while (num % i == 0)
	{
	    if(f)
	    {
		f=0;fc++;
	    }
	    f_expt[fc]++;
	    num/=i;
	}
	
    }
}

int solve()
{
  int ret,i;
  for(i=0,ret=1;i<14;i++)
    ret *= (1+f_expt[i]);
  return(ret);
}



int main()
{
  int i;
  
  scanf("%d %d",&n,&m);
  for(i=0;i<n;i++)
    scanf("%lld",&a[i]);
  for(i=0;i<m;i++)
    scanf("%lld",&b[i]);
  gcd_a=a[0];
  for(i=1;i<n;i++)
     gcd_a = gcd(gcd_a,a[i]);
  
  //printf("gcd[a]=%lld\n",gcd_a);
  
  for(i=0;i<m;i++)
    {
      if((gcd_a % b[i])>0)
	{
	  printf("0\n",ret);
	  return(0);
	}
      qb[i]=gcd_a/b[i];
    }
  gcd_qb=qb[0];
  
  for(i=1;i<m;i++)
    {
      gcd_qb = gcd(gcd_qb,qb[i]);
    }
  
  factorize(gcd_qb);
  
  ret=solve();
  printf("%d\n",ret);
  
  return(0);
}

