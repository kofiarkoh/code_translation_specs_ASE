#include<stdio.h>
#define M 1000000007
int N;
int flag;
void check(int T[N], int A[N])
{
    int i,max_T,max_A;
    flag = 1;
    max_T=T[0];
    max_A=A[0];
    for(i=1;i<N;i++){
        if(T[i]<T[i-1] || A[i]>A[i-1]){
            flag = 0;
            return;
        }
        if(T[i]>max_T){
            max_T = T[i];
        }
        if(A[i]>max_A){
            max_A = A[i];
        }
    }
    if(max_T != max_A){
        flag = 0;
    }
    return;
}
void justify_T(int T[N], int jus_T[N][2])
{
    jus_T[0][0] = T[0];
    jus_T[0][1] = 0;
    int i;
    for(i=1;i<N;i++){
        if(T[i]>T[i-1]){
            jus_T[i][0] = T[i];
            jus_T[i][1] = 0;
        }else{
            jus_T[i][0] = 0;
            jus_T[i][1] = T[i];
        }
    }
    return;
}

void justify_A(int A[N], int jus_A[N][2])
{
    jus_A[N-1][0] = A[N-1];
    jus_A[N-1][1] = 0;
    int i;
    for(i=N-2;i>=0;i--){
        if(A[i]>A[i+1]){
            jus_A[i][0] = A[i];
            jus_A[i][1] = 0;
        }else{
            jus_A[i][0] = 0;
            jus_A[i][1] = A[i];
        }
    }
    return;
}

void compare(int combine[N], int jus_T[N][2], int jus_A[N][2])
{
    int i;
    flag = 1;
    for(i=0;i<N;i++){
        if(jus_A[i][0]!=0 & jus_T[i][0]!=0 & jus_A[i][0]!= jus_T[i][0]){
            flag = 0;
            return;
        }else if(jus_A[i][0] ==0 & jus_T[i][0] ==0){
            combine[i] = jus_A[i][1]<jus_T[i][1] ? jus_A[i][1] : jus_T[i][1];
        }else{
            combine[i] = 1;
        }
    }
    return;
}

int main(void)
{
    scanf("%d",&N);
    int i,j,T[N],A[N];
    for(i=0;i<N;i++){
        scanf("%d",&T[i]);
    }
    for(i=0;i<N;i++){
        scanf("%d",&A[i]);
    }

    check(T,A);
    if(flag == 0){
        printf("0");
        return 0;
    }

    int jus_T[N][2];
    int jus_A[N][2];

    justify_T(T,jus_T);
    justify_A(A,jus_A);

    /*for(i=0;i<N;i++){
        for(j=0;j<2;j++){
            printf("%d ",jus_T[i][j]);
        }
        printf("\n");
    }
    printf("\n");
    for(i=0;i<N;i++){
        for(j=0;j<2;j++){
            printf("%d ",jus_A[i][j]);
        }
        printf("\n");
    }*/

    int combine[N];
    unsigned long long  result;
    compare(combine,jus_T,jus_A);

    if(flag==0){
        printf("0");
        return 0;
    }else{
        result = 1;
        //printf("M:%d\n",M);
        for(i=0;i<N;i++){
            //printf("combine:%d\n",combine[i]);
            result = result*combine[i]%M;
            //printf("result:%d\n",result);
        }
        printf("%ld",result);
        return 0;
    }

}