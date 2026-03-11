#include<stdio.h>
#include<stdlib.h>

int compare_int_big(const void *a,const void *b){
    return *(int *)b - *(int *)a;
}

int compare_int_small(const void *a,const void *b){
    return *(int *)a - *(int *)b;
}

int main(void)
{
    int ans = 0;
    int kouho = 0;
    int saishouKBS = 0;
    int flag = 0;

    int N,M;
    int A[100010];

    scanf("%d",&N);
    scanf("%d",&M);

    for (int i = 0; i < N; i++)
    {
        scanf("%d",&A[i]);
        if(A[i]%4==0){
            printf("0");
            return 0;
        }
    }
    
    qsort(A,N,sizeof(int),compare_int_big);

    for(int i = 0; kouho <= M ;i++){
        kouho = A[0]*i+A[0]/2;
        if(kouho > M) break;
        flag = 1;
        for(int j = 1; j < N ; j++){
            if(kouho%A[j] == A[j]/2){

            }else{
                flag = 0;
            }
        }
        if(flag == 1){
            ans++;
            saishouKBS = kouho;
            break;
        }
    }

    ans = M/saishouKBS;
    ans /= 2;
    ans ++;

    printf("%d",ans);

    return 0;
}