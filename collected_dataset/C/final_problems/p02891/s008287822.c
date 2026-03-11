#include<stdio.h>
#include<string.h>
typedef long long ll;
int main(void)
{
  char s[128];
  ll k;
  scanf("%s %lld",s,&k);
  int len=strlen(s);

  int start=0;
  int count=0;
  while(start<len){
    int tmp=0;
    while(s[start]==s[start+1]){
      start++;
      tmp++;
    }
    if(tmp>0){
      count+=(tmp+1)/2;
    }
    start++;
  }

  printf("%lld\n",count*k);
  return 0;
}
