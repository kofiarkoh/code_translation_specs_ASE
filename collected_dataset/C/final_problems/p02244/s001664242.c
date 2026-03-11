#include<stdio.h>
#define N 8
#define FREE 0
#define NOT_FREE 1
void putQueen(int);
void initialization(void);
void print(void);

int row[N],col[N],dpos[2*N-1],deng[2*N-1];
int judge[N][N];

int main(){
    int i,j,k,r,c;
    
    initialization();

    for(i=0;i<N;i++){
        for(j=0;j<N;j++){
            judge[i][j] = 0;
        }
    }

    scanf("%d",&k);

    for(i=0;i<k;i++){
        scanf("%d%d",&r,&c);
        judge[r][c] = 1;
    }

    putQueen(0);

    return 0;
}
void initialization(){
    int i;

    for(i=0;i<N;i++){
        row[i] = col[i] = FREE;
    }
    for(i=0;i<2*N-1;i++){
        dpos[i] = deng[i] = FREE;
    }
}
void print(){
    int i,j;

    for(i=0;i<N;i++){
        for(j=0;j<N;j++){
            if(judge[i][j] == 1){
                if(row[i] != j){
                    return;
                }
            }
        }
    }
    for(i=0;i<N;i++){
        for(j=0;j<N;j++){
            if(row[i] == j){
                printf("Q");
            }
            else{
                printf(".");
            }
        }
        printf("\n");
    }
}
void putQueen(int i){
    int j;

    if(i == N){
        print();
        return;
    }
    for(j=0;j<N;j++){
        if(col[j] == NOT_FREE ||
        dpos[i+j] == NOT_FREE || deng[i-j+N-1] == NOT_FREE){
            continue;
        }
        row[i] = j;
        col[j] = dpos[i+j] = deng[i-j+N-1] = NOT_FREE;
        putQueen(i+1);
        col[j] = dpos[i+j] = deng[i-j+N-1] = FREE;
    }
}
