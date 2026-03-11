/*
  AOJ #1378
 */
#include <stdio.h>

long long memo[100][100];

long long combi(int n, int r)
{
  long long ret;
  
  if(ret=memo[n][r])
    return(ret);

  if(r>n)
    ret=0;
  else if (r == 0 || r == n)
    ret=1;
  else if (r == 1)
    ret=n;
  else
    ret= (combi(n - 1, r - 1) + combi(n - 1, r));

  memo[n][r]=ret;
  return(ret);
}

long long solve(int l,int k)
{
  long long ret,c;
  int i,j;

  ret=0;
  for(i=(l+1)/2;i>=1;i--)
    for(j=0;(2*i-1)+j*(k-1)<=l;j++)
      {
	 // printf("ij=%d,%d combi=%lld\n",i,j,c=combi(i,j));
	 // ret += c;
	 ret += combi(i,j);
      }
  return(ret);
}


int main()
{
  long long ret;
  int l,k;
  
  scanf("%d %d",&l,&k);
  
  ret=solve(l,k);
  printf("%lld\n",ret);


}

