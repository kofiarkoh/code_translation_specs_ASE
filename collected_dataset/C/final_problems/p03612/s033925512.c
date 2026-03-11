#include<stdio.h>

void checkswaps(int p[], int leftswap[], int rightswap[], int equal[], int swapped[], int N, int *equalcount) 
{
 int i;

 for(i=1;i<N+1;i++)
 {
  if(p[i]==i)
  {
    (*equalcount)++;
    equal[i]=1;
   }else{
    equal[i]=0;
   }
   if(i>1)
   {
    if((p[i]!=(i-1))&(p[i-1]!=i))
    {
      leftswap[i]=1;
    }else{
      leftswap[i]=0;
    }
   }

  if(i<N)
  {
   if((p[i]!=(i+1))&(p[i+1]!=i))
   {
     rightswap[i]=1;
   }else{
     rightswap[i]=0;
   }
  }
 }


 // End points
 leftswap[1]=0;
 rightswap[N]=0;
 swapped[0]=1;
 for (i=1;i<N;i++) swapped[i]=0;
 swapped[N+1]=1;

}

void swapdata(int equal[], int leftswap[], int rightswap[], int swapped[], int p[], int N, int *swapcount)
{
 int i;
 int temp;

 for(i=1;i<N+1;i++)
 {
   if((equal[i]==1)&(leftswap[i]==1)&(swapped[i-1]==0)&(swapped[i]==0))
   {
     temp=p[i-1];
     p[i-1]=p[i];
     p[i]=temp;
     swapped[i]=1;
     swapped[i-1]=1;
     (*swapcount)++;
   }
   if((equal[i]==1)&(rightswap[i]==1)&(swapped[i]==0)&(swapped[i+1]==0))
   {
    temp=p[i+1];
    p[i+1]=p[i];
    p[i]=temp;
    swapped[i]=1;
    swapped[i+1]=1;
    (*swapcount)++;
   }
 }

}

int main()
{
 int N;
 int p[100005];
 int equal[100005];
 int leftswap[100005];
 int rightswap[100005];
 int equalcount=0;
 int i;
 int swapcount=0;
 int swapped[100005];
 int j;

 scanf("%d",&N);
 for (i=1;i<N+1;i++) scanf("%d",&p[i]);
 for(j=0;j<N; j++)
 {
   checkswaps(p, leftswap, rightswap, equal, swapped, N, &equalcount);
   if (equalcount==0) break;
   swapdata(equal, leftswap, rightswap, swapped, p, N, &swapcount);
 }

 printf("%d \n",swapcount);

 return 0;
}
