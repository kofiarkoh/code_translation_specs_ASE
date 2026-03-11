#include <stdio.h>
#include <stdlib.h>

typedef char                SINT8;
typedef unsigned char       UINT8;
typedef short               SINT16;
typedef unsigned short      UINT16;
typedef int                 SINT32;
typedef unsigned int        UINT32;
typedef long long           SINT64;
typedef unsigned long long  UINT64;

#define MAX(a,b) ((a)>(b)?(a):(b))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define ABS(a) ((a)>(0)?(a):-(a))
void quick_sort (int array[], int left, int right);
void swap_int (int *x,  int *y);
int partition (int array[], int left, int right);
void quick_sort2 (int array[], int left, int right);
int partition2 (int array[], int left, int right);

SINT32 city[100001];
SINT32 year[100001];
SINT32 num[100001];
SINT32 haya[100001];
SINT32 haya2[100001];


void swap_int (int *x,  int *y) {
    int temp;
    temp = *x;
    *x = *y;
    *y = temp;
}
 
int partition (int array[], int left, int right) {
    int i, j, pivot;
    i = left;
    j = right + 1;
    pivot = left;   // 先頭要素をpivotとする
 
    do {
        do { i++; } while (array[i] < array[pivot]);
        do { j--; } while (array[pivot] < array[j]);
        // pivotより小さいものを左へ、大きいものを右へ
        if (i < j) {
            swap_int(&array[i], &array[j]);
            swap_int(&city[i], &city[j]);
            swap_int(&num[i], &num[j]);
        }
    } while (i < j);
 
    swap_int(&array[pivot], &array[j]);   //pivotを更新
    swap_int(&city[pivot], &city[j]);   //pivotを更新
    swap_int(&num[pivot], &num[j]);   //pivotを更新
    return j;
}
 

void quick_sort (int array[], int left, int right) {
    int pivot;
    if (left < right) {
        pivot = partition(array, left, right);
        // pivotを境に再帰的にクイックソート
        quick_sort(array, left, pivot-1);
        quick_sort(array, pivot+1, right);
    }
}


int partition2 (int array[], int left, int right) {
    int i, j, pivot;
    i = left;
    j = right + 1;
    pivot = left;   // 先頭要素をpivotとする
 
    do {
        do { i++; } while (array[i] < array[pivot]);
        do { j--; } while (array[pivot] < array[j]);
        // pivotより小さいものを左へ、大きいものを右へ
        if (i < j) {
            swap_int(&array[i], &array[j]);
            swap_int(&city[i], &city[j]);
            swap_int(&haya2[i], &haya2[j]);
        }
    } while (i < j);
 
    swap_int(&array[pivot], &array[j]);   //pivotを更新
    swap_int(&city[pivot], &city[j]);   //pivotを更新
    swap_int(&haya2[pivot], &haya2[j]);   //pivotを更新
    return j;
}
 

void quick_sort2 (int array[], int left, int right) {
    int pivot;
    if (left < right) {
        pivot = partition2(array, left, right);
        // pivotを境に再帰的にクイックソート
        quick_sort2(array, left, pivot-1);
        quick_sort2(array, pivot+1, right);
    }
}


SINT32 main() {
    SINT32 n,m;
    scanf("%d%d",&n,&m);

    for (SINT32 i = 0; i < 100001; i++) {
        city[i] = 0;
        year[i] = 0;
        num[i] = 0;
        haya[i] = 0;
        haya2[i] = 0;
    }


    for (SINT32 i = 0; i < m; i++) {
        scanf("%d%d", &city[i],&year[i]);
        num[i] = i;
    }

    quick_sort(year, 0, m-1);

    for (SINT32 i = 0; i < m; i++) {
        haya2[i] = haya[city[i]] + 1;
        haya[city[i]] = haya[city[i]] + 1;
    }
/*
    for (SINT32 i = 0; i < m; i++) {
        printf("%d ",num[i]);
    }
    printf("\n");
    */

    quick_sort2(num, 0, m-1);
/*
    for (SINT32 i = 0; i < m; i++) {
        printf("%d ",num[i]);
    }
    printf("\n");
    */


    for (SINT32 i = 0; i < m; i++) {
        printf("%06d%06d\n",city[i],haya2[i]);
    } 

    
	return 0;
}