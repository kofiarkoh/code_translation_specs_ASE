#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

#define DEBUG 0

int* array;
int N;
int L;
int min_counter;

void Print_Array(void);
void sliding_Window(void);
int Min_in_L(int start);

int main(){

    scanf("%d %d",&N,&L);
    if(DEBUG)printf("N = %6d\nL = %6d\n",N,L);

    array = malloc(sizeof(int)*N);
    if(array == NULL){
        if(DEBUG)printf("Out of memory, MEOW~~\n");
        return 1;
    }


    for(int i = 0; i< N; i++){
        scanf("%d",&array[i]);
    }
    if(DEBUG)Print_Array();

    sliding_Window();


}
void sliding_Window(void){
    int left = 0;
    int min = Min_in_L(left);
    printf("%d",min);
    

    while(left+L < N){
        if(array[left+L] < min){ // 右邊進來的比原本的全部都還小 所以直接設定他 然後把min_counter 設定成1
            min = array[left+L];
            min_counter = 1;
            printf(" %d",min);
        }else{
            if(array[left+L] == min){
                if(array[left] != min){
                    min_counter ++;
                }
                printf(" %d",min);
            }else{// 右邊即將吃進來的比原本的最小值還大
                if(array[left]!= min){//即將要吐掉的不是最小值 ==> 直接印出來原本的就好
                    printf(" %d",min);
                }else{//即將吐掉的就是原本的最小值
                    if(min_counter > 1){// 原本的最小值不只一個
                        min_counter --;
                        printf(" %d",min);
                    }else{//原本的最小值就是即將要被吐掉的 且還剛好只有他一個
                        min = Min_in_L(left+1); // 只好重跑
                        printf(" %d",min);
                    }
                }
            }
        }
        left++;
    }
    printf("\n");
}

int Min_in_L(int start){
    int i;
    int min = INT_MAX;
    min_counter = 0;
    for(i = 0; i< L; i++){
        if(array[start+i] < min){
            min = array[start+i];
            min_counter = 1;
        }else if(array[start+i] == min){
            min_counter ++;
        }
    }
    return min;
}


void Print_Array(void){
    int i;
    printf("array  : ");
    for (i = 0; i< N ; i++){
        printf("%3d",array[i]);
    }
    printf("\n");
}

