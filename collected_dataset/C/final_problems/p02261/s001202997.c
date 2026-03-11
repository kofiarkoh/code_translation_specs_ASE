#include<stdio.h>
#include<stdlib.h>

typedef struct {
  char mark;
  int value;
} data;

void BubbleSort(data*, int);
void SelectionSort(data*, int);
int StableJudge(data*, data*, int*, int);
/*
  StableJudge
    return  0 -> Not Stable
            1 -> Stable
*/

void BubbleSort(data *C, int N) {
  int i,j;
  data temp;

  for(i=0;i<N;i++){
    for(j=N-1;j>i;j--){
      if(C[j].value < C[j-1].value){
        temp = C[j];
        C[j] = C[j-1];
        C[j-1] = temp;
      }
    }
  }
}

void SelectionSort(data *C, int N){
  int i, j, minj;
  data temp;

  for(i=0;i<N;i++){
    minj = i;
    for(j=i;j<N;j++){
      if(C[j].value < C[minj].value){
        minj = j;
      }
    }
    if(C[i].value != C[minj].value){
      temp = C[i];
      C[i] = C[minj];
      C[minj] = temp;
    }
  }
}

int StableJudge(data *C, data *C_sort, int *list, int N){
  int i, j, n;
  int *mark_C, *mark_C_sort;
  int flag = 0; //is it necessary to free()?

  for(i=0;i<9;i++){
    //printf("Loop %d\n",i);
    // If there is the same number
    if(list[i] > 1){
      flag = 1;

      mark_C = (int *)malloc(sizeof(int) * list[i]);
      n = 0;
      for(j=0;j<N;j++){
        if(C[j].value == i+1){
          mark_C[n] = j;
          n++;
        }
        if(n == list[i]) break;
      }
      //for(j=0;j<n;j++) printf("mark_C[%d] = %d\n",j,mark_C[j]);

      mark_C_sort = (int *)malloc(sizeof(int) * list[i]);
      n = 0;
      for(j=0;j<N;j++){
        if(C_sort[j].value == i+1){
          mark_C_sort[n] = j;
          n++;
        }
        if(n == list[i]) break;
      }
      //for(j=0;j<n;j++) printf("mark_C_sort[%d] = %d\n",j,mark_C_sort[j]);

      for(j=0;j<n;j++){
        //printf("%c  %c\n",C[mark_C[j]].mark,C_sort[mark_C_sort[j]].mark);
        if(C[mark_C[j]].mark != C_sort[mark_C_sort[j]].mark) {
          return 0;
        }
      }
    }
  }
  if(flag){
    free(mark_C);
    free(mark_C_sort);
  }
  return 1;
}

int main(){
  int N, i;
  char inp[3];
  data *C, *C_bub, *C_sel;
  // to judge same number
  int list[9] = {0,0,0,0,0,0,0,0,0};

  // data set
  scanf("%d",&N);

  C = (data *)malloc(sizeof(data) * N);
  C_bub = (data *)malloc(sizeof(data) * N);
  C_sel = (data *)malloc(sizeof(data) * N);

  for(i=0;i<N;i++){
    scanf("%s",inp);
    C[i].mark = C_bub[i].mark = C_sel[i].mark = inp[0];
    C[i].value = C_bub[i].value = C_sel[i].value = atoi(&inp[1]);
  }

  // get same number list
  for(i=0;i<N;i++){
    list[C[i].value-1]++;
  }
  //for(i=0;i<9;i++)  printf("list[%d] = %d\n",i,list[i]);


  // sort
  BubbleSort(C_bub, N);
  SelectionSort(C_sel, N);

  // print
  for(i=0;i<N;i++){
      printf("%c%d",C_bub[i].mark ,C_bub[i].value);
      if(i != N-1) printf(" ");
      else printf("\n");
  }
  if(StableJudge(C, C_bub, list, N)){
    printf("Stable\n");
  } else {
    printf("Not stable\n");
  }

  for(i=0;i<N;i++){
      printf("%c%d",C_sel[i].mark ,C_sel[i].value);
      if(i != N-1) printf(" ");
      else printf("\n");
  }
  if(StableJudge(C, C_sel, list, N)){
    printf("Stable\n");
  } else {
    printf("Not stable\n");
  }

  return 0;
}

