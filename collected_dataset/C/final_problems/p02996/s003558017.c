#include <stdio.h>

void swap (int *x, int *y) {
  int temp;    // 値を一時保存する変数

  temp = *x;
  *x = *y;
  *y = temp;
}

int main(){
    int N, AB[200000][2];
    int time=0;
    scanf("%d", &N);
    for(int i=0;i<N;i++)    scanf("%d %d", &AB[i][0], &AB[i][1]);
    
    for(int i=0;i<N-1;i++){
        for(int j=i;j<N;j++){
            if(AB[i][1] > AB[j][1]){
                swap(&AB[i][0], &AB[j][0]);
                swap(&AB[i][1], &AB[j][1]);
            }
        }
    }
    for(int i=0;i<N;i++){
        if(time+AB[i][0] > AB[i][1]){
            printf("No");
            return 0;
        }
        time += AB[i][0];
    }
    printf("Yes");
}