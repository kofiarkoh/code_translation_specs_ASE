/*
  AOJ 2708
  Title:ABC gene
  @kankichi573
*/
#include <stdio.h>
#include <string.h>


// replace all ABC to *
void findABC(char *in,char *out,int moji[3])
{
  char *p,*q;


  for(p=in,q=out;*p;)
    if(0==strncmp("ABC",p,3))
      {
	*q='*';q++;
	p+=3;
      }
    else
      {
	moji[*p-'A']++;
	*q++=*p++;
      }
    *q='\0';
}
void char_replace(char *s,char from,char to)
{
  char *p;
  for(p=s;*p;p++)
    if(*p == from)
      *p = to;

}

int main()
{
  char buf[5001],output[5001];
  int ret,i,no_use,moji[3];

  scanf("%s",buf);

  while(1)
    {
      if(0==strcmp("ABC",buf))
	{
	  ret=1;
	  //printf("SUCCES\n");
	  break;
	}
      if(strlen(buf) <= 4)
	{
	  ret=0;
	  //printf("FAIL 1\n");
	  break;
	}
      memset(moji,0,sizeof(moji));
      findABC(buf,output,moji);

      //printf("%s\n",output);

      if(2 != (moji[0]?1:0)+(moji[1]?1:0)+(moji[2]?1:0))
	{
	  ret=0;
	  //printf("FAIL 1\n");
	  break;
	}

      for(i=0,no_use=-1;i<3;i++)
	if(moji[i]==0)
	  {
	    no_use = i;
	    break;
	  }

      char_replace(output,'*','A'+no_use);

      //printf("%c",'A'+no_use);
      //printf("%s\n",output);
      strcpy(buf,output);
     
  //printf("%d",ret);
  }
  printf("%s\n",ret?"Yes":"No");


  return(0);
}