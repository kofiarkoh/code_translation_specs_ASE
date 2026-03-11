#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <limits.h>
#include <stdbool.h>

typedef double db;
typedef int in;
typedef long int li;
typedef long long int lli;
typedef long ld;
typedef long lld;

#define pi M_PI
#define in scanf
#define out printf
#define inf INT_MAX
#define rep(i,n) for(int i=0;i<n;++i)
#define repi(i,j,n) for(int i=j;i<n;++i)
#define debugd(d) printf("%d\n",d)
#define debugs(s) printf("%s\n",s)
#define debugc(c) printf("%c\n",c)
#define debugf(f) printf("%f\n",f)

int compare_int(const void *a, const void *b)
{
    return *(int*)a - *(int*)b;
}
int compare_intr(const void *a, const void *b)
{
    return *(int*)b - *(int*)a;
}



int main (void)
{
  char s[52],t[52];
  scanf("%s\n",s);
  scanf("%s\n",t);
  int slen = strlen(s);
  int tlen = strlen(t);
  int chk=0;
  int i=0,j=0;

  rep(i,slen)
  {
    if (s[slen-i]=='?') chk++;
    else if (s[slen-i]!=t[tlen-chk-1]) chk=0;
    else if (s[slen-i]==t[tlen-chk-1]) chk++;

    if (chk==tlen) 
    {
      rep(j,tlen)
      {
        s[slen-i+j]=t[j];
      }
      rep(j,slen)
      {
        if(s[j]=='?') s[j]='a';
      }
      puts(s);
      return 0;
    }
  }
  puts("UNRESTORABLE");
  return 0;
}