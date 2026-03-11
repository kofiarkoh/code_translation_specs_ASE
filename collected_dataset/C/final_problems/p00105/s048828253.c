#include<stdio.h>
#include<string.h>

int main(){

  char tempw[256],word[128][128],dict[128][128]={{0}};
  int num[128],page[128][128],mark[128]={0};
  int i,j,k,l,m,n,p,r,s,t,tempn,temp;
  i=0;

  while(scanf("%s %d",word[i],&num[i])!=EOF)
    {
     if(i>0)
	{
	  if(j<i)
	    {
	      for(j=0;j<i;j++)
		{
		  if(strcmp(word[j],word[j+1])>0)
		    {
		      strcpy(tempw,word[j]);
		      strcpy(word[j],word[j+1]);
		      strcpy(word[j+1],tempw);
		      tempn=num[j];
		      num[j]=num[j+1];
		      num[j+1]=tempn;
		      j=0;
		    }
		}
	    }
	}
      i++;
    }	

  for(l=0;l<i;l++)
    {
      for(m=l+1;m<i;m++)
	{
	  if(strcmp(word[l],"null")!=0)
	    {
	      if(strcmp(word[l],word[m])==0)
		strcpy(word[m],"null");
	      else
		break;
	    }
	}
    }
  p=0;
  // i--;
  for(l=0;l<i;l++)
    {
      if(strcmp(word[l],"null") != 0)
	{
	  strcpy(dict[p],word[l]);
	  p++;
	}
      else if(strcmp(word[l],"null") == 0)
	{
	  mark[l] = 100;
	}

    }
  r=0;//現在の単語の重複数
  s=0;//総重複数
  for(k=0;k<i;k++)
    {  
      if(mark[k]==0)
	{
	  r=0;
	  page[k-s][0]=num[k];
			
	}
      else
	{
	  r++;
	  s++;
	  page[k-s][r]=num[k];	
	}
    }

    for(k=0;k<i-s;k++)
    {
      for(t=0;t<i;t++)
	{
	  if(page[k][t] > page[k][t+1])
	    {
	      temp = page[k][t];
	      page[k][t] = page[k][t+1];
	      page[k][t+1] = temp;
	      t=-1;
	    }
	 else if(page[k][t]==0)
	    break;
	}
	}
    
  for(k=0;k<i-s;k++)
    {
      //printf("i=%d\n",k);
      // printf("word[i] %s\n",word[k]);
      printf("%s\n",dict[k]);
      //  printf("num[i] %d\n",num[k]);
      //printf("mark[i] %d\n",mark[k]);
      for(t=1;t<i-s;t++)
	{
	  if(t==1)
	    printf("%d",page[k][t]);
	  else if(page[k][t] != 0)
	          printf(" %d",page[k][t]);
	  else
	    {
	      printf("\n");
	      break;
	    }
	}
    }
  // printf("--------------------\n");	
  // i++;
    
  return 0;
}