/*
  AOJ #1564
  title:String Compression
  @kankichi573
 */
#include <stdio.h>
#include <string.h>
#define min(x,y) (((x)<(y))?(x):(y))
int alp[28];
int num[10];
char s[101];
int get_longest_span(int a[],int lim,int *st,int * h)
{
  int i,c;
  for(i=0;a[i]==0 && i<lim;i++)
    ;
  for(*h=100,*st=i;a[i] && i<lim;i++)
    { c=a[i];
      *h=min(*h,c);
    }

  return(i-*st);
}
void subtract_span(int a[],int st,int len,int h)
{
  int i;
  for(i=st;i<st+len;i++)
    a[i] -= h;
}

int solve(int a[],int lim)
{
  int ret,len,st,h;
  ret=0;

  while(1)
    {
      len=get_longest_span(a,lim,&st,&h);
      if(len==0)
	break;
      //printf("st=%d len=%d h=%d\n",st,len,h);
      subtract_span(a,st,len,h);
      
      ret += min(3,len)*h;
      
    }
  return(ret);
}


int main()
{
  int ret;
  char *p;

  
  scanf("%s",s);

  for(p=s;*p;p++)
    {
      if((*p >= 'a') && (*p <= 'z'))
	 alp[*p-'a']++;
      if((*p >= '0') && (*p <= '9'))
	num[*p-'0']++;
    }
  ret=solve(alp,26)+solve(num,10);
  printf("%d\n",ret);
  
  return(0);
}

