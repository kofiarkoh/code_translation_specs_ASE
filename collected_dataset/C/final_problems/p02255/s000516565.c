#include<stdio.h>

void aiueo(int *,int);
void insertionSort(int *,int);




int main(){
  int N,i,j;
  int A[100];

  scanf("%d",&N);

  for(i=0;i <= N-1;i++){
    scanf("%d",&A[i]);

  }aiueo(A,N);

    insertionSort(A,N);
    return 0;
  }

  void aiueo(int *A,int N){
    int i;
    for(i=0;i < N;i++){
      if(i > 0)printf(" ");
      printf("%d", A[i]);
    }
    printf("\n");
  }
  void insertionSort(int *A,int N){
    int v,i,j;

    for(i=1;i <= N-1;i++){
      v=A[i];
      j=i-1;
      while(j >= 0 && A[j] > v){
	A[j+1]=A[j];
	j--;

      }
      A[j+1] = v;
      aiueo(A,N);
    }
  }
      
		     
	
		       

