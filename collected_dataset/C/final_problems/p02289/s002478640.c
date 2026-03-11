#include <stdio.h>
#define MAX 1000000

int A[MAX+1],H=0;

void insert(int);

int extract();
void increasekey(int,int);
void maxHeapify(int);

int main()
{
  int key;
  char com[20];
  while(1)
    {
      scanf("%s",com);
      if(com[0] == 'e' && com[1] == 'n')
	break;
      if(com[0] == 'i')
	{
	  scanf("%d",&key);
	  insert(key);
	}
      else
	printf("%d\n",extract());
    }
  return 0;
}

void insert(int key)
{
  H++;
  A[H] = -1;
  increasekey(H,key);
}

void increasekey(int i,int key)
{
  int k;
  if(key < A[H])
    return;
  A[i] = key;
  while(i > 1 && A[i/2] < A[i])
    {
      k = A[i/2];
      A[i/2] = A[i];
      A[i] = k;
      i = i/2;
    }
}

int extract()
{
  int maxv;
  if( H < 1)
    return -1;
  maxv = A[1];
  A[1] = A[H];
  H--;
  maxHeapify(1);
  return maxv;
}

void maxHeapify(int u)
{
  int l = 2 * u,r = 2 * u + 1,max,k;

  if(l <= H && A[l] > A[u])
    max = l;
  else
    max = u;
   if(r <= H && A[r] > A[max])
     max = r;


  if(max != u)
    {
      k=A[u];
      A[u] = A[max];
      A[max] = k;
      maxHeapify(max);
    }
}

