#include<stdio.h>
int main(void){
    int A,B,C;
    scanf("%d",&A);
    scanf("%d",&B);
    scanf("%d",&C);             

    if(A+B>=C){
        printf("Yes\n");
    }
    else{
        printf("No\n");
    }
    return 0;
}
