#include<stdio.H>
int main()
{
    int A,B,C;
    scanf("%d %d %d",&A,&B,&C);
    A=A+B;
    if(A>=C){
        printf("Yes");
    }
    else{
        printf("No");
    }
    return 0;
}
