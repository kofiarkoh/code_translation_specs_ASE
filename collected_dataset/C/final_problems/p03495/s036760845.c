#include <stdio.h>
#include <stdlib.h>

int ordena(const void *a, const void *b){
  return (*(int *)a)- (*(int *)b);
}

int vezes[200000];  //vezes que cada numero de a[i] apareceu;

int main (){

  int n,k,c=0,a;

  scanf("%d%d", &n,&k);

  for (int i = 0; i < n; i++){
    scanf("%d", &a);
    vezes[a-1]++;
  }

  qsort(vezes,n,sizeof(int), ordena);

  for(int i=0; i<n-k; i++)
    c+=vezes[i];

  printf("%d\n", c);


  return 0;
}
