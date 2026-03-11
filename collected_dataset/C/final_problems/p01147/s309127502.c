/*
  AOJ 2022
 */
#include <stdio.h>
#include <string.h>

int N;
int num[10];
int frag[10];
char words[11][11];
char result[101];
char shortest[101];

extern void strcat_erase_common(char *,char *);
void update_shortest(char * r)
{
  if(shortest[0]=='\0' || (strlen(shortest)>strlen(r))
     || ((strlen(shortest)==strlen(r)) && 0>strcmp(shortest,r)))
    {
      strcpy(shortest,r);
    }
}
void strcat_order(int * order)
{
  int i;

  memset(&result[0],'\0',11);
  strcpy(&result[0],&words[order[N]][0]);
  for(i=1;i<N;i++)
    strcat_erase_common(result,&words[order[N-i]][0]);

}

void strcat_erase_common(char * buf,char *add)
{
  int i,len;

  if(strstr(buf,add))
    return;

  len=strlen(buf);
  for(i=strlen(add)-1;i>0;i--)
    if(0==strncmp(buf+len-i,add,i))
      {
      strcat(buf,add+i);
      return;
      }
  strcat(buf,add);
  return;
}

void fx(int n,int k)
{
  int i,f,j;
  for(i=0;i<n;++i)
    {
      f=frag[i];
      if(f == 0)
	{
	  frag[i]=1;
	  num[k] = i+1;
	  if(k==1)
	    {
	      strcat_order(num);
	      update_shortest(result);
	    }
	  else
	    {
	      fx(n,k-1);
	    }
	  frag[i] = 0;
	}
    }
}

int main()
{
int i,k;


 while(EOF!=scanf("%d",&N) && N)
   {
     memset(num,0,N*sizeof(int));
     memset(frag,0,N*sizeof(int));

     for(i=1;i<=N;i++)
       scanf("%s ",&(words[i][0]));
#ifdef DEBUG2
  for(i=1;i<=N;i++)
    printf("%s:",&words[i][0]);
  printf("\n");
#endif
     memset(shortest,'\0',100);

     fx(N,N);
     printf("%s\n",shortest);
   }
 return(0);
}