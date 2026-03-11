/*
  AOJ 1082
  Title:11224111122411
  @kankichi573
*/
#include <stdio.h>
#include <string.h>
#define min(x,y) (((x)<(y))?(x):(y))

char input[100001];

int calc_max(int n)
{
  if(n==8 || n==0)
    return(3);
  else
    return(5);
}
long long bunkatu(int n,int max_val)
{
  long long ret;
  int i;

  if(n==1)
    return(1);
  if(n==2)
    return(2);
  ret=1; //no divide
  for(i=1;i<=min(max_val,n-1);i++)
    ret += bunkatu(n-i,max_val);
  return(ret);
}
int main()
{
  int i;
  long long res;
  char prev;
  int  cnt;
  /*
  printf("%lld\n",bunkatu(1,5));
  printf("%lld\n",bunkatu(2,5));
  printf("%lld\n",bunkatu(3,5));
  printf("%lld\n",bunkatu(4,5));
  printf("%lld\n",bunkatu(5,5));
  printf("%lld\n",bunkatu(6,5));
  */
  
  while(scanf("%s",input) && input[0]!='#')
    {
      res=1;
      cnt=1;
      prev='\0';
      for(i=0;i<=strlen(input);i++)
	{
	  if(prev == input[i])
	    cnt++;
	  else
	    {
	      res *= bunkatu(cnt,calc_max(input[i-1]-'0'));
	      cnt = 1;
	    }
	  prev=input[i];
	  res %= 100000007;
	}
      printf("%lld\n",res);
    }
  
  return(0);
}