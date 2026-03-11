#include <stdio.h>
#define NIL -1
#define MAX 2000005
#define MIN -100000000

int T[MAX];
int hs;

void swap(int, int);
void maxHeapify(int);
void heapIncreaseKey(int, int);
void maxHeapInsert(int);
int heapExtractMax(void);
int parent(int);
int left(int);
int right(int);


int main()
{
  int i, a;
  char c[10];
 
  for(i=0; i<MAX; i++){
    T[i]=NIL;
  }

  for(i=1; ;i++){
    scanf("%s ",c);

    if(c[0]=='i'){
      scanf("%d", &a);
      maxHeapInsert(a);
    }
    else if(c[1]=='x'){
      a = heapExtractMax();
      printf("%d\n",a);
    }
    else break;
  }
  return 0;
}
  

void swap(int x, int y)
{
  int temp;
  temp=T[x];
  T[x]=T[y];
  T[y]=temp;
}


void maxHeapify(int i)
{
  int l = left(i);
  int r = right(i);
  int largest;

  if(l <= hs && T[l] > T[i]){
	largest = l;
  }
  else{
	largest = i;
  }
  if(r <= hs && T[r] > T[largest]){
	largest = r;
  }
  if(largest != i)
	{
	  swap(i, largest);
	  maxHeapify(largest);
	}
}


void heapIncreaseKey(int i, int key)
{
  if(key < T[i]) return;

  T[i] = key;
  while(i>1 && T[parent(i)] < T[i]){
    swap(i, parent(i));
    i = parent(i);
  }
}


void maxHeapInsert(int key)
{
  hs += 1;
  T[hs] = MIN;
  heapIncreaseKey(hs, key);
}


int heapExtractMax()
{
  int max;

  if(hs < 1){
    return NIL;
  }

  max = T[1];
  T[1] = T[hs];
  hs -= 1;
  maxHeapify(1);
  return max;
}

 
int parent(int i)
{
  return i/2;
}


int left(int i){
  return 2*i;
}


int right(int i){
  return 2*i+1;
}