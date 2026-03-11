#include<stdio.h>
#include<stdlib.h>

int main()
{
	int N,A,B,C,D;
	char S[200005];
	int i,j;
	int maxDC;
	scanf("%d %d %d %d %d",&N,&A,&B,&C,&D);
	scanf("%s",&S);
	if(C<D)
	{
	// Move B first to D
	  for(i=B;i<D;i++) 
	  {
		  if((S[i-1]=='#')&(S[i]=='#'))
		  {
			  printf("No\n");
			  return 0;
		  }
	  }
	// Move A to C
	  for(i=A;i<C;i++)  
          {
                  if((S[i-1]=='#')&(S[i]=='#'))
                  {
                          printf("No\n");
                          return 0;
                  }
          }
	  printf("Yes\n");
	  return 0;
	}else{
		// Find a spot where A can pass B
	        for(i=1;i<N-2;i++) 
		{
		   if((S[i-1]=='.')&(S[i]=='.')&(S[i+1])=='.')
		   {
			   // if the space is before A
			   if(i<A)
			   {
				   // check if can move A and B there
				   for(j=i;j<maxDC;j++) if((S[j-1]=='#')&(S[j]=='#')) break;
				   printf("Yes\n");
				   return 0;
			   }
			   // if the space is between A and B
			   if((i>=A)&(i<=B))
			   {
				   // check if can move A and B there
                                   for(j=A;j<B;j++) if((S[j-1]=='#')&(S[j]=='#')) break;
				   printf("Yes\n");
				   return 0;
			   }
			   // if the space is after B but before D
			   if((i>B)&(i<=D))
			   {
				   for(j=B;j<D;j++) if((S[j-1]=='#')&(S[j]=='#')) break;
                                   printf("Yes\n");
                                   return 0;
                           }
			   // if the space is after D
                           if(i>D)
                           {
                                   for(j=B;j<i;j++) if((S[j-1]=='#')&(S[j]=='#')) break;
                                   printf("Yes\n");
                                   return 0;
                           }
		   }
		}
		printf("No\n");
		return 0;
	   }
	return 0;
}
