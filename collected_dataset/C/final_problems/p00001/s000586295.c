#include <stdio.h>
int main(void){
    // Here your code !
    int work;
    int rank[3]={0};
    int i;
    for(i=0;i<10;i++){
        scanf("%d\n",&work);
        if(work > rank[0]){
            rank[2] = rank[1];
            rank[1] = rank[0];
            rank[0] = work;
        }else if(work > rank[1]){
            rank[2] = rank[1];
            rank[1] = work;
        }else if(work > rank[2]){
            rank[2] = work;
        }
    }
    for(i=0;i<3;i++){
        printf("%d\n",rank[i]);
    }
    return 0;
}