#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int salsa(int p)
{
  int r=0;
  int q=0;
  int a;
  while(p>0)
  {
    a=p%3;
    if(a==1) r+=2*(int)pow(3,q);
    if(a==2) r+=(int)pow(3,q);
    p/=3;
    q++;
  }
  return r;
}

int main(void)
{
  int n,m;
  int ans[531442];
  int sal[531442];
  int a=0;
  int b=0;
  char t[200001];
  int c[200001];
  int s=0;
  int r=0;

  scanf("%d",&n);
  scanf("%s",t);

  m=(int)pow(3,n);
  for(int i=0;i<m;i++)
  {
    ans[i]=i;
    sal[i]=salsa(i);
  }

  while(1)
  {
    if(t[a]=='S')
    {
      if(r>0)
      {
        if(b>0 && c[b-1]>0) c[b-1]+=r;
        else c[b++]=r;
        r=0;
      }
      s++;
    }
    if(t[a]=='R')
    {
      if(s>0)
      {
        if(s%2==1) c[b++]=0;
        s=0;
      }
      r++;
    }
    if(t[a]=='\n')
    {
      if(r>0) c[b++]=r;
      else if(s%2==1) c[b++]=0;
      break;
    }
    a++;
  }

  for(int i=0;i<b;i++)
  {
    if(c[i]==0)
    {
      for(int j=0;j<m;j++)
      {
        ans[j]=sal[ans[j]];
      }
    }
    else
    {
      for(int j=0;j<m;j++)
      {
        ans[j]=(ans[j]+c[i])%m;
      }
    }
    a++;
  }

  for(int j=0;j<m;j++)
  {
    printf("%d",ans[j]);
    if(j<m-1) printf(" ");
    else printf("\n");
  }

  return 0;
}