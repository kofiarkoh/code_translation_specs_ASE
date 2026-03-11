#include <stdio.h>
int n, cnt;
int A[1000000];

void shellSort();
void insertionSort(int);

int main() {
  int i;
  scanf("%d", &n);
  for(i = 0; i < n; i++)
  {
    scanf("%d", A+i);
  }
  shellSort();
  return 0;
}

void insertionSort(int g)
{
  int va;
  int i, j;
  for(i = g; i < n; i++)
  {
    va = A[i];
    j = i-g;

    while(j >= 0 && A[j] > va)
     {
      A[j+g] = A[j];
      j -= g;
      cnt++;
    }
    A[j+g] = va;
  }
}
void shellSort()
 {

  int G[100];
  int l = 0, g = 0;
  int i;

  while(3 * g + 1 <= n) {
    g = 3 * g + 1;
    G[l++] = g;

  }

  printf("%d\n", l);
  for(i = l - 1; i >= 0; i--)
   {

    if(i < l - 1) printf(" ");
    printf("%d", G[i]);
    insertionSort(G[i]);
  }
  puts("");


  printf("%d\n", cnt);

  for(i = 0; i < n; i++){
    printf("%d\n", A[i]);
  }
}