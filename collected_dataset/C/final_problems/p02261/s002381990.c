#include <stdio.h>
#include <string.h>

#define N 36

typedef struct{
  char gara;
  int num;
} Card;

void BubbleSort(Card *, int);
void SelectionSort(Card *, int);


int main(void){

  Card c1[N],c2[N];
  int i,j,k,n;
  char dum;

  scanf("%d ",&n);

  for(i=0;i<n;i++){
    scanf("%c%d%c",&c1[i].gara,&c1[i].num,&dum);
    c2[i] = c1[i];
  }


  BubbleSort(c1,n);
  printf("Stable\n");
  SelectionSort(c2,n);
  for(k=0;k<n;k++){
    if(c1[k].gara != c2[k].gara){
      printf("Not stable\n");
      break;
    }
    if(k == n - 1){
      printf("Stable\n");
    }
  }
  return 0;
}

void BubbleSort(Card *A,int n){

  int i,j;
  Card k;

  for(i=0;i<n;i++){
    for(j=n-1;j>=i;j--){
      if(A[j].num < A[j - 1].num){
	k = A[j];
	A[j] = A[j - 1];
	A[j - 1] = k;
      }
    }
  }
 
  for(i=0;i<n;i++){
    if(i == n - 1){
      printf("%c%d\n",A[i].gara,A[i].num);
      break;
    }
    printf("%c%d ",A[i].gara,A[i].num);
  }
}

void SelectionSort(Card *A, int n){

  int i,j,minj;
  Card k;

  for(i=0;i<n;i++){
    minj = i;
    for(j=i;j<n;j++){
      if(A[j].num < A[minj].num){
	minj = j;
      }
    }
    k = A[i];
    A[i] = A[minj];
    A[minj] = k;
  }

  for(i=0;i<n;i++){
    if(i == n - 1){
      printf("%c%d\n",A[i].gara,A[i].num);
      break;
    }
    printf("%c%d ",A[i].gara,A[i].num);
  }
}