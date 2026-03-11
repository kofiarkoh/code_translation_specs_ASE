#include <stdio.h>
#include <stdlib.h>

#define ARRAY_SIZE 100
#define MEMORY_ERROR 0
#define MEMORY_SUCCESS 1

void swap(int *a, int *b) {
  int tmp;
  tmp = *a;
  *a = *b;
  *b = tmp;
}

int insert(int **list, int *size, int *memSize, int x) {
  int i, tmpN;
  int *tmpP;
  i = ++*size;
  if (*size > ARRAY_SIZE * *memSize) {
    ++*memSize;
    tmpP = realloc(*list, sizeof(int) * (ARRAY_SIZE * *memSize + 1));
    if (tmpP == NULL) return MEMORY_ERROR;
    if (*list != tmpP) *list = tmpP;
  }
  
  (*list)[i] = x;
  while (i > 1 && (*list)[i/2] < (*list)[i]) {
    swap(&(*list)[i/2], &(*list)[i]);
    i = i / 2;
  }

  return MEMORY_SUCCESS;
}

void setValue(int *list, int size, int index) {
  int left, right, largest, tmp;

  left = 2 * index;
  right = 2 * index + 1;

  if (left <= size && list[left] > list[index]) largest = left;
  else largest = index;

  if (right <= size && list[right] > list[largest]) largest = right;

  if (largest != index) {
    swap(&list[index], &list[largest]);
    setValue(list,size,largest);
  }
}

void deleteMax(int *list, int *size) {
  int tmp;
  swap(&list[1], &list[*size]);
  --*size;
  setValue(list,*size,1);
}

void terminate(int **listAry, int *sizeAry, int *memAry, int size) {
  int i;
  for (i = 0; i < size; i++) free(listAry[i]);
  free(listAry);
  free(sizeAry);
  free(memAry);
}

int main() {
  int i, n, q, com, index, x;
  int **listAry;
  int *sizeAry;
  int *memAry;

  scanf("%d %d",&n,&q);
  ++n;

  listAry = (int**)malloc(sizeof(int*)*n);
  sizeAry = (int*)malloc(sizeof(int)*n);
  memAry = (int*)malloc(sizeof(int)*n);

  for (i = 0; i < n; i++) sizeAry[i] = 0;
  for (i = 0; i < n; i++) memAry[i] = 1;
  for (i = 0; i < n; i++) listAry[i] = (int*)malloc(sizeof(int)*(ARRAY_SIZE+1));

  for (i = 0; i < q; i++) {
    scanf("%d %d",&com,&index);
    switch (com) {
      case 0:
        scanf("%d",&x);
        if (!insert(&listAry[index],&sizeAry[index],&memAry[index], x)) {
          terminate(listAry,sizeAry,memAry, n);
          return EXIT_FAILURE;
        }
        break;
      case 1:
        if (sizeAry[index] == 0) continue;
        printf("%d\n",listAry[index][1]);
        break;
      case 2:
        if (sizeAry[index] == 0) continue;
        deleteMax(listAry[index], &sizeAry[index]);
        break;
    }
  }

  terminate(listAry,sizeAry,memAry, n);

  return 0;
}
