/*
  AOJ #3002
  title:Factrization
  @kankichi573
  2017/12/14 WA 35/37
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define max(x,y) (((x)>(y))?(x):(y))
char S[1000];
int  k[6];
int  jisuu;
int  insuu[6];

char delim[3]="+-";

int find_sign(char s[])
{
  char *p;
  int ret;

  for(p=s;*p;p++)
    if(*p=='+')
      {
        ret=1;
        goto END;
      }
    else
      {
        ret=-1;
        goto END;
      }
  ret=0;
 END:
  printf("f_s %s [%d]\n",s,ret);
}


int parse_monomial(char s[])
{ 
  char *p1,*p2;
  int a,b,ji;

  //printf("Mono=%s\n",s);
  
  
  p1=strchr(s,'x');
  p2=strchr(s,'^');

  
  if(p1 && p2 && p1!=s)
    {
      sscanf(s,"%dx^%d",&a,&b);
      ji=b;
      k[b]=a;
    }
  
  else if(p2 && p1==s)
    {
      sscanf(s,"x^%d",&b);
      ji=b;
      k[b]=1;
    }
    
  else if(p1 && !p2 && p1!=s)
    {
      if( 0==strcmp("+x",s))
        {
          ji=1;
          k[1]=1;
        }
      else if(0==strcmp("-x",s))
        {
          ji=1;
          k[1]=-1;
        }
      else
        {
          sscanf(s,"%dx",&a);
          ji=1;
          k[1]=a;
        }
    }

  else if(!p1 && !p2)
    {
      sscanf(s,"%d",&a);
      ji=0;
      k[0]=a;
    }
  return(ji);
}




int parse(char s[],int k[])
{
  char *p,*q,buf[100];
  int jisuu,ji,t;
  //printf("%s:\n",s);
  ji=0;
  t=0;
  p=q=s;
  while(1)
    {
      if(t==1)
        p++;
      while(*p!='+' && *p!='-' && *p)
        p++;
      memset(buf,0,sizeof(buf));
      strncpy(buf,q,p-q);
      jisuu=parse_monomial(buf);
      ji=max(ji,jisuu);
      q=p;
      t=1;
      if(!*p)
        return(ji);
    }
}
void dump()
{
  int i;
  for(i=5;i>=0;i--)
    printf("[%d],%d\n",i,k[i]);
}

long long calc_poly(int k[],int n)
{
  long long ret;
  ret =  k[5]*n*n*n*n*n;
  ret += k[4]*n*n*n*n;
  ret += k[3]*n*n*n;
  ret += k[2]*n*n;
  ret += k[1]*n;
  ret += k[0];
  return(ret);
}

int comp( const void * a , const void * b ) {

  if( *( int * )a < *( int * )b ) {
    return 1;
  }
  else
    if( *( int * )a == *( int * )b ) {
      return 0;
    }
  return -1;
}

void dump2()
{
  int i;
  for(i=0;i<jisuu;i++)
    printf("|%d\n",insuu[i]);
}

int main()
{
  int i,p;
  long long llret,llret2;

  scanf("%s",S);
  jisuu=parse(S,k);
  //dump();
  for(i=1,p=0;i<=200000;i++)
    if((k[0] % i)==0)
      {
        llret=calc_poly(k,i);
        llret2=calc_poly(k,-i);
        if(0==llret)
          insuu[p++]=i;
        if(0==llret2)
          insuu[p++]=-i;
      }

  qsort(insuu,jisuu,sizeof(int),comp);
  for(i=0;i<jisuu;i++)
    if(insuu[i]>0)
      printf("(x-%d)",insuu[i]);
    else
      printf("(x+%d)",-insuu[i]);
  printf("\n");
  
  return(0);
}