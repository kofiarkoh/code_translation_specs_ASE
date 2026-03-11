/*
  AOJ 1082
  Title:11224111122411
  @kankichi573
  2014/2/18 TLE
*/
#include <stdio.h>
#include <string.h>
#define min(x,y) (((x)<(y))?(x):(y))
#define LIMIT 100000

char input[100001];
long long memo[2][LIMIT];


int calc_max(int n)
{
  if(n==8 || n==0)
    return(3);
  else
    return(5);
}
unsigned long long bunkatu(int n,int max_val)
{
  unsigned long long ret;
  int i;
  
  if(n<LIMIT)
    if(max_val==5)
      {
      if(ret=memo[0][n])
	return(ret);
      }
    else
      {
        if(ret=memo[1][n])
	  return(ret);
      }
  
  if(n==1)
    ret=1;
  else if(n==2)
    ret=2;
  else
    {
      ret=1; //no divide
      for(i=1;i<=min(max_val,n-1);i++)
	{
	  ret += bunkatu(n-i,max_val);
	  ret %= 100000007;
	}
    }
  
  if(max_val==5)
    memo[0][n]=ret;
  else
    memo[1][n]=ret;
  
  return(ret);
}

int main()
{
  int i;
  unsigned long long res;
  char prev;
  int  cnt;
  int  len;

    
  (void)bunkatu(100000,5);
  (void)bunkatu(100000,3);

  while(scanf("%s",input) && input[0]!='#')
    {
      res=1;
      cnt=1;
      prev='\0';
      len=strlen(input);
      for(i=0;i<=len;i++)
	{
	  if(prev == input[i])
	    {
	      cnt++;
	    }
	  else
	    {
	      res *= bunkatu(cnt,calc_max(input[i-1]-'0'));
	      res %= 100000007;
	      cnt = 1;
	    }
	  prev=input[i];
	}
      printf("%llu\n",res);
    }
  
  return(0);
}