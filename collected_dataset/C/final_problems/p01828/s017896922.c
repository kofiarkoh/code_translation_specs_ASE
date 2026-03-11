/*
AOJ #2727
title:M and A
@kankichi573
2016/11/3 TLE 23/47
*/

#include <stdio.h>
#include <string.h>

int my_longest_match(char *s1,char *s2)
{
  char *p;

  for(;*s1 && *s2;s1++)
    {
      if(!(p=strchr(s2,*s1)))
	return(0);
      s2=p+1;
    }
  if(!*s1)
    return(1);
  else
    return(0);
}

char s[1001],t[1001];

int solve(char *s,char *t)
{

  int i,j,k,len = strlen(s),r1,r2;
  char buf[2][501];

  for(i=j=k=0;i<len;i++)
    if((i % 2)==0)
      buf[0][j++] = s[i];
    else
      buf[1][k++] = s[i];

  buf[0][j]=buf[1][k]='\0';


  //printf("%s\n",&buf[0][0]);
  //printf("%s\n",&buf[1][0]);

  r1=my_longest_match(&buf[0][0],t) ;
  r2=my_longest_match(&buf[1][0],t);
 
  //printf("r1,r2=%d %d\n",r1,r2);
 
  return(r1||r2);
}


int main()
{
  int ret;
  
  scanf("%s",s);
  scanf("%s",t);
  ret=solve(s,t);
  
  printf("%s\n",ret?"Yes":"No");
  return(0);
}