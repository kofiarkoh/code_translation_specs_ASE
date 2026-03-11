#include<stdio.h>
int main()
{
    int A;
    int B;
    int T;
    int F;


    scanf("%d %d %d", &A, &B, &T);
    if(A < 1 || A > 20){
            scanf("%d %d %d", &A, &B, &T);
        }
    if(B < 1 || B > 20){
            scanf("%d %d %d", &A, &B, &T);
        }
    if(T < 1 || T > 20){
            scanf("%d %d %d", &A, &B, &T);
        }
    F = (T+0.5) / A;
    if(A > 2*((T+0.5)/3) ){
        printf("0 \n");
        }else{
        printf("%d",F * B);
        }
    }